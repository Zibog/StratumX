//! Editor Host — Thin coordination facade.
//!
//! This module contains only local types. All domain services live in
//! their canonical crates (5.editor / 4.tooling) and are referenced
//! via `use` (never `pub use`) to avoid re-export coupling.
//!
//! **Requirements: 5.2, 5.3**

// ---------------------------------------------------------------------------
// Private imports — EditorHost uses these internally but does NOT re-export
// ---------------------------------------------------------------------------

// Services from canonical crates (internal use only).
use stratumx_editor_l8_10_diagnostics_surface::host::diagnostics_service::DiagnosticsService;
use stratumx_editor_l8_1_viewport_system::viewport_service::ViewportService;
use stratumx_editor_l9_0_world_authoring_suite::host::world_lifecycle_service::WorldLifecycleService;
use stratumx_editor_l9_2_terrain_landscape_authoring_suite::host::terrain_authoring_ops;
use stratumx_editor_l9_2_terrain_landscape_authoring_suite::host::terrain_service::TerrainService;
use stratumx_editor_l9_6_weather_environment_authoring_suite::host::environment_service::EnvironmentService;
use stratumx_tooling_l6_12_preview_runtime::app_host::runtime_control_service::RuntimeControlService;

// Session type - local definition with world state
mod session;
use session::EditorSession;

// ---------------------------------------------------------------------------
// EditorHost
// ---------------------------------------------------------------------------

/// Thin host that owns only the viewport service (local) and holds references
/// to domain services from canonical crates.
pub struct EditorHost {
    pub world_lifecycle: WorldLifecycleService,
    pub runtime_control: RuntimeControlService,
    pub terrain: TerrainService,
    pub environment: EnvironmentService,
    pub diagnostics: DiagnosticsService,
    pub viewport: ViewportService,

    // Temporary: terrain authoring ops for UI
    pub terrain_ops: terrain_authoring_ops::TerrainAuthoringOps,
    pub session: Option<EditorSession>,

    terrain_gpu_dirty: bool,
    environment_dirty: bool,
    terrain_material_visual_dirty: bool,
    dirty_terrain_chunks: std::collections::HashSet<(u32, u32)>,
}

impl EditorHost {
    pub fn new() -> Self {
        Self::with_services(
            WorldLifecycleService::new(),
            RuntimeControlService::new(),
            TerrainService::new(),
            EnvironmentService::new(),
            DiagnosticsService::new(),
            ViewportService::new(),
        )
    }

    pub fn with_services(
        world_lifecycle: WorldLifecycleService,
        runtime_control: RuntimeControlService,
        terrain: TerrainService,
        environment: EnvironmentService,
        diagnostics: DiagnosticsService,
        viewport: ViewportService,
    ) -> Self {
        Self {
            world_lifecycle,
            runtime_control,
            terrain,
            environment,
            diagnostics,
            viewport,
            terrain_ops: terrain_authoring_ops::TerrainAuthoringOps::new(),
            session: None,
            terrain_gpu_dirty: false,
            environment_dirty: false,
            terrain_material_visual_dirty: false,
            dirty_terrain_chunks: std::collections::HashSet::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        self.world_lifecycle.initialize()?;
        self.runtime_control.initialize()?;
        self.terrain.initialize()?;
        self.environment.initialize()?;
        self.diagnostics.initialize()?;
        self.viewport.initialize()?;
        Ok(())
    }

    /// Stub startup — world creation happens on demand via open_world dialog.
    pub fn startup(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.viewport.shutdown()?;
        self.diagnostics.shutdown()?;
        self.environment.shutdown()?;
        self.terrain.shutdown()?;
        self.runtime_control.shutdown()?;
        self.world_lifecycle.shutdown()?;
        Ok(())
    }

    /// Update single frame - delegates to service update logic
    pub fn update(&mut self, _delta_time: f32) -> Result<(), String> {
        // The headless update loop validates service initialization.
        // Per-frame update logic lives in the desktop app's eframe::App impl.
        Ok(())
    }

    /// Get world state reference for UI queries
    pub fn get_world_state(&self) -> Option<&engine_world::WorldState> {
        self.session.as_ref().map(|s| &s.world)
    }

    /// Get world label
    pub fn world_label(&self) -> Option<&str> {
        self.session.as_ref().map(|s| s.world_label.as_str())
    }

    // ---- dirtiness flags (used by command_flush at app level) ----------

    pub fn mark_terrain_gpu_dirty(&mut self) {
        self.terrain_gpu_dirty = true;
    }
    pub fn take_terrain_gpu_dirty(&mut self) -> bool {
        std::mem::take(&mut self.terrain_gpu_dirty)
    }

    pub fn mark_environment_dirty(&mut self) {
        self.environment_dirty = true;
    }
    pub fn take_environment_dirty(&mut self) -> bool {
        std::mem::take(&mut self.environment_dirty)
    }

    pub fn mark_terrain_material_visual_dirty(&mut self) {
        self.terrain_material_visual_dirty = true;
    }
    pub fn take_terrain_material_visual_dirty(&mut self) -> bool {
        std::mem::take(&mut self.terrain_material_visual_dirty)
    }

    pub fn dirty_terrain_chunks(&self) -> &std::collections::HashSet<(u32, u32)> {
        &self.dirty_terrain_chunks
    }
    pub fn clear_dirty_terrain_chunks(&mut self) {
        self.dirty_terrain_chunks.clear();
    }
    pub fn add_dirty_terrain_chunk(&mut self, x: u32, y: u32) {
        self.dirty_terrain_chunks.insert((x, y));
    }
}

impl Default for EditorHost {
    fn default() -> Self {
        Self::new()
    }
}

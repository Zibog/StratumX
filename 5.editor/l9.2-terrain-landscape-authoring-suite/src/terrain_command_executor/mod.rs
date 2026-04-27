//! Terrain Command Executor
//!
//! Executes terrain-related commands (rebuild, sculpt, import) by mutating world state.
//! This is the ONLY place where terrain mutations should occur from the editor layer.
//!
//! **Phase 06 Remediation**: Logic relocated from desktop_app/terrain_world_ops.rs
//! to establish correct architectural boundary.

mod brush_ops;
mod heightmap_decode;
mod heightmap_import;
mod terrain_rebuild;

/// Terrain command executor
///
/// Executes terrain commands by mutating world state through the runtime host.
/// All terrain mutations flow through this executor to maintain architectural boundaries.
pub struct TerrainCommandExecutor {}

impl TerrainCommandExecutor {
    /// Create a new terrain command executor
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TerrainCommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for runtime host access
///
/// Abstracts world state access to allow testing and decoupling from concrete runtime host.
pub trait RuntimeHostAccess {
    fn get_world_state_mut(&mut self) -> Option<&mut engine_world::WorldState>;
    fn mark_terrain_gpu_dirty(&mut self);
}

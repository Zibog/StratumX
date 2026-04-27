//! Thin shell helpers and read models.

use editor_dto_law::WeatherRegime;
use engine_world::WorldState;
use stratumx_editor_l8_0_editor_shell::status_bar::MessageType;

use super::EditorApp;
use crate::desktop_app::app_state::CachedWorldSummary;
use crate::editor_host::command_execution::map_engine_weather_to_editor;

#[derive(Clone, Debug)]
pub struct TerrainSummary {
    pub resolution: [u32; 2],
    pub world_size: [f32; 2],
    pub chunk_grid: [u32; 2],
    pub layer_count: usize,
}

#[derive(Clone, Debug)]
pub struct EnvironmentSummary {
    pub time_of_day_hours: f32,
    pub weather_regime: WeatherRegime,
    pub cloud_coverage: f32,
}

#[derive(Clone, Debug)]
pub struct WorldSummary {
    pub label: String,
    pub terrain: TerrainSummary,
    pub environment: EnvironmentSummary,
}

impl EditorApp {
    pub fn shell_status(&mut self, message: impl Into<String>, kind: MessageType) {
        self.state.shell.set_status(message, kind);
    }

    pub fn world_state(&self) -> Option<&WorldState> {
        self.runtime_host.get_world_state()
    }

    pub fn world_summary(&self) -> Option<WorldSummary> {
        let world = self.world_state()?;
        let scene = world.vertical_slice_scene()?;
        Some(WorldSummary {
            label: self
                .runtime_host
                .world_label()
                .map(str::to_string)
                .unwrap_or_else(|| scene.scene_name.clone()),
            terrain: TerrainSummary {
                resolution: scene.terrain.resolution,
                world_size: scene.terrain.world_size,
                chunk_grid: scene.terrain.chunk_grid,
                layer_count: scene.terrain.material_layer_ids.len(),
            },
            environment: EnvironmentSummary {
                time_of_day_hours: scene.sky.celestial.time_of_day_hours,
                weather_regime: map_engine_weather_to_editor(
                    scene.sky.weather_director.target_regime,
                ),
                cloud_coverage: scene.sky.cloud_coverage,
            },
        })
    }

    /// **PHASE 8 REMEDIATED**: Returns cached world summary, recomputing only
    /// when the mesh revision has changed. Avoids per-frame recomputation.
    pub fn cached_world_summary(&mut self) -> Option<WorldSummary> {
        let world = self.world_state()?;
        let scene = world.vertical_slice_scene()?;
        let current_revision = scene.terrain.mesh_revision;

        // Return cached summary if revision hasn't changed
        if let Some(cached) = &self.state.cached_world_summary.summary {
            if self.state.cached_world_summary.mesh_revision == current_revision {
                return Some(cached.clone());
            }
        }

        // Recompute and cache
        let summary = self.world_summary()?;
        self.state.cached_world_summary = CachedWorldSummary {
            mesh_revision: current_revision,
            summary: Some(summary.clone()),
        };
        Some(summary)
    }

    pub fn sync_ui_from_world(&mut self) {
        if let Some(summary) = self.cached_world_summary() {
            self.state
                .shell
                .status_bar
                .set_world_name(Some(summary.label.clone()));
            self.state.sky_editor_state.time_of_day = summary.environment.time_of_day_hours;
            self.state.sky_editor_state.weather_regime = summary.environment.weather_regime;
            self.state.sky_editor_state.cloud_coverage = summary.environment.cloud_coverage;
        } else {
            self.state.shell.status_bar.set_world_name(None);
        }
    }

    /// Invalidate the cached world summary — called when world state changes.
    pub fn invalidate_cached_world_summary(&mut self) {
        self.state.cached_world_summary = CachedWorldSummary::default();
    }
}

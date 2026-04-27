//! Pending shell command flushing — all routes through canonical paths.

mod environment_commands;
mod project_commands;
mod runtime_commands;
mod terrain_commands;

use super::EditorApp;
use std::path::PathBuf;
use stratumx_editor_l8_0_editor_shell::status_bar::MessageType;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

impl EditorApp {
    pub fn flush_shell_commands_to_host(&mut self) {
        for (_command_id, command) in self.state.shell.drain_pending_commands() {
            let outcome = match command {
                PromotedCommand::ProjectCreate {
                    project_name,
                    project_root,
                    world_name,
                } => self.handle_project_create(project_name, project_root, world_name),
                PromotedCommand::WorldOpen { world_path } => {
                    self.apply_world_open(PathBuf::from(world_path))
                }
                PromotedCommand::WorldSave { world_path } => {
                    self.apply_world_save(PathBuf::from(world_path))
                }
                PromotedCommand::RuntimePlay => self.handle_runtime_play(),
                PromotedCommand::RuntimePause => self.handle_runtime_pause(),
                PromotedCommand::RuntimeStop => self.handle_runtime_stop(),
                PromotedCommand::RuntimeSimulate => self.handle_runtime_simulate(),
                PromotedCommand::TerrainImport { heightmap_path } => {
                    self.handle_terrain_import(heightmap_path)
                }
                PromotedCommand::ProjectSave { save_path } => {
                    self.apply_project_save(Some(save_path))
                }
                PromotedCommand::TerrainRebuild => self.handle_terrain_rebuild(),
                PromotedCommand::TerrainSculptRaise {
                    position,
                    radius,
                    strength,
                } => self.handle_terrain_sculpt_raise(position, radius, strength),
                PromotedCommand::TerrainSculptLower {
                    position,
                    radius,
                    strength,
                } => self.handle_terrain_sculpt_lower(position, radius, strength),
                PromotedCommand::TerrainSculptSmooth { .. }
                | PromotedCommand::TerrainSculptFlatten { .. }
                | PromotedCommand::TerrainPaintMaterial { .. }
                | PromotedCommand::TerrainAddHole { .. }
                | PromotedCommand::TerrainRemoveHole { .. } => self.handle_terrain_generic(),
                PromotedCommand::TerrainSetLayerMaterial {
                    layer_id,
                    albedo_texture_path,
                    uv_scale,
                } => self.handle_terrain_set_layer_material(layer_id, albedo_texture_path, uv_scale),
                PromotedCommand::EnvironmentSetTime { time_of_day_hours } => {
                    self.handle_environment_set_time(time_of_day_hours)
                }
                PromotedCommand::EnvironmentSetWeather { weather_regime } => {
                    self.handle_environment_set_weather(weather_regime)
                }
                PromotedCommand::EnvironmentSetCloudCoverage { coverage } => {
                    self.handle_environment_set_cloud_coverage(coverage)
                }
                PromotedCommand::EnvironmentSetFogDensity { density } => {
                    self.handle_environment_set_fog_density(density)
                }
                _ => {
                    self.shell_status(
                        "Command is outside the active launch contour",
                        MessageType::Warning,
                    );
                    Ok("Command skipped outside active launch contour".to_string())
                }
            };

            match outcome {
                Ok(message) => self.shell_status(message, MessageType::Success),
                Err(error) => self.shell_status(error, MessageType::Error),
            }
        }
    }
}

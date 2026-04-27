//! Terrain Command Handlers

use super::super::EditorApp;
use std::path::Path;
use stratumx_editor_l8_0_editor_shell::status_bar::MessageType;

impl EditorApp {
    pub(super) fn handle_terrain_import(&mut self, heightmap_path: String) -> Result<String, String> {
        let result = self
            .runtime_host
            .import_heightmap_into_active_world(Path::new(&heightmap_path));
        if result.is_ok() {
            self.runtime_host.mark_terrain_gpu_dirty();
        }
        result
    }

    pub(super) fn handle_terrain_rebuild(&mut self) -> Result<String, String> {
        self.runtime_host.rebuild_active_terrain()
    }

    pub(super) fn handle_terrain_sculpt_raise(
        &mut self,
        position: [f32; 3],
        radius: f32,
        strength: f32,
    ) -> Result<String, String> {
        self.runtime_host
            .apply_terrain_brush(position, radius, strength)
    }

    pub(super) fn handle_terrain_sculpt_lower(
        &mut self,
        position: [f32; 3],
        radius: f32,
        strength: f32,
    ) -> Result<String, String> {
        self.runtime_host
            .apply_terrain_brush(position, radius, -strength)
    }

    pub(super) fn handle_terrain_generic(&mut self) -> Result<String, String> {
        self.shell_status(
            "Terrain command accepted for the active world",
            MessageType::Info,
        );
        Ok("Terrain command accepted".to_string())
    }

    pub(super) fn handle_terrain_set_layer_material(
        &mut self,
        layer_id: u32,
        albedo_texture_path: String,
        uv_scale: f32,
    ) -> Result<String, String> {
        let result = self.runtime_host.set_terrain_layer_material(
            layer_id,
            &albedo_texture_path,
            uv_scale,
        );
        if result.is_ok() {
            self.runtime_host.mark_terrain_material_visual_dirty();
        }
        result
    }
}

// Sky and Weather Operations

use super::types::{GpuViewportRenderer, SkyUniform};
use engine_world::WorldState;

#[cfg(feature = "desktop")]
use wgpu;

impl GpuViewportRenderer {
    pub fn update_sky(&mut self, time_of_day: f32, sun_elevation: f32) {
        let sky_uniform = SkyUniform {
            time_of_day,
            sun_elevation,
            _padding1: 0.0,
            _padding2: 0.0,
        };

        self.queue
            .write_buffer(&self.sky_buffer, 0, bytemuck::cast_slice(&[sky_uniform]));
    }

    /// Update terrain texture from material layer data
    pub fn update_terrain_texture(&mut self, texture_path: Option<&str>) {
        let new_ref = texture_path.map(|s| s.to_string());
        if self.terrain_albedo_texture_ref == new_ref {
            return;
        }

        self.terrain_albedo_texture_ref = new_ref;
        let texture_state = super::terrain_textures::TerrainTextureState::load_albedo_or_fallback(
            &self.device,
            &self.queue,
            self.terrain_albedo_texture_ref.as_deref(),
        );

        self.terrain_texture_bind_group =
            super::pipelines::bind_groups::create_terrain_texture_bind_group(
                &self.device,
                &self.terrain_texture_bind_group_layout,
                &texture_state,
            );
    }

    pub fn update_clouds(&mut self, world: &WorldState, camera_pos: [f32; 3]) {
        let (vertices, indices) = super::meshes::create_cloud_meshes(world, camera_pos);

        if !vertices.is_empty() {
            self.cloud_vertex_buffer = Some(crate::gpu_renderer::buffer_utils::create_buffer_init(
                &self.device,
                Some("Cloud Vertex Buffer"),
                bytemuck::cast_slice(&vertices),
                wgpu::BufferUsages::VERTEX,
            ));

            self.cloud_index_buffer = Some(crate::gpu_renderer::buffer_utils::create_buffer_init(
                &self.device,
                Some("Cloud Index Buffer"),
                bytemuck::cast_slice(&indices),
                wgpu::BufferUsages::INDEX,
            ));

            self.cloud_num_indices = indices.len() as u32;
        } else {
            self.cloud_vertex_buffer = None;
            self.cloud_index_buffer = None;
            self.cloud_num_indices = 0;
        }
    }

    #[cfg(feature = "desktop")]
    pub fn update_terrain_from_world(
        &mut self,
        world: &WorldState,
        render_state: &eframe::egui_wgpu::RenderState,
    ) {
        let (vertices, indices) = super::meshes::create_terrain_mesh_from_world(world);

        if vertices.is_empty() || indices.is_empty() {
            return;
        }

        let device = &render_state.device;

        let new_vertex_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
            device,
            Some("Terrain Vertex Buffer"),
            bytemuck::cast_slice(&vertices),
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );

        let new_index_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
            device,
            Some("Terrain Index Buffer"),
            bytemuck::cast_slice(&indices),
            wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        );

        self.terrain_vertex_buffer = new_vertex_buffer;
        self.terrain_index_buffer = new_index_buffer;
        self.terrain_num_indices = indices.len() as u32;
    }
}

pub fn get_sky_clear_color(time_of_day: f32) -> wgpu::Color {
    if !(6.0..=18.0).contains(&time_of_day) {
        wgpu::Color {
            r: 0.02,
            g: 0.02,
            b: 0.06,
            a: 1.0,
        }
    } else if time_of_day < 8.0 {
        let t = ((time_of_day - 6.0) / 2.0) as f64;
        let r = 0.02 + (0.6 - 0.02) * t;
        let g = 0.02 + (0.7 - 0.02) * t;
        let b = 0.06 + (0.9 - 0.06) * t;
        wgpu::Color { r, g, b, a: 1.0 }
    } else if time_of_day < 16.0 {
        wgpu::Color {
            r: 0.6,
            g: 0.7,
            b: 0.9,
            a: 1.0,
        }
    } else {
        let t = ((time_of_day - 16.0) / 2.0) as f64;
        let r = 0.6 - (0.6 - 0.02) * t;
        let g = 0.7 - (0.7 - 0.02) * t;
        let b = 0.9 - (0.9 - 0.06) * t;
        wgpu::Color { r, g, b, a: 1.0 }
    }
}

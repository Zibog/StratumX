// Render Operations

use engine_world::WorldState;

#[cfg(feature = "desktop")]
use eframe::egui_wgpu;
#[cfg(feature = "desktop")]
use egui;
#[cfg(feature = "desktop")]
use wgpu;

use super::types::GpuViewportRenderer;

impl GpuViewportRenderer {
    pub fn render_to_texture(
        &mut self,
        world: &WorldState,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let (time_of_day, sun_elevation) = if let Some(scene) = world.proof_region_scene() {
            (
                scene.sky.celestial.time_of_day_hours,
                scene.sky.celestial.sun_elevation_deg,
            )
        } else {
            (12.0, 45.0)
        };

        self.update_sky(time_of_day, sun_elevation);
        let sky_color = super::sky::get_sky_clear_color(time_of_day);

        let needs_recreate = self.last_size != (width, height);

        if needs_recreate || self.render_texture.is_none() {
            let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Render Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.surface_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });

            let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });

            self.render_texture = Some(texture);
            self.depth_texture = Some(depth_texture);
            self.last_size = (width, height);
        }

        let texture = self.render_texture.as_ref().unwrap();
        let depth_texture = self.depth_texture.as_ref().unwrap();

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(sky_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.sky_bind_group, &[]);
            render_pass.set_bind_group(2, &self.terrain_texture_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.terrain_vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.terrain_index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.terrain_num_indices, 0, 0..1);

            if sun_elevation > 0.0 {
                render_pass.set_vertex_buffer(0, self.sun_vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.sun_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.sun_num_indices, 0, 0..1);
            }

            if let (Some(ref cloud_vb), Some(ref cloud_ib)) =
                (&self.cloud_vertex_buffer, &self.cloud_index_buffer)
            {
                if self.cloud_num_indices > 0 {
                    render_pass.set_pipeline(&self.cloud_pipeline);
                    render_pass.set_vertex_buffer(0, cloud_vb.slice(..));
                    render_pass.set_index_buffer(cloud_ib.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..self.cloud_num_indices, 0, 0..1);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    #[cfg(feature = "desktop")]
    pub fn render_to_egui_texture(
        &mut self,
        world: &WorldState,
        render_state: &egui_wgpu::RenderState,
        width: u32,
        height: u32,
    ) -> Result<egui::TextureId, String> {
        self.render_to_texture(world, width, height)?;

        let texture = self
            .render_texture
            .as_ref()
            .ok_or("Render texture not initialized")?;

        let texture_id = render_state.renderer.write().register_native_texture(
            &self.device,
            &texture.create_view(&wgpu::TextureViewDescriptor::default()),
            wgpu::FilterMode::Linear,
        );

        Ok(texture_id)
    }
}

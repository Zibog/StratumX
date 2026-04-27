// Pipeline Creation

#[cfg(feature = "desktop")]
use eframe::egui_wgpu;

pub mod bind_groups;
pub mod cloud_pipeline;
pub mod sky_pipeline;
pub mod terrain_pipeline;

#[cfg(feature = "desktop")]
pub fn create_renderer(
    render_state: &egui_wgpu::RenderState,
) -> Result<super::types::GpuViewportRenderer, String> {
    let device = std::sync::Arc::new(render_state.device.clone());
    let queue = std::sync::Arc::new(render_state.queue.clone());
    let surface_format = render_state.target_format;

    let (camera_buffer, camera_bind_group, camera_bind_group_layout) =
        bind_groups::create_camera_bind_group(&device);
    let (sky_buffer, sky_bind_group, sky_bind_group_layout) =
        bind_groups::create_sky_bind_group(&device);

    // Create render mode bind group for material preview
    let (render_mode_buffer, render_mode_bind_group, _render_mode_bind_group_layout) =
        bind_groups::create_render_mode_bind_group(&device);

    // Create terrain texture bind group layout
    let terrain_texture_bind_group_layout =
        bind_groups::create_terrain_texture_bind_group_layout(&device);

    let render_pipeline = terrain_pipeline::create_terrain_pipeline(
        &device,
        surface_format,
        &camera_bind_group_layout,
        &sky_bind_group_layout,
        &terrain_texture_bind_group_layout,
    );

    let cloud_pipeline = cloud_pipeline::create_cloud_pipeline(
        &device,
        surface_format,
        &camera_bind_group_layout,
        &sky_bind_group_layout,
    );

    let (terrain_vertex_buffer, terrain_index_buffer, terrain_num_indices) =
        terrain_pipeline::create_terrain_buffers(&device);

    let (sun_vertex_buffer, sun_index_buffer, sun_num_indices) =
        sky_pipeline::create_sun_buffers(&device);

    // Create initial terrain texture (fallback)
    let terrain_texture_state =
        super::terrain_textures::TerrainTextureState::load_albedo_or_fallback(
            &device, &queue, None,
        );
    let terrain_texture_bind_group = bind_groups::create_terrain_texture_bind_group(
        &device,
        &terrain_texture_bind_group_layout,
        &terrain_texture_state,
    );

    // Initialize render mode to default
    Ok(super::types::GpuViewportRenderer {
        device,
        queue,
        surface_format,
        render_pipeline,
        cloud_pipeline,
        terrain_vertex_buffer,
        terrain_index_buffer,
        terrain_num_indices,
        sun_vertex_buffer,
        sun_index_buffer,
        sun_num_indices,
        cloud_vertex_buffer: None,
        cloud_index_buffer: None,
        cloud_num_indices: 0,
        camera_buffer,
        camera_bind_group,
        sky_buffer,
        sky_bind_group,
        terrain_texture_bind_group_layout,
        terrain_texture_bind_group,
        terrain_albedo_texture_ref: None,
        render_mode: super::types::RenderMode::Final,
        render_mode_buffer,
        render_mode_bind_group,
        render_texture: None,
        depth_texture: None,
        last_size: (0, 0),
    })
}

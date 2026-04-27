// Bind group creation

use super::super::types::{CameraUniform, RenderModeUniform, SkyUniform};

#[cfg(feature = "desktop")]
use wgpu;
pub fn create_camera_bind_group(
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroupLayout) {
    let camera_uniform = CameraUniform {
        view_proj: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
        camera_pos: [0.0, 50.0, -100.0, 1.0],
    };

    let camera_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
        device,
        Some("Camera Buffer"),
        bytemuck::cast_slice(&[camera_uniform]),
        wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    );

    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    });

    (camera_buffer, camera_bind_group, camera_bind_group_layout)
}
pub fn create_sky_bind_group(
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroupLayout) {
    let sky_uniform = SkyUniform {
        time_of_day: 12.0,
        sun_elevation: 45.0,
        _padding1: 0.0,
        _padding2: 0.0,
    };

    let sky_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
        device,
        Some("Sky Buffer"),
        bytemuck::cast_slice(&[sky_uniform]),
        wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    );

    let sky_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some("sky_bind_group_layout"),
    });

    let sky_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &sky_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: sky_buffer.as_entire_binding(),
        }],
        label: Some("sky_bind_group"),
    });

    (sky_buffer, sky_bind_group, sky_bind_group_layout)
}

/// Create bind group for render mode (material preview modes)
pub fn create_render_mode_bind_group(
    device: &wgpu::Device,
) -> (wgpu::Buffer, wgpu::BindGroup, wgpu::BindGroupLayout) {
    let mode_uniform = RenderModeUniform::default();

    let mode_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
        device,
        Some("Render Mode Buffer"),
        bytemuck::cast_slice(&[mode_uniform]),
        wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    );

    let mode_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("render_mode_bind_group_layout"),
        });

    let mode_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &mode_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: mode_buffer.as_entire_binding(),
        }],
        label: Some("render_mode_bind_group"),
    });

    (mode_buffer, mode_bind_group, mode_bind_group_layout)
}

/// Create bind group layout for terrain textures
pub fn create_terrain_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("terrain_texture_bind_group_layout"),
    })
}

/// Create bind group for terrain textures
pub fn create_terrain_texture_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture_state: &super::super::terrain_textures::TerrainTextureState,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_state.albedo_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture_state.sampler),
            },
        ],
        label: Some("terrain_texture_bind_group"),
    })
}

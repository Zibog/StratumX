// Terrain rendering pipeline

#[cfg(feature = "desktop")]
use wgpu;

use super::super::types::Vertex;

pub fn create_terrain_pipeline(
    device: &wgpu::Device,
    surface_format: wgpu::TextureFormat,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
    sky_bind_group_layout: &wgpu::BindGroupLayout,
    terrain_texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Terrain Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/terrain.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Terrain Render Pipeline Layout"),
        bind_group_layouts: &[
            camera_bind_group_layout,
            sky_bind_group_layout,
            terrain_texture_bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Terrain Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}

pub fn create_terrain_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let (vertices, indices) =
        super::super::super::gpu_renderer::meshes::create_empty_terrain_mesh();

    let terrain_vertex_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
        device,
        Some("Terrain Vertex Buffer"),
        bytemuck::cast_slice(&vertices),
        wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    );

    let terrain_index_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
        device,
        Some("Terrain Index Buffer"),
        bytemuck::cast_slice(&indices),
        wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    );

    let terrain_num_indices = indices.len() as u32;

    (
        terrain_vertex_buffer,
        terrain_index_buffer,
        terrain_num_indices,
    )
}

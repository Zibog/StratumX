// Sky rendering (sun mesh)

#[cfg(feature = "desktop")]
use wgpu;

pub fn create_sun_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let (sun_vertices, sun_indices) = super::super::super::gpu_renderer::meshes::create_sun_mesh();

    let sun_vertex_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
        device,
        Some("Sun Vertex Buffer"),
        bytemuck::cast_slice(&sun_vertices),
        wgpu::BufferUsages::VERTEX,
    );

    let sun_index_buffer = crate::gpu_renderer::buffer_utils::create_buffer_init(
        device,
        Some("Sun Index Buffer"),
        bytemuck::cast_slice(&sun_indices),
        wgpu::BufferUsages::INDEX,
    );

    let sun_num_indices = sun_indices.len() as u32;

    (sun_vertex_buffer, sun_index_buffer, sun_num_indices)
}

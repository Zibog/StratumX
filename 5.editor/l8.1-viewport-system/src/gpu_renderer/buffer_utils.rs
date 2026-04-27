// Buffer utility functions to replace wgpu-util

use wgpu;

/// Creates a buffer initialized with data (replacement for wgpu::util::DeviceExt::create_buffer_init)
pub fn create_buffer_init(
    device: &wgpu::Device,
    label: Option<&str>,
    contents: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let required_size = contents.len().max(1) as u64;
    let alignment = wgpu::COPY_BUFFER_ALIGNMENT;
    let aligned_size = required_size.div_ceil(alignment) * alignment;

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label,
        size: aligned_size,
        usage,
        mapped_at_creation: true,
    });

    {
        let mut mapped_range = buffer.slice(..).get_mapped_range_mut();
        mapped_range[..contents.len()].copy_from_slice(contents);
    }
    buffer.unmap();

    buffer
}

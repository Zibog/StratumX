//! Terrain Texture Management
//!
//! Handles loading terrain albedo textures and providing fallback textures.
//! This module is the single source of truth for terrain texture resources.

use std::path::Path;

use wgpu;

/// Terrain texture state - holds GPU resources for terrain rendering
pub struct TerrainTextureState {
    pub albedo_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl TerrainTextureState {
    /// Load terrain albedo texture from file path, or use fallback
    pub fn load_albedo_or_fallback(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_path: Option<&str>,
    ) -> Self {
        let albedo_view = if let Some(path) = texture_path {
            if let Some(texture) = load_texture_from_file(device, queue, path) {
                texture.create_view(&wgpu::TextureViewDescriptor::default())
            } else {
                create_fallback_texture(device, queue)
                    .create_view(&wgpu::TextureViewDescriptor::default())
            }
        } else {
            create_fallback_texture(device, queue)
                .create_view(&wgpu::TextureViewDescriptor::default())
        };

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            albedo_view,
            sampler,
        }
    }
}

/// Load a texture from file, returning None on failure
fn load_texture_from_file(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: &str,
) -> Option<wgpu::Texture> {
    let path = Path::new(path);
    if !path.exists() {
        return None;
    }

    let img = image::open(path).ok()?;
    let rgba = img.to_rgba8();
    let dimensions = rgba.dimensions();

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(&format!("Terrain Albedo: {}", path.display())),
        size: wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        },
    );

    Some(texture)
}

/// Create a 1x1 fallback texture (gray color)
fn create_fallback_texture(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture {
    let fallback_data = [128u8, 128, 128, 255]; // Gray

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Fallback Terrain Texture"),
        size: wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &fallback_data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4),
            rows_per_image: Some(1),
        },
        wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
    );

    texture
}

// GPU Renderer Types

#[cfg(feature = "desktop")]
use eframe::egui_wgpu;
#[cfg(feature = "desktop")]
use wgpu;

use std::sync::Arc;

/// Render mode for material preview
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderMode {
    #[default]
    Final,
    Albedo,
    Normal,
    Roughness,
    Metallic,
    AO,
    Depth,
}

pub struct GpuViewportRenderer {
    pub(super) device: Arc<wgpu::Device>,
    pub(super) queue: Arc<wgpu::Queue>,
    pub(super) surface_format: wgpu::TextureFormat,

    pub(super) render_pipeline: wgpu::RenderPipeline,
    pub(super) cloud_pipeline: wgpu::RenderPipeline,

    pub(super) terrain_vertex_buffer: wgpu::Buffer,
    pub(super) terrain_index_buffer: wgpu::Buffer,
    pub(super) terrain_num_indices: u32,

    pub(super) sun_vertex_buffer: wgpu::Buffer,
    pub(super) sun_index_buffer: wgpu::Buffer,
    pub(super) sun_num_indices: u32,

    pub(super) cloud_vertex_buffer: Option<wgpu::Buffer>,
    pub(super) cloud_index_buffer: Option<wgpu::Buffer>,
    pub(super) cloud_num_indices: u32,

    pub(super) camera_buffer: wgpu::Buffer,
    pub(super) camera_bind_group: wgpu::BindGroup,

    pub(super) sky_buffer: wgpu::Buffer,
    pub(super) sky_bind_group: wgpu::BindGroup,

    // Terrain texture resources
    pub(super) terrain_texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(super) terrain_texture_bind_group: wgpu::BindGroup,
    pub(super) terrain_albedo_texture_ref: Option<String>,

    // Render mode for material preview
    pub render_mode: RenderMode,
    pub render_mode_buffer: wgpu::Buffer,
    pub render_mode_bind_group: wgpu::BindGroup,

    pub(super) render_texture: Option<wgpu::Texture>,
    pub(super) depth_texture: Option<wgpu::Texture>,
    pub(super) last_size: (u32, u32),
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress
                        + std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub camera_pos: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct SkyUniform {
    pub time_of_day: f32,
    pub sun_elevation: f32,
    pub _padding1: f32,
    pub _padding2: f32,
}

/// Render mode uniform - controls material preview mode
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderModeUniform {
    pub mode: u32, // 0=Final, 1=Albedo, 2=Normal, 3=Roughness, 4=Metallic, 5=AO, 6=Depth
    pub _padding1: u32,
    pub _padding2: u32,
    pub _padding3: u32,
}

#[cfg(feature = "desktop")]
impl GpuViewportRenderer {
    pub fn new_with_render_state(render_state: &egui_wgpu::RenderState) -> Result<Self, String> {
        super::pipelines::create_renderer(render_state)
    }

    /// Set render mode for material preview
    pub fn set_render_mode(&mut self, mode: RenderMode) {
        self.render_mode = mode;
        let mode_value = match mode {
            RenderMode::Final => 0,
            RenderMode::Albedo => 1,
            RenderMode::Normal => 2,
            RenderMode::Roughness => 3,
            RenderMode::Metallic => 4,
            RenderMode::AO => 5,
            RenderMode::Depth => 6,
        };

        let uniform = RenderModeUniform {
            mode: mode_value,
            ..Default::default()
        };

        self.queue
            .write_buffer(&self.render_mode_buffer, 0, bytemuck::bytes_of(&uniform));
    }
}

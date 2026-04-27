// GPU Renderer Module - Real 3D rendering with wgpu

pub mod buffer_utils;
pub mod camera;
pub mod meshes;
pub mod pipelines;
pub mod render;
pub mod sky;
pub mod terrain_textures;
pub mod types;

pub use types::GpuViewportRenderer;

// Ensure the type is recognized as used by referencing it in a type alias
#[doc(hidden)]
pub type _GpuRendererExport = GpuViewportRenderer;

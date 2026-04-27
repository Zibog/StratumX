//! Shader definitions for viewport rendering.

/// Vertex shader for basic rendering
pub const BASIC_VERT: &str = include_str!("basic.wgsl");

/// Shader for sky rendering
pub const SKY_SHADER: &str = include_str!("sky.wgsl");

/// Shader for terrain rendering
pub const TERRAIN_SHADER: &str = include_str!("terrain.wgsl");

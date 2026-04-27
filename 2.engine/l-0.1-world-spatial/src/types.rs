//! Spatial type definitions and constants.

use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

/// Canonical region address in grid coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RegionAddress {
    pub x: i32,
    pub y: i32,
    pub slab_z: i32,
}

/// Canonical chunk address in global coordinates.
/// Chunks are directly addressed globally; region is computed from chunk coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChunkAddress {
    pub region: RegionAddress,
    pub chunk_x: i32,
    pub chunk_y: i32,
}

/// World-space coordinate in meters.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorldCoordinate {
    pub meters: Vec3,
}

/// 3D rigid transformation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

/// Coordinate system reference frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinateSpace {
    /// World-local coordinates (global origin).
    WorldLocal,
    /// Region-local coordinates (position relative to region origin).
    RegionLocal { region: RegionAddress },
    /// Presentation coordinates (rebased for camera/UI anchor).
    Presentation { anchor_region: RegionAddress },
}

/// Spatial address abstraction (region or chunk level).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpatialAddress {
    Region(RegionAddress),
    Chunk(ChunkAddress),
}

/// Classification of spatial relationship between two chunks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpatialRelation {
    /// Chunks are the same location.
    Contained,
    /// Chunks are orthogonally adjacent (±1 in x or y, same z slab).
    Adjacent,
    /// Chunks are not adjacent.
    Disjoint,
}

// Spatial constants
pub const CHUNK_EDGE_METERS: f32 = 32.0;
pub const VERTICAL_SLAB_METERS: f32 = 16.0;
pub const REGION_EDGE_CHUNKS: i32 = 32;

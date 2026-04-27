use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RegionAddress {
    pub x: i32,
    pub y: i32,
    pub slab_z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChunkAddress {
    pub region: RegionAddress,
    pub chunk_x: i32,
    pub chunk_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorldCoordinate {
    pub meters: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinateSpace {
    WorldLocal,
    RegionLocal { region: RegionAddress },
    Presentation { anchor_region: RegionAddress },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpatialAddress {
    Region(RegionAddress),
    Chunk(ChunkAddress),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialRelation {
    Adjacent,
    Contained,
    Disjoint,
}

pub const CHUNK_EDGE_METERS: f32 = 32.0;
pub const VERTICAL_SLAB_METERS: f32 = 16.0;
pub const REGION_EDGE_CHUNKS: i32 = 32;

pub fn chunk_address(chunk_x: i32, chunk_y: i32, slab_z: i32) -> ChunkAddress {
    ChunkAddress {
        region: RegionAddress {
            x: chunk_x.div_euclid(REGION_EDGE_CHUNKS),
            y: chunk_y.div_euclid(REGION_EDGE_CHUNKS),
            slab_z,
        },
        chunk_x,
        chunk_y,
    }
}

pub fn address_for_world_coordinate(coord: WorldCoordinate) -> ChunkAddress {
    let chunk_x = (coord.meters.x / CHUNK_EDGE_METERS).floor() as i32;
    let chunk_y = (coord.meters.y / CHUNK_EDGE_METERS).floor() as i32;
    let slab_z = (coord.meters.z / VERTICAL_SLAB_METERS).floor() as i32;
    chunk_address(chunk_x, chunk_y, slab_z)
}

pub fn spatial_address_for_world_coordinate(coord: WorldCoordinate) -> SpatialAddress {
    SpatialAddress::Chunk(address_for_world_coordinate(coord))
}

pub fn region_origin_meters(region: RegionAddress) -> Vec3 {
    Vec3::new(
        region.x as f32 * CHUNK_EDGE_METERS * REGION_EDGE_CHUNKS as f32,
        region.y as f32 * CHUNK_EDGE_METERS * REGION_EDGE_CHUNKS as f32,
        region.slab_z as f32 * VERTICAL_SLAB_METERS,
    )
}

pub fn chunk_origin_meters(chunk: ChunkAddress) -> Vec3 {
    Vec3::new(
        chunk.chunk_x as f32 * CHUNK_EDGE_METERS,
        chunk.chunk_y as f32 * CHUNK_EDGE_METERS,
        chunk.region.slab_z as f32 * VERTICAL_SLAB_METERS,
    )
}

pub fn to_region_local(coord: WorldCoordinate, region: RegionAddress) -> Vec3 {
    coord.meters - region_origin_meters(region)
}

pub fn rebase_for_presentation(coord: WorldCoordinate, anchor_region: RegionAddress) -> Vec3 {
    to_region_local(coord, anchor_region)
}

pub fn chunk_halo(center: ChunkAddress, radius: u8) -> Vec<ChunkAddress> {
    let radius = i32::from(radius);
    let mut output = Vec::with_capacity(((radius * 2 + 1).pow(2)) as usize);
    for chunk_y in (center.chunk_y - radius)..=(center.chunk_y + radius) {
        for chunk_x in (center.chunk_x - radius)..=(center.chunk_x + radius) {
            output.push(chunk_address(chunk_x, chunk_y, center.region.slab_z));
        }
    }
    output
}

pub fn compose_transform(parent: Transform, local: Transform) -> Transform {
    Transform {
        translation: parent.translation + local.translation,
        rotation: parent.rotation * local.rotation,
        scale: parent.scale * local.scale,
    }
}

pub fn classify_relation(a: ChunkAddress, b: ChunkAddress) -> SpatialRelation {
    if a == b {
        return SpatialRelation::Contained;
    }

    let dx = (a.chunk_x - b.chunk_x).abs();
    let dy = (a.chunk_y - b.chunk_y).abs();
    if dx <= 1 && dy <= 1 && a.region.slab_z == b.region.slab_z {
        SpatialRelation::Adjacent
    } else {
        SpatialRelation::Disjoint
    }
}

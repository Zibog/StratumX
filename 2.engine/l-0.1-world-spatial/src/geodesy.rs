use crate::types::{
    RegionAddress, WorldCoordinate, CHUNK_EDGE_METERS, REGION_EDGE_CHUNKS, VERTICAL_SLAB_METERS,
};
use crate::SpatialValidationError;
use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrecisionZoneCode {
    LocalContact,
    Interactive,
    ReducedExact,
    FarSummary,
    TheaterSummary,
}

impl PrecisionZoneCode {
    pub fn for_horizontal_distance_m(distance_m: f32) -> Self {
        match distance_m {
            d if d <= 5.0 => Self::LocalContact,
            d if d <= 500.0 => Self::Interactive,
            d if d <= 5_000.0 => Self::ReducedExact,
            d if d <= 30_000.0 => Self::FarSummary,
            _ => Self::TheaterSummary,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeoAnchorRef {
    pub coarse_world_origin_m: [i64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionFrameRef {
    pub geo_anchor_ref: GeoAnchorRef,
    pub region_frame_ref: RegionAddress,
    pub region_origin_m: [i64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellFrameRef {
    pub region_frame_ref: RegionAddress,
    pub cell_origin_m: [i32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebaseDeltaRef {
    pub from_region_frame_ref: RegionAddress,
    pub to_region_frame_ref: RegionAddress,
    pub delta_m: [i64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarPhenomenonTrackRef {
    pub track_id: u64,
    pub anchor_region_frame_ref: RegionAddress,
    pub family_tag: u16,
    pub precision_zone_code: PrecisionZoneCode,
}

impl GeoAnchorRef {
    pub fn new(coarse_world_origin_m: [i64; 3]) -> Self {
        Self {
            coarse_world_origin_m,
        }
    }
}

impl RegionFrameRef {
    pub fn new(geo_anchor_ref: GeoAnchorRef, region_frame_ref: RegionAddress) -> Self {
        Self {
            geo_anchor_ref,
            region_frame_ref,
            region_origin_m: region_origin_i64(region_frame_ref),
        }
    }
}

impl CellFrameRef {
    pub fn new(region_frame_ref: RegionAddress, cell_origin_m: [i32; 3]) -> Self {
        Self {
            region_frame_ref,
            cell_origin_m,
        }
    }
}

impl FarPhenomenonTrackRef {
    pub fn new(
        track_id: u64,
        anchor_region_frame_ref: RegionAddress,
        family_tag: u16,
        precision_zone_code: PrecisionZoneCode,
    ) -> Result<Self, SpatialValidationError> {
        if track_id == 0 || family_tag == 0 {
            return Err(SpatialValidationError::InvalidFarPhenomenonTrack);
        }
        if precision_zone_code == PrecisionZoneCode::LocalContact
            || precision_zone_code == PrecisionZoneCode::Interactive
        {
            return Err(SpatialValidationError::InvalidPrecisionZone);
        }
        Ok(Self {
            track_id,
            anchor_region_frame_ref,
            family_tag,
            precision_zone_code,
        })
    }
}

pub fn publish_rebase_delta(
    from_region_frame_ref: RegionAddress,
    to_region_frame_ref: RegionAddress,
) -> Result<RebaseDeltaRef, SpatialValidationError> {
    if from_region_frame_ref == to_region_frame_ref {
        return Err(SpatialValidationError::InvalidRebaseDelta);
    }
    let from = region_origin_vec3(from_region_frame_ref);
    let to = region_origin_vec3(to_region_frame_ref);
    let delta = to - from;
    Ok(RebaseDeltaRef {
        from_region_frame_ref,
        to_region_frame_ref,
        delta_m: [delta.x as i64, delta.y as i64, delta.z as i64],
    })
}

pub fn precision_zone_for_coordinate(
    coord: WorldCoordinate,
    anchor_region_frame_ref: RegionAddress,
) -> PrecisionZoneCode {
    let anchor_origin = region_origin_vec3(anchor_region_frame_ref);
    let planar_distance_m = (coord.meters.truncate() - anchor_origin.truncate()).length();
    PrecisionZoneCode::for_horizontal_distance_m(planar_distance_m)
}

fn region_origin_i64(region: RegionAddress) -> [i64; 3] {
    let edge_m = (CHUNK_EDGE_METERS * REGION_EDGE_CHUNKS as f32) as i64;
    let slab_m = VERTICAL_SLAB_METERS as i64;
    [
        i64::from(region.x) * edge_m,
        i64::from(region.y) * edge_m,
        i64::from(region.slab_z) * slab_m,
    ]
}

fn region_origin_vec3(region: RegionAddress) -> Vec3 {
    let origin = region_origin_i64(region);
    Vec3::new(origin[0] as f32, origin[1] as f32, origin[2] as f32)
}

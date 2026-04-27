//! Canonical public API exports for engine_world_spatial.

pub use crate::types::{
    ChunkAddress, CoordinateSpace, RegionAddress, SpatialAddress, SpatialRelation, Transform,
    WorldCoordinate, CHUNK_EDGE_METERS, REGION_EDGE_CHUNKS, VERTICAL_SLAB_METERS,
};

pub use crate::runtime::{
    address_for_world_coordinate, chunk_address, chunk_halo, chunk_origin_meters,
    classify_relation, compose_transform, rebase_for_presentation, region_origin_meters,
    spatial_address_for_world_coordinate, to_region_local,
};

pub use crate::validation::SpatialValidationError;

pub use crate::queries::SpatialQuery;

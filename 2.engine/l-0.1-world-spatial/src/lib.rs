//! Spatial hierarchy and locality operations.
//! **Owner**: engine_world_spatial — canonical source of truth for spatial addressing,
//! chunk-to-region mapping, coordinate transformations, and adjacency classification.
//!
//! ## Crate Invariants
//! - Region addresses uniquely identify spatial regions in 3D space.
//! - Chunk coordinates are globally absolute (not relative to regions).
//! - Coordinate transformations preserve spatial relationships.
//! - Adjacency classification is deterministic based on Euclidean proximity.

pub mod types;
pub use types::{
    ChunkAddress, CoordinateSpace, RegionAddress, SpatialAddress, SpatialRelation, Transform,
    WorldCoordinate, CHUNK_EDGE_METERS, REGION_EDGE_CHUNKS, VERTICAL_SLAB_METERS,
};

pub mod runtime;
pub use runtime::{
    address_for_world_coordinate, chunk_address, chunk_halo, chunk_origin_meters,
    classify_relation, compose_transform, rebase_for_presentation, region_origin_meters,
    spatial_address_for_world_coordinate, to_region_local,
};

pub mod geodesy;
pub use geodesy::{
    precision_zone_for_coordinate, publish_rebase_delta, CellFrameRef, FarPhenomenonTrackRef,
    GeoAnchorRef, PrecisionZoneCode, RebaseDeltaRef, RegionFrameRef,
};

pub mod validation;
pub use validation::SpatialValidationError;

pub mod queries;
pub use queries::SpatialQuery;

pub mod exports;

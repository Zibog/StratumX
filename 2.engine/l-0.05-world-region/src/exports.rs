//! Canonical public API exports for engine_world_region.
//!
//! This module defines the official public surface of the crate.
//! Only types and functions listed here should be used by external consumers.

pub use crate::types::{
    ChunkDescriptor, DirtyChunkEntry, DirtyFlags, DirtyRegionSnapshot, RegionDescriptor,
    RegionPriority, RegionState, RegionVersion,
};

pub use engine_world_spatial::{ChunkAddress, RegionAddress};

pub use crate::RegionSubstrate;

pub use crate::validation::RegionValidationError;

pub use crate::queries::{DirtyRegionQuery, RegionQuery};

// Re-export constants
pub use crate::types::{
    CHUNK_EDGE_METERS, MAX_FIELD_SOLVE_HALO_WIDTH, SAME_TICK_HALO_WIDTH, VERTICAL_SLAB_METERS,
};

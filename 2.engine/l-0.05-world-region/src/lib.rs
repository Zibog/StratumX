//! Region geometry and partition basics.
//! **Owner**: engine_world_region — canonical source of truth for region lifecycle,
//! state, priority, dirty tracking, and spatial bounds.
//!
//! ## Crate Invariants
//! - Region state is the canonical authority; no duplicated region states elsewhere.
//! - Region bounds and priority are immutable once assigned.
//! - Dirty chunk tracking is the sole consumer of DirtyFlags; synthesis systems consume through publications.
//! - Streaming state is observational; streaming edges are not stored here, only streamed out/in flags.

// Type definitions and IDs
pub mod types;
pub use types::{
    ChunkDescriptor, DirtyChunkEntry, DirtyFlags, DirtyRegionSnapshot, RegionDescriptor,
    RegionPriority, RegionState, RegionVersion, CHUNK_EDGE_METERS, MAX_FIELD_SOLVE_HALO_WIDTH,
    SAME_TICK_HALO_WIDTH, VERTICAL_SLAB_METERS,
};

// Re-export spatial types from dependency
pub use engine_world_spatial::{ChunkAddress, RegionAddress};

// Runtime substrate and behaviors
mod substrate;
pub use substrate::RegionSubstrate;

// Validation and error types
pub mod validation;
pub use validation::RegionValidationError;

// Query interfaces
pub mod queries;
pub use queries::{DirtyRegionQuery, RegionQuery};

// Exports for canonical visibility
pub mod exports;

//! Terrain Authority Container implementation modules.
//!
//! Contains the split impl blocks for TerrainAuthorityContainer.
//! Types are defined in types.rs module.

mod layer_ops;
mod lifecycle;
mod query_ops;
mod types;

pub use types::{
    BrushState, HeightmapData, TerrainAuthorityContainer, TerrainChunk, TerrainLayer,
    TerrainLifecycleState, TerrainRegistryState,
};

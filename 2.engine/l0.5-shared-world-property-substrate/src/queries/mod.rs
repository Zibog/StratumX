//! Query interfaces for world property substrate.

mod field_queries;
mod material_queries;
mod substrate_queries;
mod surface_queries;

pub use material_queries::PropertyQueryResult;

/// Query interface for world property substrate.
pub struct SubstrateQuery;

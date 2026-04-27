//! Query surface and deterministic reads.
//!
//! ## Crate Invariants
//! - Queries are read-only and do not mutate ECS state
//! - Query results are deterministic for a given ECS state
//! - Query descriptors uniquely identify query patterns
//! - Query execution is isolated from concurrent mutations
pub mod descriptor;
pub mod exports;
pub mod queries;
pub mod types;
pub mod validation;

pub use descriptor::QueryDescriptor;
pub use queries::QueryInterface;
pub use types::*;
pub use validation::QueryError;

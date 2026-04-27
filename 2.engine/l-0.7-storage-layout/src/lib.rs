//! StorageLayout module — defines storage layout descriptors, access patterns, and locality classification.

pub mod descriptor;
pub mod exports;
pub mod queries;
pub mod types;
pub mod validation;

pub use descriptor::*;
pub use types::*;

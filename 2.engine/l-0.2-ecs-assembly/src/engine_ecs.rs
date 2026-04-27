//! ECS assembly and world composition.
pub mod types;
pub mod runtime;
pub mod validation;
pub mod queries;
pub mod exports;

pub use types::EntityDescriptor;
pub use runtime::EcsSubstrate;
pub use validation::EcsError;
pub use queries::EcsQuery;

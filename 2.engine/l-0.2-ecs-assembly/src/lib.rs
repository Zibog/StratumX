//! ECS assembly and world composition.
//! **Owner**: engine_ecs — canonical assembly and entity-component binding.
//!
//! ## Crate Invariants
//! - Entity IDs are unique within a world
//! - Component bindings are consistent (no dangling references)
//! - Entity lifecycle is managed atomically (create/destroy)
//! - Component data is owned by the ECS substrate

pub mod types;
pub use types::EntityDescriptor;

pub mod runtime;
pub use runtime::EcsSubstrate;

pub mod validation;
pub use validation::EcsError;

pub mod queries;
pub use queries::EcsQuery;

pub mod exports;

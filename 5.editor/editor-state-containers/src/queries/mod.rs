//! Query/read model types

pub mod diagnostics_queries;
pub mod project_queries;
pub mod workspace_queries;
pub mod world_queries;

pub use diagnostics_queries::*;
pub use project_queries::*;
pub use workspace_queries::*;
pub use world_queries::*;

pub trait ReadModel<T> {
    fn build(owner: &T) -> Self;
}

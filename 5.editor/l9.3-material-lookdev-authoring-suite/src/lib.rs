// The crate root keeps only the active split while the deferred modules stay out of the build.
// mod material_authoring_service;
// mod material_cache;
pub mod material_registry_state;
mod model;
mod runtime;
mod validation;

// Public API
pub mod api;

// Re-export public API at crate root for backward compatibility with tests
pub use api::*;

// Re-export material_registry_state types explicitly to avoid ambiguity with MaterialProfile
pub use material_registry_state::{
    MaterialBinding, MaterialProfileId, MaterialRegistryState, ResponseEntry, ResponseTable,
    SurfaceFamilyId,
};
// Note: material_registry_state::MaterialProfile is NOT re-exported to avoid conflict with model::MaterialProfile

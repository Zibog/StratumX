//! Material Registry State Container
//!
//! Owns authoritative state for material profiles, surface bindings,
//! and response tables from Material_Registry.

mod ids;
mod material_binding;
mod material_profile;
mod response_table;
mod state;

pub use ids::{MaterialProfileId, SurfaceFamilyId};
pub use material_binding::MaterialBinding;
pub use material_profile::MaterialProfile;
pub use response_table::{ResponseEntry, ResponseTable};
pub use state::MaterialRegistryState;

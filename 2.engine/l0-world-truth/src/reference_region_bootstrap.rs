// Reference region bootstrap - modules

mod bootstrap;
mod reference_state;

pub use bootstrap::ReferenceRegionBootstrapper;
pub use reference_state::ReferenceRegionState;
pub type ProofRegionState = ReferenceRegionState;

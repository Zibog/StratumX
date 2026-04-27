mod branch_coverage;
mod errors;
mod ids;
mod material_profile;
mod registry;
mod runtime_rung;

pub use branch_coverage::BranchCoverage;
pub use errors::ToolingError;
pub use ids::{EntityId, ObjectHandle};
pub use material_profile::{MaterialProfile, TextureSlot};
pub use registry::{EditorProduct, MaterialRegistry};
pub use runtime_rung::RuntimeCheapnessRung;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectClass {
    World,
    Scene,
    Terrain,
    Material,
    Logic,
    Asset,
    Build,
}

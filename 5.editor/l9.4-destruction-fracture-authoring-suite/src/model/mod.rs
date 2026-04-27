mod errors;
mod field_config;
mod ids;
mod overlay;

pub use errors::DestructionError;
pub use field_config::{FieldConfig, FieldId};
pub use ids::ObjectHandle;
pub use overlay::{DisabledReason, Overlay, OverlayId, OverlayType};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectClass {
    World,
    Scene,
    Terrain,
    Material,
    Destruction,
    Logic,
    Asset,
    Build,
}

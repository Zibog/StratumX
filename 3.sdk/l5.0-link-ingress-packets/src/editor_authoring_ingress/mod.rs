pub mod animation;
pub mod diagnostics;
pub mod scene;
pub mod sky;
pub mod terrain;

pub use animation::{
    AnimationCommand, AnimationIngressPacket, AnimationStateDef, AnimationTransitionDef,
};
pub use diagnostics::{
    DestructionCommand, EcologyCommand, MaterialArchetypeInput, MaterialCommand,
    MaterialLayerInput, MaterialWorldCommand, NavDoorInventoryCommand, PopulationCommand,
    ReasonChainCommand, TacticsCommand,
};
pub use scene::{
    ActorCommand, AssetCommand, BallisticsCommand, BallisticsFireInput, SceneCommand, Transform,
};
pub use sky::{SkyCommand, StormCommand};
pub use terrain::{SurfacePaintInput, TerrainCommand, TerrainPatchInput};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EditorAuthoringCommand {
    Scene(super::scene::SceneCommand),
    Material(MaterialCommand),
    Terrain(TerrainCommand),
    Actor(super::scene::ActorCommand),
    Asset(super::scene::AssetCommand),
    Ballistics(super::scene::BallisticsCommand),
    MaterialWorld(MaterialWorldCommand),
    Destruction(DestructionCommand),
    NavDoorInventory(NavDoorInventoryCommand),
    Population(PopulationCommand),
    Tactics(TacticsCommand),
    Ecology(EcologyCommand),
    InspectReasonChain(ReasonChainCommand),
    Sky(SkyCommand),
    Storm(StormCommand),
    Animation(AnimationCommand),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorAuthoringPacket {
    pub command: EditorAuthoringCommand,
    pub request_id: u64,
}

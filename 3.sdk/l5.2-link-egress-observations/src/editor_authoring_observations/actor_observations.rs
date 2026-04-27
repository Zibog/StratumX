use super::world::{ActorDto, ActorPresetDto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActorObservation {
    ActorSpawned {
        actor: ActorDto,
    },
    ActorPresetList {
        presets: Vec<ActorPresetDto>,
    },
    WeaponAttached {
        actor_id: u32,
        weapon_profile_id: u16,
    },
    ActiveActorSet {
        actor_id: u32,
    },
    ActiveActorInfo {
        actor: Option<ActorDto>,
    },
}

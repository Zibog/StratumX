use super::world::{AuthoringSceneDto, EntityDetailsDto, EntityDto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SceneObservation {
    SceneCreated { scene: AuthoringSceneDto },
    EntityCreated { entity: EntityDto },
    EntityTransformUpdated { entity_id: u32 },
    EntityList { entities: Vec<EntityDto> },
    EntityDeleted { entity_id: u32 },
    EntityDetails { details: EntityDetailsDto },
}

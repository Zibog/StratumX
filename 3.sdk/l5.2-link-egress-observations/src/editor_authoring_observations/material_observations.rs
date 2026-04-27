use super::material::{AuthoringMaterialStackDto, MaterialArchetypeDto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MaterialObservation {
    MaterialArchetypeCreated {
        archetype: MaterialArchetypeDto,
    },
    MaterialArchetypeList {
        archetypes: Vec<MaterialArchetypeDto>,
    },
    MaterialStackCreated {
        stack: AuthoringMaterialStackDto,
    },
    MaterialStackUpdated {
        stack: AuthoringMaterialStackDto,
    },
    MaterialStackList {
        stacks: Vec<AuthoringMaterialStackDto>,
    },
    MaterialStackDetails {
        stack: AuthoringMaterialStackDto,
    },
    MaterialAssigned {
        entity_id: u32,
        slot_id: u8,
        stack_id: u16,
    },
}

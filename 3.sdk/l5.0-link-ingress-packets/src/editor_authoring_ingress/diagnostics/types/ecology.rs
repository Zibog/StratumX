use super::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcologyCommand {
    CreateCreatureEcology {
        creature_id: u32,
        species: String,
        position: [f32; 3],
    },
    SetCreatureHunger {
        hunger: f32,
    },
    SetCreatureFear {
        fear: f32,
    },
    GetCreatureState,
    EvaluateCreatureMigration,
    GetCreatureMigration,
}

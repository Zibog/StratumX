use super::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DestructionCommand {
    SetTerrainMaterial {
        material_type: String,
    },
    GetTerrainMaterial,
    TriggerBlast {
        position: [f32; 3],
        energy_j: f32,
    },
    GetTerrainBlastResponse,
    SetWallIntegrity {
        integrity: f32,
    },
    GetWallIntegrity,
    GetWallDestroyedState,
    SetSupportObjectType {
        structure_type: String,
    },
    GetSupportObjectState,
    ApplySupportDamage {
        energy_j: f32,
        impact_direction: [f32; 3],
    },
    GetDestructionSummary,
    ResetDestructionState,
}

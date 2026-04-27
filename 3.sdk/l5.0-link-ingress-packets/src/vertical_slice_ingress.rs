use serde::{Deserialize, Serialize};

// ============================================================================
// VERTICAL SLICE COMMANDS
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerticalSliceCommand {
    BootstrapScene,
    FireTestShot { weapon_entity_id: u32 },
    ResetScene,
    SelectEntity { entity_id: u32 },
    AssignMaterialStack { entity_id: u32, stack_id: u16 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerticalSliceIngressPacket {
    pub command: VerticalSliceCommand,
    pub request_id: u64,
}

impl VerticalSliceIngressPacket {
    pub fn bootstrap_scene(request_id: u64) -> Self {
        Self {
            command: VerticalSliceCommand::BootstrapScene,
            request_id,
        }
    }

    pub fn fire_test_shot(request_id: u64, weapon_entity_id: u32) -> Self {
        Self {
            command: VerticalSliceCommand::FireTestShot { weapon_entity_id },
            request_id,
        }
    }

    pub fn reset_scene(request_id: u64) -> Self {
        Self {
            command: VerticalSliceCommand::ResetScene,
            request_id,
        }
    }
}

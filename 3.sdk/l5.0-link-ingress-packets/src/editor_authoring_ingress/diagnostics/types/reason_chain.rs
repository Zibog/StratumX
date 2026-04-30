use super::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReasonChainCommand {
    InspectNpc { npc_id: u32 },
    InspectScope,
    GetStats,
}

use super::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TacticsCommand {
    CreateSquad {
        squad_id: u32,
        member_ids: Vec<u32>,
        roles: Vec<String>,
    },
    SetSquadMemberCover {
        npc_id: u32,
        cover_position: [f32; 3],
    },
    EvaluateSquadTactic {
        enemy_position: [f32; 3],
    },
    GetSquadTacticState,
    InvalidateSquadCover {
        destroyed_position: [f32; 3],
    },
    CheckCoverValidity {
        cover_position: [f32; 3],
    },
}

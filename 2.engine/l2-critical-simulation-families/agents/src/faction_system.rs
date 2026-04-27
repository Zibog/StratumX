use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactionId(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactionStatus {
    pub faction_id: FactionId,
    pub reputation: f32, // -100 to 100
    pub rank: u8,
    pub joined_at: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub member_count: u32,
    pub territory_size: f32,
    pub resources: f32,
}

impl Faction {
    pub fn can_form_base(&self) -> bool {
        self.member_count >= 3 && self.resources > 100.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FactionEvent {
    MemberJoined { npc_id: u32, reason: String },
    MemberLeft { npc_id: u32, reason: String },
    BaseFormed { position: [f32; 3], reason: String },
    ReputationChanged { old: f32, new: f32, reason: String },
}

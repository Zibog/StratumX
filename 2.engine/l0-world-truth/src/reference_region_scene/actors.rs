use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NpcSpawn {
    pub id: u64,
    pub position: [f32; 3],
    pub camp_id: Option<u64>,
    pub aggression: f32,
    pub needs_hunger: f32,
    pub needs_rest: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SquadSpawn {
    pub id: u64,
    pub leader_position: [f32; 3],
    pub member_count: usize,
    pub hostile: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatureSpawn {
    pub id: u64,
    pub species: String,
    pub position: [f32; 3],
    pub migration_target: [f32; 3],
}

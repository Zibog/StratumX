use super::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PopulationCommand {
    CreateNpcProfile {
        npc_id: u32,
        name: String,
        position: [f32; 3],
    },
    SetNpcTraits {
        aggression: f32,
        greed: f32,
        loyalty: f32,
        courage: f32,
        discipline: f32,
        sociability: f32,
    },
    GetNpcTraits,
    SetNpcNeed {
        need_type: String,
        value: f32,
    },
    GetNpcNeed {
        need_type: String,
    },
    SetNpcActivity {
        activity: String,
        start_time: f32,
        duration: f32,
        location: [f32; 3],
    },
    GetNpcSchedule,
    IncreaseScarcity {
        scarcity_factor: f32,
    },
    GetCrimePressure,
    EvaluateCrimeEscalation {
        scarcity_factor: f32,
    },
    GetCriminalStatus,
    SetNpcFaction {
        faction_id: u32,
        reputation: f32,
    },
    GetNpcFaction,
}

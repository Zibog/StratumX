use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SquadRole {
    Leader,
    Suppressor,
    Flanker,
    Support,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TacticState {
    Idle,
    Advancing,
    Suppressing,
    Flanking,
    Regrouping,
    Retreating,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SquadMember {
    pub npc_id: u32,
    pub role: SquadRole,
    pub position: [f32; 3],
    pub in_cover: bool,
    pub cover_position: Option<[f32; 3]>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Squad {
    pub id: u32,
    pub members: Vec<SquadMember>,
    pub tactic_state: TacticState,
    pub target_position: Option<[f32; 3]>,
}

impl Squad {
    pub fn plan_suppress_and_flank(&mut self, enemy_position: [f32; 3]) -> Vec<TacticReason> {
        let mut reasons = Vec::new();

        for member in &mut self.members {
            match member.role {
                SquadRole::Suppressor => {
                    self.tactic_state = TacticState::Suppressing;
                    reasons.push(TacticReason::SuppressingEnemy {
                        npc_id: member.npc_id,
                        target: enemy_position,
                    });
                }
                SquadRole::Flanker => {
                    self.tactic_state = TacticState::Flanking;
                    reasons.push(TacticReason::FlankingEnemy {
                        npc_id: member.npc_id,
                        flank_position: [
                            enemy_position[0] + 5.0,
                            enemy_position[1],
                            enemy_position[2],
                        ],
                    });
                }
                _ => {}
            }
        }

        reasons
    }

    pub fn invalidate_cover(&mut self, destroyed_position: [f32; 3]) -> Vec<TacticReason> {
        let mut reasons = Vec::new();

        for member in &mut self.members {
            if let Some(cover_pos) = member.cover_position {
                let distance = ((cover_pos[0] - destroyed_position[0]).powi(2)
                    + (cover_pos[2] - destroyed_position[2]).powi(2))
                .sqrt();

                if distance < 2.0 {
                    member.in_cover = false;
                    member.cover_position = None;
                    self.tactic_state = TacticState::Regrouping;

                    reasons.push(TacticReason::CoverDestroyed {
                        npc_id: member.npc_id,
                        old_cover: cover_pos,
                        destruction_position: destroyed_position,
                    });
                }
            }
        }

        reasons
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TacticReason {
    SuppressingEnemy {
        npc_id: u32,
        target: [f32; 3],
    },
    FlankingEnemy {
        npc_id: u32,
        flank_position: [f32; 3],
    },
    CoverDestroyed {
        npc_id: u32,
        old_cover: [f32; 3],
        destruction_position: [f32; 3],
    },
    RegroupingAfterLoss {
        npc_id: u32,
        reason: String,
    },
}

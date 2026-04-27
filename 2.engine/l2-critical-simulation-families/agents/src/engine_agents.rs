pub mod creature_ecology;
pub mod crime_escalation;
pub mod faction_system;
pub mod npc_personality;
pub mod reason_chain;
pub mod squad_tactics;

pub use creature_ecology::*;
pub use crime_escalation::*;
pub use faction_system::*;
pub use npc_personality::*;
pub use reason_chain::*;
pub use squad_tactics::*;

use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use engine_ecs::EcsSubstrate;
use engine_world::{ApplySegment, WorldState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub max_action_intents: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentsContext {
    pub tick: Tick,
    pub region_key: (i32, i32, i32),
    pub action_intent_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentsDelta {
    pub apply_segments: Vec<ApplySegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentsMetrics {
    pub action_intent_count: usize,
}

#[derive(Debug, Clone)]
pub struct AgentsFamily {
    config: AgentsConfig,
}

impl AgentsFamily {
    pub fn new(config: AgentsConfig) -> Self {
        Self { config }
    }
    pub fn simulate(
        &self,
        _ecs: &EcsSubstrate,
        _world: &WorldState,
        context: AgentsContext,
    ) -> EngineCoreResult<(AgentsDelta, AgentsMetrics)> {
        if context.action_intent_count > self.config.max_action_intents {
            return Err(EngineCoreError::InvalidDescriptor(
                "action intent count exceeds configured ceiling",
            ));
        }
        Ok((
            AgentsDelta {
                apply_segments: vec![ApplySegment {
                    region_key: context.region_key,
                    family_tags: vec![30],
                }],
            },
            AgentsMetrics {
                action_intent_count: context.action_intent_count,
            },
        ))
    }
}

use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AgentId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentState {
    pub id: AgentId,
    pub region_key: (i32, i32, i32),
    pub energy: u8,
    pub alertness: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentIntentKind {
    Observe,
    Reposition,
    Recover,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentIntent {
    pub agent_id: AgentId,
    pub kind: AgentIntentKind,
    pub priority: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentTickSummary {
    pub intents: Vec<AgentIntent>,
}

#[derive(Debug, Clone, Default)]
pub struct AgentsSubstrate {
    states: BTreeMap<AgentId, AgentState>,
}

impl AgentsSubstrate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_agent(&mut self, state: AgentState) -> EngineCoreResult<()> {
        if state.id.0 == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "agent substrate requires non-zero identity",
            ));
        }
        if self.states.insert(state.id, state).is_some() {
            return Err(EngineCoreError::InvalidDescriptor(
                "agent identity must remain unique",
            ));
        }
        Ok(())
    }

    pub fn agent(&self, agent_id: AgentId) -> Option<&AgentState> {
        self.states.get(&agent_id)
    }

    pub fn tick(&self) -> AgentTickSummary {
        let intents = self
            .states
            .values()
            .map(|state| AgentIntent {
                agent_id: state.id,
                kind: if state.energy <= 2 {
                    AgentIntentKind::Recover
                } else if state.alertness >= 7 {
                    AgentIntentKind::Reposition
                } else {
                    AgentIntentKind::Observe
                },
                priority: if state.energy <= 2 {
                    3
                } else if state.alertness >= 7 {
                    2
                } else {
                    1
                },
            })
            .collect();
        AgentTickSummary { intents }
    }
}

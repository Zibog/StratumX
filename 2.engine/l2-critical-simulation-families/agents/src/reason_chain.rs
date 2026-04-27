// Reason Chain State - Canonical Diagnostics Truth Layer
//
// DecisionTraceEvent: Domain fact about why an entity/system made a decision
// ReasonChainState: Canonical diagnostics truth layer for causality inspection

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionTraceEvent {
    CrimeCommitted {
        npc_id: u32,
        crime_type: String,
        reason: String,
        timestamp: f32,
    },
    TacticExecuted {
        squad_id: u32,
        tactic: String,
        reason: String,
        timestamp: f32,
    },
    FactionChanged {
        npc_id: u32,
        faction_id: u32,
        reputation_delta: f32,
        reason: String,
        timestamp: f32,
    },
    NavigationFailed {
        npc_id: u32,
        destination: [f32; 3],
        reason: String,
        timestamp: f32,
    },
    MigrationTriggered {
        creature_id: u32,
        species: String,
        destination: [f32; 3],
        reason: String,
        timestamp: f32,
    },
    NeedEscalated {
        npc_id: u32,
        need_type: String,
        value: f32,
        reason: String,
        timestamp: f32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasonChainState {
    pub events: Vec<DecisionTraceEvent>,
    pub capacity: usize,
}

impl ReasonChainState {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn record(&mut self, event: DecisionTraceEvent) {
        if self.events.len() >= self.capacity {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn query_npc(&self, npc_id: u32) -> Vec<&DecisionTraceEvent> {
        self.events
            .iter()
            .filter(|e| match e {
                DecisionTraceEvent::CrimeCommitted { npc_id: id, .. } => *id == npc_id,
                DecisionTraceEvent::FactionChanged { npc_id: id, .. } => *id == npc_id,
                DecisionTraceEvent::NavigationFailed { npc_id: id, .. } => *id == npc_id,
                DecisionTraceEvent::NeedEscalated { npc_id: id, .. } => *id == npc_id,
                _ => false,
            })
            .collect()
    }

    pub fn query_scope(&self) -> &[DecisionTraceEvent] {
        &self.events
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn explain(&self, event: &DecisionTraceEvent) -> String {
        match event {
            DecisionTraceEvent::CrimeCommitted {
                npc_id,
                crime_type,
                reason,
                timestamp,
            } => {
                format!(
                    "NPC {} committed {} at {:.2}s: {}",
                    npc_id, crime_type, timestamp, reason
                )
            }
            DecisionTraceEvent::TacticExecuted {
                squad_id,
                tactic,
                reason,
                timestamp,
            } => {
                format!(
                    "Squad {} executed {} at {:.2}s: {}",
                    squad_id, tactic, timestamp, reason
                )
            }
            DecisionTraceEvent::FactionChanged {
                npc_id,
                faction_id,
                reputation_delta,
                reason,
                timestamp,
            } => {
                format!(
                    "NPC {} changed faction {} by {:.2} at {:.2}s: {}",
                    npc_id, faction_id, reputation_delta, timestamp, reason
                )
            }
            DecisionTraceEvent::NavigationFailed {
                npc_id,
                destination,
                reason,
                timestamp,
            } => {
                format!(
                    "NPC {} navigation to {:?} failed at {:.2}s: {}",
                    npc_id, destination, timestamp, reason
                )
            }
            DecisionTraceEvent::MigrationTriggered {
                creature_id,
                species,
                destination,
                reason,
                timestamp,
            } => {
                format!(
                    "Creature {} ({}) migrated to {:?} at {:.2}s: {}",
                    creature_id, species, destination, timestamp, reason
                )
            }
            DecisionTraceEvent::NeedEscalated {
                npc_id,
                need_type,
                value,
                reason,
                timestamp,
            } => {
                format!(
                    "NPC {} need {} escalated to {:.1} at {:.2}s: {}",
                    npc_id, need_type, value, timestamp, reason
                )
            }
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for ReasonChainState {
    fn default() -> Self {
        Self::new(1000)
    }
}

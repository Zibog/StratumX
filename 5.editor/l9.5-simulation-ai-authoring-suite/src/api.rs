use crate::ai_agent::*;
use crate::behavior_tree::*;
use std::collections::HashMap;

pub struct AIAuthoringService {
    agents: HashMap<AIAgentId, AIAgent>,
    trees: HashMap<uuid::Uuid, BehaviorTree>,
}

impl AIAuthoringService {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            trees: HashMap::new(),
        }
    }

    pub fn create_agent(&mut self, name: String) -> AIAgentId {
        let agent = AIAgent::new(name);
        let id = agent.id;
        self.agents.insert(id, agent);
        id
    }
}

impl Default for AIAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}
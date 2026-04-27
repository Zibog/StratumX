use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorNode {
    Sequence(Vec<BehaviorNode>),
    Selector(Vec<BehaviorNode>),
    Action(String),
    Condition(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTree {
    pub id: Uuid,
    pub name: String,
    pub root: BehaviorNode,
}

impl BehaviorTree {
    pub fn new(name: String, root: BehaviorNode) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            root,
        }
    }
}
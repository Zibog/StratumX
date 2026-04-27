use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    QuestCompleted(String),
    ItemOwned(String),
    LocationVisited(String),
    And(Vec<Condition>),
    Or(Vec<Condition>),
}

impl Condition {
    pub fn evaluate(&self) -> bool {
        match self {
            Condition::And(conditions) => conditions.iter().all(|c| c.evaluate()),
            Condition::Or(conditions) => conditions.iter().any(|c| c.evaluate()),
            _ => false,
        }
    }
}
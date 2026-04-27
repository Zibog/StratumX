use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub id: Uuid,
    pub name: String,
    pub nodes: Vec<Uuid>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: Uuid,
    pub to: Uuid,
}

impl Graph {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}
use crate::graph::*;
use std::collections::HashMap;

pub struct GraphAuthoringService {
    graphs: HashMap<uuid::Uuid, Graph>,
}

impl GraphAuthoringService {
    pub fn new() -> Self {
        Self {
            graphs: HashMap::new(),
        }
    }

    pub fn create_graph(&mut self, name: String) -> uuid::Uuid {
        let graph = Graph::new(name);
        let id = graph.id;
        self.graphs.insert(id, graph);
        id
    }
}

impl Default for GraphAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}
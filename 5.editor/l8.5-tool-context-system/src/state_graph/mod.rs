//! State Graph
//!
//! Maintains a directed acyclic graph (DAG) of state dependencies. The graph is
//! split into explicit owner, mutation, and publication surfaces.

use std::collections::HashMap;

use crate::runtime::state_container_system::StateId;

mod mutation_graph;
mod owner_graph;
mod publication_graph;

pub use owner_graph::{StateNode, StateNodeType};

/// State dependency graph.
///
/// Maintains a DAG showing dependencies between state elements.
/// All edges point from derived state to owner containers.
#[derive(Debug)]
pub struct StateGraph {
    /// Nodes in the graph.
    nodes: HashMap<StateId, StateNode>,

    /// Edges: (from_derived, to_owner)
    /// Represents "from_derived depends on to_owner".
    pub edges: Vec<(StateId, StateId)>,
}

impl StateGraph {
    /// Creates a new empty state graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }
}

impl Default for StateGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;

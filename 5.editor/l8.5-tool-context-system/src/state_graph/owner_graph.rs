use crate::runtime::state_container_system::{OwnerId, StateId};

use super::StateGraph;

/// State graph node type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateNodeType {
    /// Owner container (authoritative state).
    OwnerContainer,

    /// Substate (owned by a container).
    Substate,

    /// Derived state (computed from owner containers).
    DerivedState,
}

/// State graph node.
#[derive(Debug, Clone)]
pub struct StateNode {
    /// State identifier.
    pub id: StateId,

    /// Node type.
    pub node_type: StateNodeType,

    /// Owner (if any).
    pub owner: Option<OwnerId>,
}

impl StateNode {
    /// Creates a new state node.
    pub fn new(id: StateId, node_type: StateNodeType, owner: Option<OwnerId>) -> Self {
        Self {
            id,
            node_type,
            owner,
        }
    }
}

impl StateGraph {
    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: StateNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Gets a node by ID.
    pub fn get_node(&self, state_id: &StateId) -> Option<&StateNode> {
        self.nodes.get(state_id)
    }

    /// Gets all nodes.
    pub fn get_all_nodes(&self) -> Vec<&StateNode> {
        self.nodes.values().collect()
    }
}

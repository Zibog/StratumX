use crate::runtime::state_container_system::StateId;

use super::{StateGraph, StateNodeType};

impl StateGraph {
    /// Adds a dependency edge.
    ///
    /// Represents "derived depends on owner".
    pub fn add_dependency(&mut self, derived: StateId, owner: StateId) -> Result<(), String> {
        if !self.nodes.contains_key(&derived) {
            return Err(format!("Derived state {:?} not found in graph", derived));
        }
        if !self.nodes.contains_key(&owner) {
            return Err(format!("Owner state {:?} not found in graph", owner));
        }

        if let Some(derived_node) = self.nodes.get(&derived) {
            if derived_node.node_type == StateNodeType::DerivedState {
                if let Some(owner_node) = self.nodes.get(&owner) {
                    if owner_node.node_type == StateNodeType::DerivedState {
                        return Err(format!(
                            "Derived state {:?} cannot depend on another derived state {:?}",
                            derived, owner
                        ));
                    }
                }
            }
        }

        self.edges.push((derived.clone(), owner.clone()));

        if let Err(cycle) = self.validate_acyclic() {
            self.edges.pop();
            return Err(format!("Adding dependency would create cycle: {:?}", cycle));
        }

        Ok(())
    }

    /// Gets all dependencies of a state (states it depends on).
    pub fn get_dependencies(&self, state_id: &StateId) -> Vec<StateId> {
        self.edges
            .iter()
            .filter(|(from, _)| from == state_id)
            .map(|(_, to)| to.clone())
            .collect()
    }

    /// Gets all dependents of a state (states that depend on it).
    pub fn get_dependents(&self, state_id: &StateId) -> Vec<StateId> {
        self.edges
            .iter()
            .filter(|(_, to)| to == state_id)
            .map(|(from, _)| from.clone())
            .collect()
    }
}

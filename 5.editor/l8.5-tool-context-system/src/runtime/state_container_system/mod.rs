//! State Container System
//!
//! Thin orchestration seam for state ownership and validation.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::state_graph::StateGraph;
use crate::{DiagnosticsState, ProjectState, WorkspaceState, WorldState};

mod ownership_bindings;
mod ownership_validation;
mod state_ids;
mod violation_types;

pub use state_ids::{OwnerId, StateId};
pub use violation_types::{OwnershipViolation, ViolationType};

/// State container system coordinator.
///
/// Manages ownership mapping and validation for all editor state.
/// Ensures exactly one owner per state item.
pub struct StateContainerSystem {
    /// Reference to project state container.
    pub project_state: Arc<Mutex<ProjectState>>,

    /// Reference to workspace state container.
    pub workspace_state: Arc<Mutex<WorkspaceState>>,

    /// Reference to world state container (optional, only when world is open).
    pub world_state: Option<Arc<Mutex<WorldState>>>,

    /// Reference to diagnostics state container.
    pub diagnostics_state: Arc<Mutex<DiagnosticsState>>,

    /// State dependency graph.
    state_graph: StateGraph,

    /// Ownership mapping: StateId -> OwnerId.
    ownership_map: HashMap<StateId, OwnerId>,
}

impl StateContainerSystem {
    /// Creates a new state container system.
    ///
    /// Validates ownership uniqueness during initialization.
    /// Returns Err if any ownership violations are detected.
    pub fn new(
        project_state: Arc<Mutex<ProjectState>>,
        workspace_state: Arc<Mutex<WorkspaceState>>,
        diagnostics_state: Arc<Mutex<DiagnosticsState>>,
    ) -> Result<Self, Vec<OwnershipViolation>> {
        let mut system = Self {
            project_state,
            workspace_state,
            world_state: None,
            diagnostics_state,
            state_graph: StateGraph::new(),
            ownership_map: HashMap::new(),
        };

        system.register_binding_entries(ownership_bindings::BASE_OWNERSHIP_BINDINGS);
        system.validate_ownership_uniqueness()?;

        Ok(system)
    }

    /// Sets the world state container (when a world is opened).
    pub fn set_world_state(&mut self, world_state: Arc<Mutex<WorldState>>) {
        self.world_state = Some(world_state);
        self.register_binding_entries(ownership_bindings::WORLD_OWNERSHIP_BINDINGS);
    }

    /// Clears the world state container (when a world is closed).
    pub fn clear_world_state(&mut self) {
        self.world_state = None;
        self.remove_binding_entries(ownership_bindings::WORLD_REQUIRED_STATES);
    }

    /// Returns true when a world owner is currently bound into the system.
    pub fn has_world_state(&self) -> bool {
        self.world_state.is_some()
    }

    /// Returns the states that must always have an owner.
    pub fn always_required_states() -> &'static [StateId] {
        ownership_bindings::ALWAYS_REQUIRED_STATES
    }

    /// Returns the world-scoped states that are required only when a world is bound.
    pub fn world_required_states() -> &'static [StateId] {
        ownership_bindings::WORLD_REQUIRED_STATES
    }

    /// Gets the owner of a specific state item.
    pub fn get_owner(&self, state_id: &StateId) -> Option<OwnerId> {
        self.ownership_map.get(state_id).copied()
    }

    /// Registers a derived state dependency.
    ///
    /// This is used by the state graph to track which owner containers
    /// a derived state depends on.
    pub fn register_derived_state(&mut self, derived_id: StateId, owner_id: OwnerId) {
        self.ownership_map.insert(derived_id, owner_id);
    }

    /// Gets the state graph.
    pub fn get_state_graph(&self) -> &StateGraph {
        &self.state_graph
    }

    /// Gets a mutable reference to the state graph.
    pub fn get_state_graph_mut(&mut self) -> &mut StateGraph {
        &mut self.state_graph
    }

    fn register_binding_entries(&mut self, entries: &[(StateId, OwnerId)]) {
        for (state_id, owner_id) in entries {
            self.ownership_map.insert(*state_id, *owner_id);
        }
    }

    fn remove_binding_entries(&mut self, states: &[StateId]) {
        for state_id in states {
            self.ownership_map.remove(state_id);
        }
    }
}

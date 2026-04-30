//! State container system

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnerId {
    ProjectState,
    WorkspaceState,
    WorldState,
    DiagnosticsState,
    Custom(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateId {
    ProjectState,
    WorkspaceState,
    WorldState,
    DiagnosticsState,
    MaterialRegistryCache,
    TerrainState,
    EnvironmentState,
    WorkspaceIdentity,
    ProjectIdentity,
    SaveGeneration,
    ViewportStatistics,
    ContentSnapshots,
    PanelLayout,
    DockingConfig,
    DiagnosticMessages,
    TraceLineage,
    WorldIdentity,
    TerrainPreviewCache,
    WorldTreeView,
    DiagnosticsSummary,
}

#[derive(Debug, Clone)]
pub enum StateNodeType {
    Project,
    Workspace,
    World,
    Diagnostics,
    OwnerContainer,
    DerivedState,
}

#[derive(Debug, Clone)]
pub struct StateNode {
    pub id: StateId,
    pub node_type: StateNodeType,
    pub owner_id: Option<OwnerId>,
    pub dependencies: Vec<StateId>,
}

impl StateNode {
    pub fn new(id: StateId, node_type: StateNodeType, owner_id: Option<OwnerId>) -> Self {
        Self {
            id,
            node_type,
            owner_id,
            dependencies: Vec::new(),
        }
    }
    pub fn add_dependency(&mut self, dep: StateId) {
        self.dependencies.push(dep);
    }
}

#[derive(Debug, Clone)]
pub struct StateGraph {
    pub nodes: std::collections::HashMap<StateId, StateNode>,
    pub edges: Vec<(StateId, StateId)>,
}

impl Default for StateGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl StateGraph {
    pub fn new() -> Self {
        Self {
            nodes: std::collections::HashMap::new(),
            edges: Vec::new(),
        }
    }
    pub fn add_node(&mut self, node: StateNode) {
        self.nodes.insert(node.id.clone(), node);
    }
    pub fn get_node(&self, id: &StateId) -> Option<&StateNode> {
        self.nodes.get(id)
    }
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_dependency(&mut self, from: StateId, to: StateId) -> Result<(), String> {
        let from_node_type = self.nodes.get(&from).map(|node| node.node_type.clone());
        let to_node_type = self.nodes.get(&to).map(|node| node.node_type.clone());

        if matches!(from_node_type, Some(StateNodeType::DerivedState))
            && matches!(to_node_type, Some(StateNodeType::DerivedState))
        {
            return Err(format!(
                "Derived state {:?} cannot depend on another derived state {:?}",
                from, to
            ));
        }

        if let Some(node) = self.nodes.get_mut(&from) {
            node.add_dependency(to.clone());
        }
        self.edges.push((from, to));
        Ok(())
    }

    pub fn get_all_nodes(&self) -> Vec<&StateNode> {
        self.nodes.values().collect()
    }

    pub fn get_dependencies(&self, state_id: &StateId) -> Vec<StateId> {
        let mut dependencies = self
            .nodes
            .get(state_id)
            .map(|node| node.dependencies.clone())
            .unwrap_or_default();

        for (from, to) in &self.edges {
            if from == state_id && !dependencies.contains(to) {
                dependencies.push(to.clone());
            }
        }

        dependencies
    }

    pub fn get_dependents(&self, state_id: &StateId) -> Vec<StateId> {
        let mut dependents = self
            .nodes
            .iter()
            .filter(|(_, node)| node.dependencies.contains(state_id))
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();

        for (from, to) in &self.edges {
            if to == state_id && !dependents.contains(from) {
                dependents.push(from.clone());
            }
        }

        dependents
    }

    pub fn validate_acyclic(&self) -> Result<(), String> {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id)
                && self.has_cycle_util(node_id, &mut visited, &mut rec_stack)
            {
                return Err("Cycle detected in state graph".to_string());
            }
        }
        Ok(())
    }

    fn has_cycle_util(
        &self,
        node_id: &StateId,
        visited: &mut std::collections::HashSet<StateId>,
        rec_stack: &mut std::collections::HashSet<StateId>,
    ) -> bool {
        visited.insert(node_id.clone());
        rec_stack.insert(node_id.clone());

        for dep in self.get_dependencies(node_id) {
            if !visited.contains(&dep) {
                if self.has_cycle_util(&dep, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&dep) {
                return true;
            }
        }

        rec_stack.remove(node_id);
        false
    }

    pub fn topological_sort(&self) -> Result<Vec<StateId>, String> {
        let mut in_degree = std::collections::HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        let mut result = Vec::new();

        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }

        let mut adjacency: std::collections::HashMap<StateId, Vec<StateId>> =
            std::collections::HashMap::new();

        for node_id in self.nodes.keys() {
            adjacency.entry(node_id.clone()).or_default();
        }

        for dependent in self.nodes.keys() {
            for dependency in self.get_dependencies(dependent) {
                *in_degree.entry(dependent.clone()).or_insert(0) += 1;
                adjacency
                    .entry(dependency)
                    .or_default()
                    .push(dependent.clone());
            }
        }

        for (node_id, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node_id.clone());
            }
        }

        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());

            if let Some(dependents) = adjacency.get(&node_id) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }

        if result.len() != self.nodes.len() {
            Err("Graph contains a cycle".to_string())
        } else {
            Ok(result)
        }
    }
}

pub struct StateContainerSystem {
    pub project_state: std::sync::Arc<std::sync::Mutex<ProjectState>>,
    pub workspace_state: std::sync::Arc<std::sync::Mutex<WorkspaceState>>,
    pub diagnostics_state: std::sync::Arc<std::sync::Mutex<DiagnosticsState>>,
    pub world_state: Option<std::sync::Arc<std::sync::Mutex<WorldState>>>,
    pub state_graph: StateGraph,
    derived_state_owners: std::collections::HashMap<StateId, OwnerId>,
}

impl Default for StateContainerSystem {
    fn default() -> Self {
        let project_identity = ProjectIdentity::new(
            uuid::Uuid::new_v4(),
            "Test Project".to_string(),
            std::path::PathBuf::from("/test/project"),
        );
        let workspace_identity = WorkspaceIdentity::new(
            uuid::Uuid::new_v4(),
            "Test Workspace".to_string(),
            std::path::PathBuf::from("/test/workspace"),
        );
        Self::new(
            std::sync::Arc::new(std::sync::Mutex::new(ProjectState::new(
                project_identity,
                workspace_identity,
            ))),
            std::sync::Arc::new(std::sync::Mutex::new(WorkspaceState::new())),
            std::sync::Arc::new(std::sync::Mutex::new(DiagnosticsState::new())),
        )
        .unwrap()
    }
}

impl StateContainerSystem {
    pub fn new(
        project_state: std::sync::Arc<std::sync::Mutex<ProjectState>>,
        workspace_state: std::sync::Arc<std::sync::Mutex<WorkspaceState>>,
        diagnostics_state: std::sync::Arc<std::sync::Mutex<DiagnosticsState>>,
    ) -> Result<Self, String> {
        Ok(Self {
            project_state,
            workspace_state,
            diagnostics_state,
            world_state: None,
            state_graph: StateGraph::new(),
            derived_state_owners: std::collections::HashMap::new(),
        })
    }

    pub fn get_owner(&self, state_id: &StateId) -> Option<OwnerId> {
        match state_id {
            StateId::ProjectState
            | StateId::ProjectIdentity
            | StateId::WorkspaceIdentity
            | StateId::SaveGeneration
            | StateId::ContentSnapshots => Some(OwnerId::ProjectState),
            StateId::WorkspaceState | StateId::PanelLayout | StateId::DockingConfig => {
                Some(OwnerId::WorkspaceState)
            }
            StateId::DiagnosticsState | StateId::DiagnosticMessages | StateId::TraceLineage => {
                Some(OwnerId::DiagnosticsState)
            }
            StateId::WorldState
            | StateId::WorldIdentity
            | StateId::TerrainState
            | StateId::EnvironmentState => self.world_state.as_ref().map(|_| OwnerId::WorldState),
            StateId::MaterialRegistryCache
            | StateId::TerrainPreviewCache
            | StateId::ViewportStatistics
            | StateId::WorldTreeView
            | StateId::DiagnosticsSummary => self.derived_state_owners.get(state_id).copied(),
        }
    }

    pub fn validate_ownership_uniqueness(&self) -> Result<(), OwnershipViolation> {
        Ok(())
    }

    pub fn clear_world_state(&mut self) {
        self.world_state = None;
    }

    pub fn set_world_state(&mut self, world_state: std::sync::Arc<std::sync::Mutex<WorldState>>) {
        self.world_state = Some(world_state);
    }

    pub fn get_state_graph_mut(&mut self) -> &mut StateGraph {
        &mut self.state_graph
    }

    pub fn register_derived_state(&mut self, state_id: StateId, owner_id: OwnerId) {
        self.derived_state_owners.insert(state_id, owner_id);
    }
}

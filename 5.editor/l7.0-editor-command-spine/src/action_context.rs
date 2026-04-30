//! ActionContext — execution context for actions.

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub project_state: Option<ProjectState>,
    pub session_state: Option<SessionState>,
    pub selection_state: SelectionState,
    pub focus_state: FocusState,
    pub workspace_state: Option<WorkspaceState>,
    pub transaction_state: Option<TransactionState>,
}

impl ActionContext {
    pub fn project(&self) -> Option<&ProjectState> {
        self.project_state.as_ref()
    }
    pub fn session(&self) -> Option<&SessionState> {
        self.session_state.as_ref()
    }
    pub fn selection(&self) -> &SelectionState {
        &self.selection_state
    }
    pub fn focus(&self) -> &FocusState {
        &self.focus_state
    }
    pub fn workspace(&self) -> Option<&WorkspaceState> {
        self.workspace_state.as_ref()
    }
    pub fn transaction(&self) -> Option<&TransactionState> {
        self.transaction_state.as_ref()
    }
    pub fn has_material_registry(&self) -> bool {
        self.session_state
            .as_ref()
            .map(|s| s.material_registry_available)
            .unwrap_or(false)
    }
    pub fn has_audio_registry(&self) -> bool {
        self.session_state
            .as_ref()
            .map(|s| s.audio_registry_available)
            .unwrap_or(false)
    }
    pub fn has_world_registry(&self) -> bool {
        self.session_state
            .as_ref()
            .and_then(|s| s.active_world_id)
            .is_some()
    }
    pub fn has_diagnostics_registry(&self) -> bool {
        self.session_state
            .as_ref()
            .map(|s| s.diagnostics_available)
            .unwrap_or(false)
    }
    pub fn has_unsaved_changes(&self) -> bool {
        self.workspace_state
            .as_ref()
            .map(|s| s.has_unsaved_changes)
            .unwrap_or(false)
    }
    pub fn is_loading(&self) -> bool {
        self.session_state
            .as_ref()
            .map(|s| s.loading_in_progress)
            .unwrap_or(false)
    }
    pub fn has_project(&self) -> bool {
        self.project_state.is_some()
    }
    pub fn has_active_world(&self) -> bool {
        self.session_state
            .as_ref()
            .and_then(|s| s.active_world_id)
            .is_some()
    }
    pub fn has_selection(&self) -> bool {
        !self.selection_state.selected_entities.is_empty()
    }

    pub fn test_empty() -> Self {
        Self {
            project_state: None,
            session_state: None,
            selection_state: SelectionState::default(),
            focus_state: FocusState::default(),
            workspace_state: None,
            transaction_state: None,
        }
    }
    pub fn test_with_project(project_id: Uuid) -> Self {
        Self {
            project_state: Some(ProjectState {
                project_id,
                project_path: "/test/project".into(),
            }),
            session_state: None,
            selection_state: SelectionState::default(),
            focus_state: FocusState::default(),
            workspace_state: None,
            transaction_state: None,
        }
    }
    pub fn test_with_world(project_id: Uuid, world_id: Uuid) -> Self {
        Self {
            project_state: Some(ProjectState {
                project_id,
                project_path: "/test/project".into(),
            }),
            session_state: Some(SessionState {
                active_world_id: Some(world_id),
                material_registry_available: true,
                audio_registry_available: true,
                diagnostics_available: true,
                loading_in_progress: false,
            }),
            selection_state: SelectionState::default(),
            focus_state: FocusState::default(),
            workspace_state: None,
            transaction_state: None,
        }
    }
    pub fn test_with_selection(project_id: Uuid, world_id: Uuid, entities: Vec<Uuid>) -> Self {
        Self {
            project_state: Some(ProjectState {
                project_id,
                project_path: "/test/project".into(),
            }),
            session_state: Some(SessionState {
                active_world_id: Some(world_id),
                material_registry_available: true,
                audio_registry_available: true,
                diagnostics_available: true,
                loading_in_progress: false,
            }),
            selection_state: SelectionState {
                selected_entities: entities,
            },
            focus_state: FocusState::default(),
            workspace_state: None,
            transaction_state: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectState {
    pub project_id: Uuid,
    pub project_path: String,
}
#[derive(Debug, Clone)]
pub struct SessionState {
    pub active_world_id: Option<Uuid>,
    pub material_registry_available: bool,
    pub audio_registry_available: bool,
    pub diagnostics_available: bool,
    pub loading_in_progress: bool,
}
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    pub selected_entities: Vec<Uuid>,
}
#[derive(Debug, Clone, Default)]
pub struct FocusState {
    pub focused_panel: Option<String>,
}
#[derive(Debug, Clone)]
pub struct WorkspaceState {
    pub workspace_id: Uuid,
    pub has_unsaved_changes: bool,
}
#[derive(Debug, Clone)]
pub struct TransactionState {
    pub transaction_id: Uuid,
    pub transaction_active: bool,
}

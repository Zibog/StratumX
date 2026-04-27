use crate::bridge_types::*;
use crate::tooling_types::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ToolingRuntime {
    pub(crate) bridge: BridgeRuntime,
    pub(crate) session: Option<SessionHandle>,
    pub(crate) generation: u64,
    pub(crate) next_order: u64,
    pub(crate) ledger: Vec<ToolTransaction>,
    pub(crate) preview_cache: Vec<PreviewResult>,
    pub(crate) validation_history: Vec<Vec<ValidationDiagnostic>>,
    pub(crate) workspace: WorkspaceState,
    pub proposals: Vec<AssistantProposal>,
    pub(crate) next_proposal_id: u64,
    pub(crate) assistant_evidence: Vec<AssistantEvidence>,
}

impl Default for ToolingRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolingRuntime {
    pub fn new() -> Self {
        let bridge = BridgeRuntime::new(BridgeConfig::default());
        Self {
            bridge,
            session: None,
            generation: 0,
            next_order: 1,
            ledger: Vec::new(),
            preview_cache: Vec::new(),
            validation_history: Vec::new(),
            workspace: WorkspaceState {
                focused_view: "viewport".to_string(),
                open_views: vec![
                    "viewport".to_string(),
                    "outliner".to_string(),
                    "inspector".to_string(),
                ],
            },
            proposals: Vec::new(),
            next_proposal_id: 1,
            assistant_evidence: Vec::new(),
        }
    }

    pub fn with_bridge(bridge: BridgeRuntime) -> Self {
        Self {
            bridge,
            session: None,
            generation: 0,
            next_order: 1,
            ledger: Vec::new(),
            preview_cache: Vec::new(),
            validation_history: Vec::new(),
            workspace: WorkspaceState {
                focused_view: "viewport".to_string(),
                open_views: vec![
                    "viewport".to_string(),
                    "outliner".to_string(),
                    "inspector".to_string(),
                ],
            },
            proposals: Vec::new(),
            next_proposal_id: 1,
            assistant_evidence: Vec::new(),
        }
    }

    pub fn create_object(
        &mut self,
        label: impl Into<String>,
        class: ObjectClass,
    ) -> Result<ObjectHandle, String> {
        let session = self.ensure_session();
        let handle = self
            .bridge
            .register_object_in_session(session, class, label)?;
        self.generation = self.generation.saturating_add(1);
        Ok(handle)
    }

    pub fn snapshot(&self) -> ToolSnapshot {
        ToolSnapshot {
            generation: self.generation,
            objects: self.current_objects(),
            transactions: self.ledger.clone(),
        }
    }

    pub fn index(&self) -> ToolingIndex {
        let objects = self.current_objects();
        let mut by_tag = BTreeMap::<String, Vec<ObjectHandle>>::new();
        for object in &objects {
            for tag in &object.tags {
                by_tag.entry(tag.clone()).or_default().push(object.handle);
            }
        }

        let snapshot = self.snapshot();
        let summary = DerivedSummary::from_snapshot(&snapshot);

        ToolingIndex {
            objects,
            summary,
            by_tag,
        }
    }

    pub fn ledger(&self) -> &[ToolTransaction] {
        &self.ledger
    }

    pub fn preview_cache(&self) -> Vec<PreviewResult> {
        self.preview_cache.clone()
    }

    pub fn validation_history(&self) -> &[Vec<ValidationDiagnostic>] {
        &self.validation_history
    }

    pub fn workspace(&self) -> &WorkspaceState {
        &self.workspace
    }

    pub fn set_workspace_focus(&mut self, focused: impl Into<String>) {
        let focused = focused.into();
        self.workspace.focused_view = focused.clone();
        if !self.workspace.open_views.contains(&focused) {
            self.workspace.open_views.push(focused);
        }
    }

    pub fn clear_workspace(&mut self) {
        self.workspace.focused_view.clear();
        self.workspace.open_views.clear();
    }

    pub fn plan_goal(&self, goal: impl Into<String>) -> AssistantPlan {
        let goal = goal.into();
        AssistantPlan {
            goal: goal.clone(),
            steps: vec!["Step 1".to_string(), "Step 2".to_string()],
            suggested_commands: vec![
                ToolCommand::CreateObject {
                    label: format!("world-{goal}"),
                    class: ObjectClass::World,
                },
                ToolCommand::CreateObject {
                    label: format!("scene-{goal}"),
                    class: ObjectClass::Scene,
                },
                ToolCommand::CreateObject {
                    label: format!("logic-{goal}"),
                    class: ObjectClass::Logic,
                },
            ],
        }
    }

    pub fn assistant_evidence(&self) -> &[AssistantEvidence] {
        &self.assistant_evidence
    }

    fn ensure_session(&mut self) -> SessionHandle {
        if let Some(session) = self.session {
            return session;
        }

        let session = self
            .bridge
            .sessions
            .keys()
            .next_back()
            .copied()
            .unwrap_or_else(|| self.bridge.open_session("tooling_session"));
        self.session = Some(session);
        session
    }

    fn current_objects(&self) -> Vec<ToolObject> {
        self.bridge
            .objects
            .values()
            .map(|obj| ToolObject {
                handle: obj.handle,
                class: obj.class,
                label: obj.label.clone(),
                fields: obj.fields.clone(),
                tags: obj.tags.clone(),
            })
            .collect()
    }
}

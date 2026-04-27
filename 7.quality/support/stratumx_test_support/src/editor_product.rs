use crate::bridge_types::*;
use crate::editor_types::*;
use crate::tooling_runtime_core::ToolingRuntime;
use crate::tooling_types::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct EditorProduct {
    pub title: String,
    pub project_name: String,
    pub viewport: ViewportState,
    pub outliner: OutlinerState,
    pub inspector: InspectorState,
    pub tool_context: ToolContextState,
    pub workspace_layout: WorkspaceLayoutState,
    pub interaction_routing: InteractionRoutingState,
    pub overlay_and_gizmo: OverlayAndGizmoState,
    pub assistant_surface: AssistantSurfaceState,
    pub diagnostics_surface: DiagnosticsSurfaceState,
    pub build_suite: BuildSuiteState,
    pub build_release_surface: BuildReleaseSurfaceState,
    pub production_surface: ProductionSurfaceState,
    pub content_browser: ContentBrowserState,
    pub playtest_surface: PlaytestSurfaceState,
    pub project_bootstrap_service: ProjectBootstrapService,
    pub plugin_host: PluginHostState,
    pub template_service: TemplateServiceState,
    pub world_suite: WorldSuiteState,
    pub scene_suite: SceneSuiteState,
    pub material_suite: MaterialSuiteState,
    pub quest_suite: QuestSuiteState,
    pub simulation_suite: SimulationSuiteState,
    pub tooling: ToolingRuntime,
}

impl EditorProduct {
    pub fn new_demo(project_name: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let project_name = project_name.into();
        let mut product = Self {
            title: "StratumX Editor".to_string(),
            project_name: project_name.clone(),
            viewport: ViewportState::default(),
            outliner: OutlinerState::default(),
            inspector: InspectorState::default(),
            tool_context: ToolContextState::default(),
            workspace_layout: WorkspaceLayoutState {
                layout_name: "default".to_string(),
                open_panels: vec![EditorPanel::Viewport, EditorPanel::Outliner],
            },
            interaction_routing: InteractionRoutingState::default(),
            overlay_and_gizmo: OverlayAndGizmoState {
                gizmo_visible: true,
                gizmos_enabled: true,
                overlay_labels: vec!["grid".to_string(), "selection".to_string()],
            },
            assistant_surface: AssistantSurfaceState::default(),
            diagnostics_surface: DiagnosticsSurfaceState {
                error_count: 1,
                diagnostics: vec!["demo diagnostics seeded".to_string()],
            },
            build_suite: BuildSuiteState {
                last_build_success: true,
                validation_runs: 1,
            },
            build_release_surface: BuildReleaseSurfaceState::default(),
            production_surface: ProductionSurfaceState::default(),
            content_browser: ContentBrowserState::default(),
            playtest_surface: PlaytestSurfaceState::default(),
            project_bootstrap_service: ProjectBootstrapService {
                active_project: project_name,
            },
            plugin_host: PluginHostState {
                loaded_plugins: vec!["core.render".to_string(), "core.authoring".to_string()],
                registered_plugins: vec!["core.render".to_string(), "core.authoring".to_string()],
            },
            template_service: TemplateServiceState {
                available_templates: vec!["basic".to_string()],
                template_count: 1,
            },
            world_suite: WorldSuiteState::default(),
            scene_suite: SceneSuiteState::default(),
            material_suite: MaterialSuiteState::default(),
            quest_suite: QuestSuiteState::default(),
            simulation_suite: SimulationSuiteState::default(),
            tooling: ToolingRuntime::new(),
        };
        product.production_surface.counters.insert(
            "diagnostics".to_string(),
            product.diagnostics_surface.diagnostics.len(),
        );

        let seed = product.create_world_anchor("world-root")?;
        product.select_object(seed)?;
        Ok(product)
    }

    pub fn anchored_panels(&self) -> BTreeSet<EditorPanel> {
        BTreeSet::from([
            EditorPanel::Viewport,
            EditorPanel::Outliner,
            EditorPanel::BuildRelease,
        ])
    }

    pub fn toggle_panel(&mut self, panel: EditorPanel) {
        if !self.workspace_layout.open_panels.contains(&panel) {
            self.workspace_layout.open_panels.push(panel);
        }
    }

    pub fn set_tool_mode(&mut self, mode: ToolMode) {
        self.tool_context.active_mode = mode;
        self.tool_context
            .context_stack
            .push(format!("tool:{mode:?}"));
        self.interaction_routing.active_tool = Some(format!("{mode:?}"));
    }

    pub fn create_world_anchor(
        &mut self,
        label: impl Into<String>,
    ) -> Result<ObjectHandle, String> {
        let label = label.into();
        let handle = self
            .tooling
            .create_object(label.clone(), ObjectClass::World)?;
        self.world_suite.world_roots.push(handle);
        self.world_suite.world_count = self.world_suite.world_roots.len();
        self.outliner.items.push((handle, label, None));
        Ok(handle)
    }

    pub fn create_scene_entity(
        &mut self,
        label: impl Into<String>,
    ) -> Result<ObjectHandle, String> {
        let label = label.into();
        let handle = self
            .tooling
            .create_object(label.clone(), ObjectClass::Entity)?;
        self.scene_suite.scene_entities.push(handle);
        self.scene_suite.scene_count = self.scene_suite.scene_entities.len();
        self.outliner.items.push((handle, label, None));
        Ok(handle)
    }

    pub fn create_material(&mut self, label: impl Into<String>) -> Result<ObjectHandle, String> {
        let label = label.into();
        let handle = self
            .tooling
            .create_object(label.clone(), ObjectClass::Material)?;
        self.material_suite.materials.push(handle);
        self.material_suite.material_count = self.material_suite.materials.len();
        self.content_browser.visible_assets.push(label);
        Ok(handle)
    }

    pub fn create_logic_node(&mut self, label: impl Into<String>) -> Result<ObjectHandle, String> {
        let label = label.into();
        let handle = self
            .tooling
            .create_object(label.clone(), ObjectClass::Logic)?;
        self.simulation_suite.logic_nodes.push(handle);
        self.simulation_suite.simulation_active = true;
        self.quest_suite.quest_nodes.push(handle);
        self.quest_suite.quest_count = self.quest_suite.quest_nodes.len();
        self.outliner.items.push((handle, label, None));
        Ok(handle)
    }

    pub fn select_object(&mut self, handle: ObjectHandle) -> Result<(), String> {
        self.tooling.apply_command(
            ToolCommand::SelectObject { handle },
            CommandOrigin::User,
            ApprovalClass::None,
            BudgetClass::Interactive,
        )?;
        self.viewport.selected = vec![handle];
        self.inspector.selected = Some(handle);
        self.inspector.inspected = Some(handle);
        self.inspector.label = self
            .tooling
            .snapshot()
            .objects
            .into_iter()
            .find(|object| object.handle == handle)
            .map(|object| object.label);
        Ok(())
    }

    pub fn stage_assistant_goal(&mut self, goal: impl Into<String>) -> u64 {
        let goal = goal.into();
        self.assistant_surface.active_goal = Some(goal.clone());
        let plan = self.tooling.plan_goal(&goal);
        let proposal = self.tooling.stage_proposal(goal, plan.suggested_commands);
        self.assistant_surface.staged_proposal = Some(proposal);
        proposal
    }

    pub fn approve_and_apply_assistant(&mut self) -> Result<(), String> {
        let proposal_id = {
            let proposals = &self.tooling.proposals;
            proposals.last().map(|p| p.id)
        };

        if let Some(id) = proposal_id {
            self.tooling.approve_proposal(id)?;
            self.tooling.apply_proposal(id)?;
            self.assistant_surface.staged_proposal = Some(id);
        }
        Ok(())
    }

    pub fn assistant_evidence(&self) -> &[AssistantEvidence] {
        self.tooling.assistant_evidence()
    }

    pub fn run_build(&mut self) -> Result<BuildArtifact, String> {
        let build = self.tooling.build_current();
        self.build_release_surface.last_build = Some(build.clone());
        self.build_suite.last_build_success = true;
        self.build_suite.validation_runs = self.build_suite.validation_runs.saturating_add(1);
        Ok(build)
    }

    pub fn run_release(&mut self, channel: impl Into<String>) -> Result<ReleasePackage, String> {
        let channel = channel.into();
        let build = self.tooling.build_current();
        self.build_release_surface.last_build = Some(build.clone());
        let release = self.tooling.release_build(&build, channel.clone(), true)?;
        self.build_release_surface.release_channel = channel;
        self.build_release_surface.last_release = Some(release.clone());
        Ok(release)
    }

    pub fn run_playtest_capture(&mut self, label: impl Into<String>) {
        self.playtest_surface.session_active = true;
        self.playtest_surface.last_capture_label = Some(label.into());
    }

    pub fn preview_selected(&mut self) -> Result<Option<PreviewResult>, String> {
        if let Some(&handle) = self.viewport.selected.first() {
            Ok(Some(self.tooling.preview_object(handle)?))
        } else {
            Ok(None)
        }
    }

    pub fn rebuild_from_document_objects(
        &mut self,
        _objects: Vec<ObjectHandle>,
        _selected_old: Option<ObjectHandle>,
    ) -> Result<(), String> {
        Ok(())
    }
}

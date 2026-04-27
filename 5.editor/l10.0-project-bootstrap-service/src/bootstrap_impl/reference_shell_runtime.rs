use crate::*;

impl EditorProduct {
    pub(crate) fn reference_shell(
        project_name: String,
        tooling: ToolingRuntime,
        world: ObjectHandle,
        scene: ObjectHandle,
        material: ObjectHandle,
        logic: ObjectHandle,
    ) -> Self {
        Self {
            title: "StratumX Editor".into(),
            project_name: project_name.clone(),
            tooling,
            viewport: ViewportSystem {
                active_viewport: 0,
                projection: ViewportProjection::Perspective,
                selected: vec![world],
                camera_bookmark: "origin".into(),
            },
            outliner: OutlinerSystem {
                items: Vec::new(),
                expanded_labels: BTreeSet::new(),
            },
            content_browser: ContentBrowserSystem {
                visible_assets: Vec::new(),
                active_path: "/Game".into(),
                search_query: String::new(),
            },
            inspector: InspectorSystem {
                selected: Some(world),
                label: None,
                fields: Vec::new(),
            },
            tool_context: ToolContextSystem {
                active_mode: ToolMode::Select,
                active_suite: "world".into(),
            },
            overlay_and_gizmo: OverlayAndGizmoSystem {
                gizmos_enabled: true,
                overlay_labels: vec!["selection".into(), "budget".into()],
            },
            workspace_layout: WorkspaceLayoutSystem {
                open_panels: anchored_panels(),
                focused_panel: EditorPanel::Viewport,
            },
            interaction_routing: InteractionRoutingSystem {
                command_palette_open: false,
                pointer_captured: false,
            },
            assistant_surface: AssistantSurface {
                goal_draft: String::new(),
                staged_proposal: None,
            },
            diagnostics_surface: DiagnosticsSurface {
                diagnostics: Vec::new(),
            },
            build_release_surface: BuildReleaseSurface {
                last_build: None,
                last_release: None,
            },
            world_suite: WorldAuthoringSuite {
                world_roots: vec![world],
            },
            scene_suite: SceneEntityAuthoringSuite {
                scene_entities: vec![scene],
            },
            terrain_suite: TerrainLandscapeAuthoringSuite {
                terrains: Vec::new(),
            },
            material_suite: MaterialLookdevAuthoringSuite {
                materials: vec![material],
            },
            destruction_suite: DestructionFractureAuthoringSuite {
                fracture_targets: Vec::new(),
            },
            simulation_suite: SimulationAiAuthoringSuite {
                logic_nodes: vec![logic],
            },
            weather_suite: WeatherEnvironmentAuthoringSuite {
                environment_nodes: Vec::new(),
            },
            animation_suite: AnimationCinematicsAuthoringSuite {
                timelines: Vec::new(),
            },
            audio_suite: AudioVoiceAuthoringSuite {
                audio_nodes: Vec::new(),
            },
            ui_suite: UiHudAuthoringSuite {
                widgets: Vec::new(),
            },
            quest_suite: QuestEventLogicAuthoringSuite {
                quest_nodes: vec![logic],
            },
            build_suite: BuildValidationReleaseSuite { validation_runs: 0 },
            project_bootstrap_service: ProjectBootstrapService {
                active_project: project_name,
            },
            import_export_service: ImportExportPipelineService {
                import_queue_depth: 0,
            },
            graph_authoring_service: GraphAuthoringService { graph_count: 1 },
            automation_service: AutomationAndBatchService { active_batches: 0 },
            script_service: ScriptAndHotReloadService { reload_count: 0 },
            plugin_host: PluginAndExtensionHost {
                registered_plugins: vec!["core.viewport".into(), "core.asset".into()],
            },
            template_service: TemplatePresetAndScaffoldService { template_count: 3 },
            package_service: PackageMarketAndDependencyService {
                installed_packages: vec!["stratumx.core".into()],
            },
            collaboration_surface: CollaborationSessionSurface {
                reviewers: vec!["local-user".into()],
            },
            review_surface: ReviewAnnotationSurface { notes: Vec::new() },
            approval_surface: AssetGateAndApprovalSurface {
                pending_assets: Vec::new(),
            },
            playtest_surface: PlaytestAndCaptureOperations {
                last_capture_label: None,
            },
            production_surface: ProductionDashboardAndTraceability {
                counters: BTreeMap::new(),
            },
            learning_surface: LearningOnboardingAndHelpSurface {
                quick_start_steps: vec![
                    "Open a project".into(),
                    "Select a world root".into(),
                    "Run validation".into(),
                    "Build a package".into(),
                ],
            },
        }
    }
}

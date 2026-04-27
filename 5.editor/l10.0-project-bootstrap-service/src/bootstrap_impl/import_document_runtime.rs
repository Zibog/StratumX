use crate::*;

impl EditorProduct {
    pub fn load_project_from_path(path: impl AsRef<Path>) -> Result<Self, String> {
        let data =
            fs::read_to_string(path.as_ref()).map_err(|err: std::io::Error| err.to_string())?;
        let document: EditorProjectDocument =
            serde_json::from_str(&data).map_err(|err: serde_json::Error| err.to_string())?;
        Self::from_document(document)
    }
    pub fn from_document(document: EditorProjectDocument) -> Result<Self, String> {
        // Create product with default values
        let mut product = Self {
            title: document.title,
            project_name: document.project_name.clone(),
            tooling: ToolingRuntime::new(),
            viewport: document.viewport,
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
                selected: None,
                label: None,
                fields: Vec::new(),
            },
            tool_context: document.tool_context,
            overlay_and_gizmo: OverlayAndGizmoSystem {
                gizmos_enabled: true,
                overlay_labels: vec!["selection".into(), "budget".into()],
            },
            workspace_layout: document.workspace_layout,
            interaction_routing: document.interaction_routing,
            assistant_surface: document.assistant_surface,
            diagnostics_surface: DiagnosticsSurface {
                diagnostics: Vec::new(),
            },
            build_release_surface: BuildReleaseSurface {
                last_build: None,
                last_release: None,
            },
            world_suite: WorldAuthoringSuite {
                world_roots: Vec::new(),
            },
            scene_suite: SceneEntityAuthoringSuite {
                scene_entities: Vec::new(),
            },
            terrain_suite: TerrainLandscapeAuthoringSuite {
                terrains: Vec::new(),
            },
            material_suite: MaterialLookdevAuthoringSuite {
                materials: Vec::new(),
            },
            destruction_suite: DestructionFractureAuthoringSuite {
                fracture_targets: Vec::new(),
            },
            simulation_suite: SimulationAiAuthoringSuite {
                logic_nodes: Vec::new(),
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
                quest_nodes: Vec::new(),
            },
            build_suite: BuildValidationReleaseSuite { validation_runs: 0 },
            project_bootstrap_service: ProjectBootstrapService {
                active_project: document.project_name,
            },
            import_export_service: ImportExportPipelineService {
                import_queue_depth: 0,
            },
            graph_authoring_service: GraphAuthoringService { graph_count: 1 },
            automation_service: AutomationAndBatchService { active_batches: 0 },
            script_service: ScriptAndHotReloadService { reload_count: 0 },
            plugin_host: document.plugin_host,
            template_service: TemplatePresetAndScaffoldService { template_count: 3 },
            package_service: document.package_service,
            collaboration_surface: document.collaboration_surface,
            review_surface: document.review_surface,
            approval_surface: document.approval_surface,
            playtest_surface: document.playtest_surface,
            production_surface: document.production_surface,
            learning_surface: document.learning_surface,
        };

        // Rebuild from document objects using existing method
        product.rebuild_from_document_objects(document.objects, None)?;

        Ok(product)
    }
}

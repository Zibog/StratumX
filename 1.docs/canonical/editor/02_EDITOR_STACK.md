# Editor Stack

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Layer map
### `L8` Editor product shell and interaction layer
- `L8.0  editor_shell`
- `L8.1  viewport_system`
- `L8.2  outliner_system`
- `L8.3  content_browser_system`
- `L8.4  inspector_system`
- `L8.5  tool_context_system`
- `L8.6  overlay_and_gizmo_system`
- `L8.7  workspace_layout_system`
- `L8.8  interaction_routing_system`
- `L8.9  assistant_surface`
- `L8.10 diagnostics_surface`
- `L8.11 build_release_surface`

### `L9` Domain authoring suites
- `L9.0  world_authoring_suite`
- `L9.1  scene_entity_authoring_suite`
- `L9.2  terrain_landscape_authoring_suite`
- `L9.3  material_lookdev_authoring_suite`
- `L9.4  destruction_fracture_authoring_suite`
- `L9.5  simulation_ai_authoring_suite`
- `L9.6  weather_environment_authoring_suite`
- `L9.7  animation_cinematics_authoring_suite`
- `L9.8  audio_voice_authoring_suite`
- `L9.9  ui_hud_authoring_suite`
- `L9.10 quest_event_logic_authoring_suite`
- `L9.11 build_validation_release_suite`

### `L10` Editor services and extensibility
- `L10.0 project_bootstrap_service`
- `L10.1 import_export_pipeline_service`
- `L10.2 graph_authoring_service`
- `L10.3 automation_and_batch_service`
- `L10.4 script_and_hot_reload_service`
- `L10.5 plugin_and_extension_host`
- `L10.6 template_preset_and_scaffold_service`
- `L10.7 package_market_and_dependency_service`

### `L11` Collaboration, review, and production operations
- `L11.0 collaboration_session_surface`
- `L11.1 review_annotation_surface`
- `L11.2 asset_gate_and_approval_surface`
- `L11.3 playtest_and_capture_operations`
- `L11.4 production_dashboard_and_traceability`
- `L11.5 learning_onboarding_and_help_surface`

## Structural reading
- `L8` = visible shell, panels, views, command surfaces
- `L9` = concrete authoring suites (world, scene, timeline, build validation)
- `L10` = service runtime and extensibility hosts
- `L11` = collaboration, production, runtime-playtest, and capture operations

## Temperature law
- `L8`: warm while visible; inactive views are cold
- `L9`: warm only for active suites and active tool contexts
- `L10`: cold or batch-activated, except lightweight registries
- `L11`: cold or session-activated

## Hot path law
The editor hot path is limited to:
- visible shell routing
- active viewport and active manipulation context
- focused interaction routing
- active panel view-model refresh
- active suite tool context
Everything else must degrade to warm, background, or cold execution.

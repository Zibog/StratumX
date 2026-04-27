> Phase 4 reconciliation note
>
> `editor/111` is descriptive only.
> `editor/110` plus `7.quality/suites/editor_command_matrix/button_route_coverage.json` are the exact-button authority.
> This file may not introduce any `btn.*` id absent from `editor/110`.

# Editor Menu Tree And Base Command Groups Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the default production menu tree for the editor product surface.
This document exists so that the editor grows as a lawful production instrument rather than an accretion of convenient buttons.

The shell must feel:
- as direct and creator-friendly as a modern timeline-first tool;
- as serious as a full engine workstation;
- material-first and world-first without degenerating into one-game vocabulary.

## Visibility modes
### Base authoring surface
This is the everyday shell.
It must be enough to start a project, author a believable world, inspect it in free camera, save it, validate it, and reach the assistant without entering debug labyrinths.

### Advanced authoring
This is deliberate deeper work that is still production-authoring, not certification hell.
Complex audio authoring, response previews, richer runtime shaping, and extension management live here.

### Lab / review / certification
Trace, compare, capture, certify, diagnostics, performance, regression, release, and deep debug live here.
They remain first-class canon, but they are not promoted as top-level day-zero clutter.

## Top-level menu tree

### Project
- `btn.project.new_project`
- `btn.project.open_project`
- `btn.project.save_project`
- `btn.project.save_project_as`
- human-grade recent-projects, pinned workspaces, and path recovery surfaces are allowed but remain subordinate to `118`

### World
- `btn.world.open_world_package`
- `btn.world.save_world_package`
- `btn.world.validate_world`
- the shell may expose terrain, vegetation, structures, grouped props, and simulation steps as stage-local submenus without creating new top-level truth owners

### Terrain
- `btn.terrain.import_heightmap`
- `btn.terrain.sculpt_primary`
- `btn.terrain.smooth_patch`
- `btn.terrain.flatten_patch`
- `btn.terrain.paint_layer`
- `btn.terrain.paint_biome_mask`
- `btn.terrain.rebuild_chunks`
- `btn.terrain.load_chunks`
- `btn.terrain.save_chunks`

### Materials
- `btn.material.new_profile`
- `btn.material.duplicate_profile`
- `btn.material.assign_archetype`
- `btn.material.bind_surface_family`
- `btn.material.bind_response_profile`
- `btn.material.bind_texture_stack`
- `btn.material.bind_microdetail_profile`
- `btn.material.bind_weather_modulation`
- `btn.material.bind_visual_response_family`
- `btn.material.bind_acoustic_profile`
- `btn.material.bind_light_response`
- `btn.material.inspect_branch_coverage`
- `btn.material.set_cheap_runtime_rung`
- `btn.material.preview_bullet_hit`
- `btn.material.preview_blast`
- `btn.material.preview_wetness`
- `btn.material.preview_burn`
- `btn.material.save_profile_as`
- `btn.material.capture_proof_artifacts`
- `btn.material.review_freeze_blockers`

### Sky / Lighting / Weather
- `btn.sky.bind_sky_profile`
- `btn.sky.set_time_of_day`
- `btn.sky.set_weather_regime`
- `btn.sky.bind_cloud_profile`
- richer weather/front/light panels are lawful when they remain routed through the existing sky/environment shell and corresponding labs

### Audio
- `btn.audio.assign_emitter_class_world_source`
- `btn.audio.bind_zone_profile_world_surface`
- `btn.audio.bind_priority_ducking_policy`
- `btn.audio.preview_audibility_free_camera`
- `btn.audio.preview_obstruction_vs_occlusion`
- `btn.audio.preview_indoor_outdoor_transition`
- `btn.audio.preview_voice_subtitle_legality`
- `btn.audio.inspect_listener_profile`
- `btn.audio.inspect_bus_ducking`

### View
- `btn.view.viewport`
- `btn.view.outliner`
- `btn.view.inspector`
- `btn.view.content_browser`
- `btn.view.material_lab`
- `btn.view.terrain_lab`
- `btn.view.sky_lab`
- viewport-local creation bars, timeline strips, minimaps, and semantic navigators are lawful as shell sub-surfaces under `editor/130–131`

### Window / Workspace
- `btn.window.load_workspace_layout`
- `btn.window.save_workspace_layout`
- `btn.window.reset_workspace_layout`
- `btn.window.detach_panel`
- `btn.workspace.open_stage_world`
- `btn.workspace.open_stage_simulation`
- `btn.workspace.open_stage_capture`

### AI / Extensions
- `btn.assistant.open_dock`
- `btn.assistant.apply_proposal`
- `btn.assistant.revert_last_apply`
- `btn.extensions.open_extension_manager`
- `btn.extensions.install_extension_bundle`
- `btn.extensions.toggle_extension_mount`

## Menu-to-manifest law
No button promoted above may exist only as prose.
Every promoted id must be present in `editor/110` with:
- visibility class;
- dependency gate;
- focus restoration rule;
- shell state publication;
- implementation tail if the action mutates truth.

## Human-grade shell law
The base shell must read as one creator conveyor, not as disconnected technical tribes.
This means:
- World is the entry concept, not raw subsystems.
- Terrain, materials, audio, sky, and viewport remain reachable in one or two moves.
- The right inspector and bottom diagnostics/timeline strip may change content per stage, but not their identity.
- The menu tree must support both novice route memory and expert speed.

## View-surface law
View buttons are not “free”.
Each promoted view button must formalize:
- activate/deactivate surface;
- preserve selection when legal;
- restore previous context when switching back;
- deny unavailable surface when dependency missing;
- publish shell status and active-surface state.

## Demotion law
The following families are forbidden as top-level everyday buttons:
- debug-only simulate toggles;
- compare/capture/certify as always-on primary toolbar clutter;
- recovery buttons on the first row without active failure context;
- engine-ownership controls masquerading as editor shortcuts;
- game-specific jargon that would lock the shell to one title rather than one technology stack.

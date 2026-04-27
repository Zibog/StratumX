# Canonical Family Registry

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

This document is the authoritative registry of editor families for the current package version.
Canonical family identity is defined by this registry and by the local family contracts inside the active editor package.
Physical folder names are packaging labels only.

## L8 families
- `shell_and_view_host_family`
- `viewport_and_manipulation_family`
- `browser_and_inspector_family`
- `command_and_interaction_family`
- `assistant_diagnostics_build_family`

## L9 families
- `world_scene_family`
- `terrain_material_environment_family`
- `destruction_simulation_family`
- `animation_audio_ui_family`
- `quest_event_logic_family`
- `build_validation_release_family`

## L10 families
- `pipeline_and_graph_family`
- `extension_and_automation_family`

## L11 families
- `collaboration_and_production_family`

## First-closure subset
The first product-result closure must activate these families as hot or warm:
- `shell_and_view_host_family`
- `viewport_and_manipulation_family`
- `browser_and_inspector_family`
- `command_and_interaction_family`
- `assistant_diagnostics_build_family`
- `world_scene_family`
- `terrain_material_environment_family`
- `build_validation_release_family`

`animation_audio_ui_family` stays warm until the first spine is honest, then expands through `49_INTERACTION_DRIVEN_MOTION_AND_CONSTRAINT_CANON.md`.

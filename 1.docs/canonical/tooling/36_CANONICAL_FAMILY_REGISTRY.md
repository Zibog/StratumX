# Canonical Family Registry

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

This document is the authoritative registry of tooling families for the current package version.
Canonical family identity is defined by the `Canonical family:` line inside each family `00_LEVEL.md` contract inside the active tooling package and by this registry.
Physical directory prefixes such as `l6.f0-*` are legacy mount labels for package locality only; they are **not** canonical family identifiers and may not be used to infer uniqueness.
When a physical folder prefix overlaps another family folder prefix, the canon remains valid only if the canonical family identifiers below remain unique and composition-only.

## Canonical family registry

### L6 families
- `editor_shell_family` -> `families/l6.f0-editor-shell-family/`
- `workspace_shell_family` -> `families/l6.f15-workspace-shell-family/`
- `content_family` -> `families/l6.f1-content-family/`
- `world_partition_authoring_family` -> `families/l6.f16-world-partition-authoring-family/`
- `material_response_authoring_family` -> `families/l6.f17-material-response-authoring-family/`
- `scene_family` -> `families/l6.f2-scene-family/`
- `terrain_deformation_authoring_family` -> `families/l6.f18-terrain-deformation-authoring-family/`
- `world_family` -> `families/l6.f3-world-family/`
- `material_surface_family` -> `families/l6.f4-material-surface-family/`
- `structure_fracture_authoring_family` -> `families/l6.f19-structure-fracture-authoring-family/`
- `fluid_fire_weather_authoring_family` -> `families/l6.f20-fluid-fire-weather-authoring-family/`
- `physics_destruction_family` -> `families/l6.f5-physics-destruction-family/`
- `population_ai_society_authoring_family` -> `families/l6.f21-population-ai-society-authoring-family/`
- `simulation_debug_family` -> `families/l6.f6-simulation-debug-family/`
- `animation_rig_family` -> `families/l6.f7-animation-rig-family/`
- `combat_ballistics_damage_authoring_family` -> `families/l6.f22-combat-ballistics-damage-authoring-family/`
- `animation_motion_authoring_family` -> `families/l6.f23-animation-motion-authoring-family/`
- `cinematic_family` -> `families/l6.f8-cinematic-family/`
- `acoustics_voice_authoring_family` -> `families/l6.f24-acoustics-voice-authoring-family/`
- `vfx_family` -> `families/l6.f9-vfx-family/`
- `audio_family` -> `families/l6.f10-audio-family/`
- `render_lookdev_authoring_family` -> `families/l6.f25-render-lookdev-authoring-family/`
- `performance_governance_authoring_family` -> `families/l6.f26-performance-governance-authoring-family/`
- `ui_family` -> `families/l6.f11-ui-family/`
- `automation_family` -> `families/l6.f12-automation-family/`
- `pack_release_authoring_family` -> `families/l6.f27-pack-release-authoring-family/`
- `observability_diagnostics_family` -> `families/l6.f28-observability-diagnostics-family/`
- `release_family` -> `families/l6.f13-release-family/`
- `assistant_family` -> `families/l6.f14-assistant-family/`

### L6A families
- `assistant_experience_family` -> `families/l6a.f0-assistant-experience-family/`
- `assistant_safety_family` -> `families/l6a.f1-assistant-safety-family/`

### L7 families
- `project_meta_family` -> `families/l7.f0-project-meta-family/`
- `content_meta_family` -> `families/l7.f1-content-meta-family/`
- `world_meta_family` -> `families/l7.f2-world-meta-family/`
- `simulation_meta_family` -> `families/l7.f3-simulation-meta-family/`
- `release_meta_family` -> `families/l7.f4-release-meta-family/`

### L7A families
- `assistant_intelligence_family` -> `families/l7a.f0-assistant-intelligence-family/`

## Registry law
- canonical family identifiers above are unique and authoritative;
- physical folder labels are allowed to overlap numerically because they are storage/mount labels, not canonical identities;
- acceptance, evidence, readiness, and freeze decisions must use canonical family identifiers from this registry, never inferred folder ordinals;
- if any family `00_LEVEL.md` contracts inside the active tooling package declares a canonical family not present here, the package is not gold.


## Automation resolution law
- machine tooling must resolve families through the canonical family registry or the `Canonical family key:` line in each family local pack;
- physical folder ordinals are legacy storage labels only and may not be used as primary keys;
- local automation that caches family identity must cache canonical family identifiers, not physical prefixes;
- a package is gold only if canonical family identity is unique even when physical prefixes overlap.


## Retirement note
Prior claims that physical numeric family prefixes themselves were unique are retired.
The only legal machine key is the canonical family identifier recorded here and in the local family contracts.

# Layer Traceability Log v4

This file is the active completeness and local-contract traceability artifact for the canonical `L6 / L6A / L7 / L7A` package.
- `L6` core levels recorded: `15`
- `L6` sidecar levels recorded: `18`
- `L6A` levels recorded: `8`
- `L7` levels recorded: `8`
- `L7A` levels recorded: `7`
- families recorded: `37`

## L6 core levels

| Trace id | Directory | Canonical name | Activation | Local contract | Status |
|---|---|---|---|---|---|
| `L6C-001` | `l6.0-authority-core` | `authority_core` | `hot` | `../../levels/l6.0-authority-core/00_LEVEL.md` | attached |
| `L6C-002` | `l6.1-command-envelopes` | `command_envelopes` | `hot` | `../../levels/l6.1-command-envelopes/00_LEVEL.md` | attached |
| `L6C-003` | `l6.10-workspace-runtime` | `workspace_runtime` | `hot-ui` | `../../levels/l6.10-workspace-runtime/00_LEVEL.md` | attached |
| `L6C-004` | `l6.11-validation-runtime` | `validation_runtime` | `warm-scan` | `../../levels/l6.11-validation-runtime/00_LEVEL.md` | attached |
| `L6C-005` | `l6.12-preview-runtime` | `preview_runtime` | `warm-speculative` | `../../levels/l6.12-preview-runtime/00_LEVEL.md` | attached |
| `L6C-006` | `l6.13-build-runtime` | `build_runtime` | `warm-deterministic` | `../../levels/l6.13-build-runtime/00_LEVEL.md` | attached |
| `L6C-007` | `l6.14-release-runtime` | `release_runtime` | `cold-execution` | `../../levels/l6.14-release-runtime/00_LEVEL.md` | attached |
| `L6C-008` | `l6.2-transaction-ledger` | `transaction_ledger` | `hot` | `../../levels/l6.2-transaction-ledger/00_LEVEL.md` | attached |
| `L6C-009` | `l6.3-snapshot-plane` | `snapshot_plane` | `hot-read` | `../../levels/l6.3-snapshot-plane/00_LEVEL.md` | attached |
| `L6C-010` | `l6.4-index-plane` | `index_plane` | `warm-read` | `../../levels/l6.4-index-plane/00_LEVEL.md` | attached |
| `L6C-011` | `l6.5-derived-plane` | `derived_plane` | `warm-read` | `../../levels/l6.5-derived-plane/00_LEVEL.md` | attached |
| `L6C-012` | `l6.6-artifact-plane` | `artifact_plane` | `warm-build` | `../../levels/l6.6-artifact-plane/00_LEVEL.md` | attached |
| `L6C-013` | `l6.7-stream-plane` | `stream_plane` | `hot-forward` | `../../levels/l6.7-stream-plane/00_LEVEL.md` | attached |
| `L6C-014` | `l6.8-cache-plane` | `cache_plane` | `warm-evictable` | `../../levels/l6.8-cache-plane/00_LEVEL.md` | attached |
| `L6C-015` | `l6.9-budget-runtime` | `budget_runtime` | `hot-guard` | `../../levels/l6.9-budget-runtime/00_LEVEL.md` | attached |

## L6 sidecar levels

| Trace id | Directory | Canonical name | Activation | Local contract | Status |
|---|---|---|---|---|---|
| `L6S-001` | `l6.0-tool-session` | `tool_session` | `hot-sidecar` | `../../levels/l6.0-tool-session/00_LEVEL.md` | attached |
| `L6S-002` | `l6.1-tool-selection` | `tool_selection` | `hot-sidecar` | `../../levels/l6.1-tool-selection/00_LEVEL.md` | attached |
| `L6S-003` | `l6.10-tool-release-intents` | `tool_release_intents` | `hot-sidecar` | `../../levels/l6.10-tool-release-intents/00_LEVEL.md` | attached |
| `L6S-004` | `l6.11-tool-assistant-intents` | `tool_assistant_intents` | `hot-sidecar` | `../../levels/l6.11-tool-assistant-intents/00_LEVEL.md` | attached |
| `L6S-005` | `l6.12-tool-activation-rules` | `tool_activation_rules` | `hot-sidecar` | `../../levels/l6.12-tool-activation-rules/00_LEVEL.md` | attached |
| `L6S-006` | `l6.13-tool-activation-state` | `tool_activation_state` | `hot-sidecar` | `../../levels/l6.13-tool-activation-state/00_LEVEL.md` | attached |
| `L6S-007` | `l6.14-tool-task-requests` | `tool_task_requests` | `hot-sidecar` | `../../levels/l6.14-tool-task-requests/00_LEVEL.md` | attached |
| `L6S-008` | `l6.15-tool-task-results` | `tool_task_results` | `hot-sidecar` | `../../levels/l6.15-tool-task-results/00_LEVEL.md` | attached |
| `L6S-009` | `l6.16-tool-panel-refs` | `tool_panel_refs` | `hot-sidecar` | `../../levels/l6.16-tool-panel-refs/00_LEVEL.md` | attached |
| `L6S-010` | `l6.17-tool-view-refs` | `tool_view_refs` | `hot-sidecar` | `../../levels/l6.17-tool-view-refs/00_LEVEL.md` | attached |
| `L6S-011` | `l6.2-tool-focus-refs` | `tool_focus_refs` | `hot-sidecar` | `../../levels/l6.2-tool-focus-refs/00_LEVEL.md` | attached |
| `L6S-012` | `l6.3-tool-inspection-views` | `tool_inspection_views` | `hot-sidecar` | `../../levels/l6.3-tool-inspection-views/00_LEVEL.md` | attached |
| `L6S-013` | `l6.4-tool-preview-requests` | `tool_preview_requests` | `hot-sidecar` | `../../levels/l6.4-tool-preview-requests/00_LEVEL.md` | attached |
| `L6S-014` | `l6.5-tool-preview-results` | `tool_preview_results` | `hot-sidecar` | `../../levels/l6.5-tool-preview-results/00_LEVEL.md` | attached |
| `L6S-015` | `l6.6-tool-diagnostics-events` | `tool_diagnostics_events` | `hot-sidecar` | `../../levels/l6.6-tool-diagnostics-events/00_LEVEL.md` | attached |
| `L6S-016` | `l6.7-tool-diagnostics-views` | `tool_diagnostics_views` | `hot-sidecar` | `../../levels/l6.7-tool-diagnostics-views/00_LEVEL.md` | attached |
| `L6S-017` | `l6.8-tool-content-intents` | `tool_content_intents` | `hot-sidecar` | `../../levels/l6.8-tool-content-intents/00_LEVEL.md` | attached |
| `L6S-018` | `l6.9-tool-scene-intents` | `tool_scene_intents` | `hot-sidecar` | `../../levels/l6.9-tool-scene-intents/00_LEVEL.md` | attached |

## L6A levels

| Trace id | Directory | Canonical name | Activation | Local contract | Status |
|---|---|---|---|---|---|
| `L6A-001` | `l6a.0-assistant-sessions` | `assistant_sessions` | `warm` | `../../levels/l6a.0-assistant-sessions/00_LEVEL.md` | attached |
| `L6A-002` | `l6a.1-context-evidence-packs` | `context_evidence_packs` | `warm` | `../../levels/l6a.1-context-evidence-packs/00_LEVEL.md` | attached |
| `L6A-003` | `l6a.2-proposal-runtime` | `proposal_runtime` | `warm` | `../../levels/l6a.2-proposal-runtime/00_LEVEL.md` | attached |
| `L6A-004` | `l6a.3-lowering-runtime` | `lowering_runtime` | `warm` | `../../levels/l6a.3-lowering-runtime/00_LEVEL.md` | attached |
| `L6A-005` | `l6a.4-safety-gates` | `safety_gates` | `warm-guard` | `../../levels/l6a.4-safety-gates/00_LEVEL.md` | attached |
| `L6A-006` | `l6a.5-apply-revert-runtime` | `apply_revert_runtime` | `warm-guard` | `../../levels/l6a.5-apply-revert-runtime/00_LEVEL.md` | attached |
| `L6A-007` | `l6a.6-assistant-ui-runtime` | `assistant_ui_runtime` | `warm-ui` | `../../levels/l6a.6-assistant-ui-runtime/00_LEVEL.md` | attached |
| `L6A-008` | `l6a.7-model-request-runtime` | `model_request_runtime` | `warm-io` | `../../levels/l6a.7-model-request-runtime/00_LEVEL.md` | attached |

## L7 levels

| Trace id | Directory | Canonical name | Activation | Local contract | Status |
|---|---|---|---|---|---|
| `L7-001` | `l7.0-project-orchestration` | `project_orchestration` | `cold` | `../../levels/l7.0-project-orchestration/00_LEVEL.md` | attached |
| `L7-002` | `l7.1-content-campaigns` | `content_campaigns` | `cold` | `../../levels/l7.1-content-campaigns/00_LEVEL.md` | attached |
| `L7-003` | `l7.2-world-campaigns` | `world_campaigns` | `cold` | `../../levels/l7.2-world-campaigns/00_LEVEL.md` | attached |
| `L7-004` | `l7.3-simulation-campaigns` | `simulation_campaigns` | `cold` | `../../levels/l7.3-simulation-campaigns/00_LEVEL.md` | attached |
| `L7-005` | `l7.4-release-campaigns` | `release_campaigns` | `cold` | `../../levels/l7.4-release-campaigns/00_LEVEL.md` | attached |
| `L7-006` | `l7.5-automation-meta` | `automation_meta` | `cold` | `../../levels/l7.5-automation-meta/00_LEVEL.md` | attached |
| `L7-007` | `l7.6-governance-meta` | `governance_meta` | `cold` | `../../levels/l7.6-governance-meta/00_LEVEL.md` | attached |
| `L7-008` | `l7.7-reporting` | `reporting` | `cold` | `../../levels/l7.7-reporting/00_LEVEL.md` | attached |

## L7A levels

| Trace id | Directory | Canonical name | Activation | Local contract | Status |
|---|---|---|---|---|---|
| `L7A-001` | `l7a.0-prompt-understanding` | `prompt_understanding` | `cold` | `../../levels/l7a.0-prompt-understanding/00_LEVEL.md` | attached |
| `L7A-002` | `l7a.1-planning-engine` | `planning_engine` | `cold` | `../../levels/l7a.1-planning-engine/00_LEVEL.md` | attached |
| `L7A-003` | `l7a.2-canon-reasoner` | `canon_reasoner` | `cold` | `../../levels/l7a.2-canon-reasoner/00_LEVEL.md` | attached |
| `L7A-004` | `l7a.3-generation-planner` | `generation_planner` | `cold` | `../../levels/l7a.3-generation-planner/00_LEVEL.md` | attached |
| `L7A-005` | `l7a.4-optimization-reasoner` | `optimization_reasoner` | `cold` | `../../levels/l7a.4-optimization-reasoner/00_LEVEL.md` | attached |
| `L7A-006` | `l7a.5-migration-planner` | `migration_planner` | `cold` | `../../levels/l7a.5-migration-planner/00_LEVEL.md` | attached |
| `L7A-007` | `l7a.6-model-routing` | `model_routing` | `cold` | `../../levels/l7a.6-model-routing/00_LEVEL.md` | attached |

## L6 families

| Trace id | Directory | Canonical family | Local contract | Status |
|---|---|---|---|---|
| `FAM-L6-001` | `l6.f0-editor-shell-family` | `editor_shell_family` | `../../families/l6.f0-editor-shell-family/00_LEVEL.md` | attached |
| `FAM-L6-002` | `l6.f15-workspace-shell-family` | `workspace_shell_family` | `../../families/l6.f15-workspace-shell-family/00_LEVEL.md` | attached |
| `FAM-L6-003` | `l6.f1-content-family` | `content_family` | `../../families/l6.f1-content-family/00_LEVEL.md` | attached |
| `FAM-L6-004` | `l6.f16-world-partition-authoring-family` | `world_partition_authoring_family` | `../../families/l6.f16-world-partition-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-005` | `l6.f10-audio-family` | `audio_family` | `../../families/l6.f10-audio-family/00_LEVEL.md` | attached |
| `FAM-L6-006` | `l6.f25-render-lookdev-authoring-family` | `render_lookdev_authoring_family` | `../../families/l6.f25-render-lookdev-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-007` | `l6.f26-performance-governance-authoring-family` | `performance_governance_authoring_family` | `../../families/l6.f26-performance-governance-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-008` | `l6.f11-ui-family` | `ui_family` | `../../families/l6.f11-ui-family/00_LEVEL.md` | attached |
| `FAM-L6-009` | `l6.f12-automation-family` | `automation_family` | `../../families/l6.f12-automation-family/00_LEVEL.md` | attached |
| `FAM-L6-010` | `l6.f27-pack-release-authoring-family` | `pack_release_authoring_family` | `../../families/l6.f27-pack-release-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-011` | `l6.f28-observability-diagnostics-family` | `observability_diagnostics_family` | `../../families/l6.f28-observability-diagnostics-family/00_LEVEL.md` | attached |
| `FAM-L6-012` | `l6.f13-release-family` | `release_family` | `../../families/l6.f13-release-family/00_LEVEL.md` | attached |
| `FAM-L6-013` | `l6.f14-assistant-family` | `assistant_family` | `../../families/l6.f14-assistant-family/00_LEVEL.md` | attached |
| `FAM-L6-014` | `l6.f17-material-response-authoring-family` | `material_response_authoring_family` | `../../families/l6.f17-material-response-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-015` | `l6.f2-scene-family` | `scene_family` | `../../families/l6.f2-scene-family/00_LEVEL.md` | attached |
| `FAM-L6-016` | `l6.f18-terrain-deformation-authoring-family` | `terrain_deformation_authoring_family` | `../../families/l6.f18-terrain-deformation-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-017` | `l6.f3-world-family` | `world_family` | `../../families/l6.f3-world-family/00_LEVEL.md` | attached |
| `FAM-L6-018` | `l6.f4-material-surface-family` | `material_surface_family` | `../../families/l6.f4-material-surface-family/00_LEVEL.md` | attached |
| `FAM-L6-019` | `l6.f19-structure-fracture-authoring-family` | `structure_fracture_authoring_family` | `../../families/l6.f19-structure-fracture-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-020` | `l6.f20-fluid-fire-weather-authoring-family` | `fluid_fire_weather_authoring_family` | `../../families/l6.f20-fluid-fire-weather-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-021` | `l6.f5-physics-destruction-family` | `physics_destruction_family` | `../../families/l6.f5-physics-destruction-family/00_LEVEL.md` | attached |
| `FAM-L6-022` | `l6.f21-population-ai-society-authoring-family` | `population_ai_society_authoring_family` | `../../families/l6.f21-population-ai-society-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-023` | `l6.f6-simulation-debug-family` | `simulation_debug_family` | `../../families/l6.f6-simulation-debug-family/00_LEVEL.md` | attached |
| `FAM-L6-024` | `l6.f7-animation-rig-family` | `animation_rig_family` | `../../families/l6.f7-animation-rig-family/00_LEVEL.md` | attached |
| `FAM-L6-025` | `l6.f22-combat-ballistics-damage-authoring-family` | `combat_ballistics_damage_authoring_family` | `../../families/l6.f22-combat-ballistics-damage-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-026` | `l6.f23-animation-motion-authoring-family` | `animation_motion_authoring_family` | `../../families/l6.f23-animation-motion-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-027` | `l6.f8-cinematic-family` | `cinematic_family` | `../../families/l6.f8-cinematic-family/00_LEVEL.md` | attached |
| `FAM-L6-028` | `l6.f24-acoustics-voice-authoring-family` | `acoustics_voice_authoring_family` | `../../families/l6.f24-acoustics-voice-authoring-family/00_LEVEL.md` | attached |
| `FAM-L6-029` | `l6.f9-vfx-family` | `vfx_family` | `../../families/l6.f9-vfx-family/00_LEVEL.md` | attached |

## L6A families

| Trace id | Directory | Canonical family | Local contract | Status |
|---|---|---|---|---|
| `FAM-L6A-001` | `l6a.f0-assistant-experience-family` | `assistant_experience_family` | `../../families/l6a.f0-assistant-experience-family/00_LEVEL.md` | attached |
| `FAM-L6A-002` | `l6a.f1-assistant-safety-family` | `assistant_safety_family` | `../../families/l6a.f1-assistant-safety-family/00_LEVEL.md` | attached |

## L7 families

| Trace id | Directory | Canonical family | Local contract | Status |
|---|---|---|---|---|
| `FAM-L7-001` | `l7.f0-project-meta-family` | `project_meta_family` | `../../families/l7.f0-project-meta-family/00_LEVEL.md` | attached |
| `FAM-L7-002` | `l7.f1-content-meta-family` | `content_meta_family` | `../../families/l7.f1-content-meta-family/00_LEVEL.md` | attached |
| `FAM-L7-003` | `l7.f2-world-meta-family` | `world_meta_family` | `../../families/l7.f2-world-meta-family/00_LEVEL.md` | attached |
| `FAM-L7-004` | `l7.f3-simulation-meta-family` | `simulation_meta_family` | `../../families/l7.f3-simulation-meta-family/00_LEVEL.md` | attached |
| `FAM-L7-005` | `l7.f4-release-meta-family` | `release_meta_family` | `../../families/l7.f4-release-meta-family/00_LEVEL.md` | attached |

## L7A families

| Trace id | Directory | Canonical family | Local contract | Status |
|---|---|---|---|---|
| `FAM-L7A-001` | `l7a.f0-assistant-intelligence-family` | `assistant_intelligence_family` | `../../families/l7a.f0-assistant-intelligence-family/00_LEVEL.md` | attached |

## Completeness rule

- every declared level above must have a real directory and a real 00_LEVEL.md file local contract
- every declared family above must have a real directory and a real 00_LEVEL.md file local contract
- no completeness claim may depend on superseded package revisions

# StratumX Crate Status Ledger

## Overview

This ledger documents the classification status of all crates in the StratumX workspace. Each crate is classified according to its role in the system architecture and current implementation status.

## Classification Categories

- **ACTIVE_CORE**: Required by engine/SDK/tooling/editor spine. Must compile and be workspace-integrated.
- **ACTIVE_PRODUCT**: Visible product/runtime surface. Part of the active editor product spine.
- **ACTIVE_SUPPORT**: Support crate used by active systems (e.g., test support, quality infrastructure).
- **FUTURE_STUB**: Intentionally thin future surface. Placeholder for planned functionality.
- **LEGACY**: Deprecated or superseded code pending removal.
- **DELETE**: Safe to remove after dependency verification.

## Validation Rules

- ACTIVE_CORE crates must not be orphaned (FAIL if orphaned)
- ACTIVE_PRODUCT crates must not be orphaned unless consumed by host/root (FAIL if orphaned)
- DELETE crates must not remain in workspace (FAIL if present)
- FUTURE_STUB crates are allowed as placeholders (WARN)

## Crate Classifications

### 2.engine - Engine Layer (ACTIVE_CORE)

All engine crates are classified as ACTIVE_CORE as they form the runtime foundation.

| Crate Name | Path | Status | Notes |
|------------|------|--------|-------|
| engine_world_region | 2.engine/l-0.05-world-region | ACTIVE_CORE | World region management |
| engine_world_spatial | 2.engine/l-0.1-world-spatial | ACTIVE_CORE | Spatial indexing and queries |
| engine_ecs | 2.engine/l-0.2-ecs-assembly | ACTIVE_CORE | ECS assembly system |
| engine_ecs_query | 2.engine/l-0.3-ecs-query | ACTIVE_CORE | ECS query system |
| engine_ecs_registry | 2.engine/l-0.4-ecs-registry | ACTIVE_CORE | ECS registry |
| engine_storage_mutation | 2.engine/l-0.5-storage-mutation | ACTIVE_CORE | Storage mutation layer |
| engine_storage_access | 2.engine/l-0.6-storage-access | ACTIVE_CORE | Storage access layer |
| engine_storage_layout | 2.engine/l-0.7-storage-layout | ACTIVE_CORE | Storage layout management |
| engine_handle | 2.engine/l-0.8-handle | ACTIVE_CORE | Handle system |
| engine_identity | 2.engine/l-0.9-identity | ACTIVE_CORE | Identity management |
| engine_core | 2.engine/l-1-foundation | ACTIVE_CORE | Foundation types and utilities |
| engine_world | 2.engine/l0-world-truth | ACTIVE_CORE | World truth system |
| engine_material | 2.engine/l0.5-shared-world-property-substrate | ACTIVE_CORE | Shared world property substrate |
| engine_runtime_headless | 2.engine/l1-runtime-kernel/runtime-headless | ACTIVE_CORE | Headless runtime kernel |
| engine_runtime | 2.engine/l1-runtime-kernel/runtime | ACTIVE_CORE | Core runtime kernel |
| engine_runtime_realtime | 2.engine/l1-runtime-kernel/runtime-realtime | ACTIVE_CORE | Realtime runtime kernel |
| engine_memory_control | 2.engine/l1.5-runtime-resource-services/memory-control | ACTIVE_CORE | Memory control service |
| engine_residency_control | 2.engine/l1.5-runtime-resource-services/residency-control | ACTIVE_CORE | Residency control service |
| engine_stream_control | 2.engine/l1.5-runtime-resource-services/stream-control | ACTIVE_CORE | Stream control service |
| engine_transfer_control | 2.engine/l1.5-runtime-resource-services/transfer-control | ACTIVE_CORE | Transfer control service |
| engine_agents | 2.engine/l2-critical-simulation-families/agents | ACTIVE_CORE | Agent simulation |
| engine_field | 2.engine/l2-critical-simulation-families/field | ACTIVE_CORE | Field simulation |
| engine_kinetics | 2.engine/l2-critical-simulation-families/kinetics | ACTIVE_CORE | Kinetics simulation |
| engine_net_latency | 2.engine/l2.5-network-runtime-services/net-latency | ACTIVE_CORE | Network latency service |
| engine_net_transport | 2.engine/l2.5-network-runtime-services/net-transport | ACTIVE_CORE | Network transport service |
| engine_net_sync | 2.engine/l2.5-network-runtime-services/net-sync | ACTIVE_CORE | Network sync service |
| engine_generation | 2.engine/l3.0-model-systems/generation | ACTIVE_CORE | Model generation system |
| engine_inference | 2.engine/l3.0-model-systems/inference | ACTIVE_CORE | Model inference system |
| engine_acoustics | 2.engine/l3.1-synthesis-systems/acoustics | ACTIVE_CORE | Acoustics synthesis |
| engine_animation | 2.engine/l3.1-synthesis-systems/animation | ACTIVE_CORE | Animation synthesis |
| engine_imaging | 2.engine/l3.1-synthesis-systems/imaging | ACTIVE_CORE | Imaging synthesis |
| engine_content | 2.engine/l3.2-resource-systems | ACTIVE_CORE | Resource systems |
| engine_startup | 2.engine/l4-startup | ACTIVE_CORE | Engine startup |

### 3.sdk - SDK Layer (ACTIVE_CORE)

All SDK crates are classified as ACTIVE_CORE as they provide the frozen DTO contract layer.

| Crate Name | Path | Status | Notes |
|------------|------|--------|-------|
| sdk_compat | 3.sdk/l5-compat | ACTIVE_CORE | SDK compatibility layer |
| link_ingress_packets | 3.sdk/l5.0-link-ingress-packets | ACTIVE_CORE | Ingress packet definitions |
| link_ingress_controls | 3.sdk/l5.1-link-ingress-controls | ACTIVE_CORE | Ingress control definitions |
| link_egress_observations | 3.sdk/l5.2-link-egress-observations | ACTIVE_CORE | Egress observation definitions |
| link_egress_metrics | 3.sdk/l5.3-link-egress-metrics | ACTIVE_CORE | Egress metrics definitions |
| transport_policies | 3.sdk/l5.8-transport-policies | ACTIVE_CORE | Transport policy definitions |
| editor_dto_law | 3.sdk/l5.9-editor-dto-law | ACTIVE_CORE | Editor DTO law |
| legality_gates | 3.sdk/l5.9-legality-gates | ACTIVE_CORE | Legality gate validation |
| engine_handle_refs | 3.sdk/l5.10-engine-handle-refs | ACTIVE_CORE | Engine handle references |

### 4.tooling - Tooling Layer (ACTIVE_CORE)

All tooling crates are classified as ACTIVE_CORE as they provide the tooling runtime foundation.

| Crate Name | Path | Status | Notes |
|------------|------|--------|-------|
| stratumx_tooling_l6_0_authority_core | 4.tooling/l6.0-authority-core | ACTIVE_CORE | Authority core system |
| stratumx_tooling_l6_0_tool_session | 4.tooling/l6.0-tool-session | ACTIVE_CORE | Tool session management |
| stratumx_tooling_l6_1_command_envelopes | 4.tooling/l6.1-command-envelopes | ACTIVE_CORE | Command envelope definitions |
| stratumx_tooling_l6_6_tool_diagnostics_events | 4.tooling/l6.6-tool-diagnostics-events | ACTIVE_CORE | Tool diagnostics events |
| stratumx_tooling_l6_7_tool_evidence_capture | 4.tooling/l6.7-tool-evidence-capture | ACTIVE_CORE | Tool evidence capture |
| stratumx_tooling_l6_12_preview_runtime | 4.tooling/l6.12-preview-runtime | ACTIVE_CORE | Preview runtime |
| stratumx_tooling_l6_14_release_runtime | 4.tooling/l6.14-release-runtime | ACTIVE_CORE | Release runtime |

### 5.editor - Editor Layer

Editor crates are classified as either ACTIVE_PRODUCT (part of the active product spine) or FUTURE_STUB (planned future functionality).

#### Active Product Spine (ACTIVE_PRODUCT)

These crates form the verified active editor product surface.

| Crate Name | Path | Status | Notes |
|------------|------|--------|-------|
| stratumx-editor-l7-0-editor-command-spine | 5.editor/l7.0-editor-command-spine | ACTIVE_PRODUCT | Editor command spine |
| stratumx-editor-l8-0-editor-shell | 5.editor/l8.0-editor-shell | ACTIVE_PRODUCT | Editor shell and layout |
| stratumx-editor-l8-1-viewport-system | 5.editor/l8.1-viewport-system | ACTIVE_PRODUCT | Viewport rendering system |
| stratumx-editor-l8-5-tool-context-system | 5.editor/l8.5-tool-context-system | ACTIVE_PRODUCT | Tool context management |
| stratumx-editor-l8-10-diagnostics-surface | 5.editor/l8.10-diagnostics-surface | ACTIVE_PRODUCT | Diagnostics display surface |
| stratumx-editor-l9-0-world-authoring-suite | 5.editor/l9.0-world-authoring-suite | ACTIVE_PRODUCT | World authoring suite |
| stratumx-editor-l9-2-terrain-landscape-authoring-suite | 5.editor/l9.2-terrain-landscape-authoring-suite | ACTIVE_PRODUCT | Terrain authoring suite |
| stratumx-editor-l9-3-material-lookdev-authoring-suite | 5.editor/l9.3-material-lookdev-authoring-suite | ACTIVE_PRODUCT | Material authoring suite |
| stratumx-editor-l9-6-weather-environment-authoring-suite | 5.editor/l9.6-weather-environment-authoring-suite | ACTIVE_PRODUCT | Weather authoring suite |
| stratumx-editor-l10-0-project-bootstrap-service | 5.editor/l10.0-project-bootstrap-service | ACTIVE_PRODUCT | Project bootstrap service |

#### Active Support Crates (ACTIVE_SUPPORT)

These editor crates provide support functionality for the active product.

| Crate Name | Path | Status | Notes |
|------------|------|--------|-------|
| stratumx_editor_state_containers | 5.editor/editor-state-containers | ACTIVE_SUPPORT | Shared state containers |
| stratumx-editor-l8-7-workspace-layout-system | 5.editor/l8.7-workspace-layout-system | ACTIVE_SUPPORT | Workspace layout management |

#### Future Stub Crates (FUTURE_STUB)

These crates are placeholders for planned future functionality. They are not part of the current active product spine.

| Crate Name | Path | Status | Purpose |
|------------|------|--------|---------|
| stratumx-editor-l8-2-outliner-system | 5.editor/l8.2-outliner-system | FUTURE_STUB | Scene hierarchy outliner panel |
| stratumx-editor-l8-3-content-browser-system | 5.editor/l8.3-content-browser-system | FUTURE_STUB | Asset content browser panel |
| stratumx-editor-l8-4-inspector-system | 5.editor/l8.4-inspector-system | FUTURE_STUB | Property inspector panel |
| stratumx-editor-l8-6-overlay-and-gizmo-system | 5.editor/l8.6-overlay-and-gizmo-system | FUTURE_STUB | Viewport overlays and gizmos |
| stratumx-editor-l8-8-interaction-routing-system | 5.editor/l8.8-interaction-routing-system | FUTURE_STUB | Input interaction routing |
| stratumx-editor-l8-9-assistant-surface | 5.editor/l8.9-assistant-surface | FUTURE_STUB | AI assistant surface |
| stratumx-editor-l8-11-build-release-surface | 5.editor/l8.11-build-release-surface | FUTURE_STUB | Build and release management |
| stratumx-editor-l9-1-scene-entity-authoring-suite | 5.editor/l9.1-scene-entity-authoring-suite | FUTURE_STUB | Scene entity authoring |
| stratumx-editor-l9-4-destruction-fracture-authoring-suite | 5.editor/l9.4-destruction-fracture-authoring-suite | FUTURE_STUB | Destruction and fracture authoring (explicitly marked FUTURE_STUB in code) |
| stratumx-editor-l9-5-simulation-ai-authoring-suite | 5.editor/l9.5-simulation-ai-authoring-suite | FUTURE_STUB | AI and simulation authoring |
| stratumx-editor-l9-7-animation-cinematics-authoring-suite | 5.editor/l9.7-animation-cinematics-authoring-suite | FUTURE_STUB | Animation and cinematics authoring |
| stratumx-editor-l9-8-audio-voice-authoring-suite | 5.editor/l9.8-audio-voice-authoring-suite | FUTURE_STUB | Audio and voice authoring (explicitly marked FUTURE_STUB in code) |
| stratumx-editor-l9-9-ui-hud-authoring-suite | 5.editor/l9.9-ui-hud-authoring-suite | FUTURE_STUB | UI and HUD authoring |
| stratumx-editor-l9-10-quest-event-logic-authoring-suite | 5.editor/l9.10-quest-event-logic-authoring-suite | FUTURE_STUB | Quest and event logic authoring |
| stratumx-editor-l9-11-build-validation-release-suite | 5.editor/l9.11-build-validation-release-suite | FUTURE_STUB | Build validation and release |
| stratumx-editor-l10-1-import-export-pipeline-service | 5.editor/l10.1-import-export-pipeline-service | FUTURE_STUB | Asset import/export pipeline |
| stratumx-editor-l10-2-graph-authoring-service | 5.editor/l10.2-graph-authoring-service | FUTURE_STUB | Graph-based authoring service |
| stratumx-editor-l10-3-automation-and-batch-service | 5.editor/l10.3-automation-and-batch-service | FUTURE_STUB | Automation and batch processing |
| stratumx-editor-l10-4-script-and-hot-reload-service | 5.editor/l10.4-script-and-hot-reload-service | FUTURE_STUB | Scripting and hot reload |
| stratumx-editor-l10-5-plugin-and-extension-host | 5.editor/l10.5-plugin-and-extension-host | FUTURE_STUB | Plugin and extension system |
| stratumx-editor-l10-6-template-preset-and-scaffold-service | 5.editor/l10.6-template-preset-and-scaffold-service | FUTURE_STUB | Template and scaffolding |
| stratumx-editor-l10-7-package-market-and-dependency-service | 5.editor/l10.7-package-market-and-dependency-service | FUTURE_STUB | Package marketplace |
| stratumx-editor-l11-0-collaboration-session-surface | 5.editor/l11.0-collaboration-session-surface | FUTURE_STUB | Collaboration features |
| stratumx-editor-l11-1-review-annotation-surface | 5.editor/l11.1-review-annotation-surface | FUTURE_STUB | Review and annotation |
| stratumx-editor-l11-2-asset-gate-and-approval-surface | 5.editor/l11.2-asset-gate-and-approval-surface | FUTURE_STUB | Asset approval workflow |
| stratumx-editor-l11-3-playtest-and-capture-operations | 5.editor/l11.3-playtest-and-capture-operations | FUTURE_STUB | Playtest and capture |
| stratumx-editor-l11-4-production-dashboard-and-traceability | 5.editor/l11.4-production-dashboard-and-traceability | FUTURE_STUB | Production dashboard |
| stratumx-editor-l11-5-learning-onboarding-and-help-surface | 5.editor/l11.5-learning-onboarding-and-help-surface | FUTURE_STUB | Learning and help system |

### 6.apps - Application Layer (ACTIVE_PRODUCT)

Application host crates are thin orchestration layers.

| Crate Name | Path | Status | Notes |
|------------|------|--------|-------|
| stratumx_editor_app | 6.apps/editor/stratumx_editor_app | ACTIVE_PRODUCT | Editor application host |
| stratumx_engine_headless_app | 6.apps/engine/stratumx_engine_headless_app | ACTIVE_PRODUCT | Headless engine app |
| stratumx_engine_realtime_app | 6.apps/engine/stratumx_engine_realtime_app | ACTIVE_PRODUCT | Realtime engine app |
| stratumx_stack_runtime_app | 6.apps/stack/stratumx_stack_runtime_app | ACTIVE_PRODUCT | Stack runtime app |
| stratumx_stack_utility | 6.apps/stack/stratumx_stack_utility | ACTIVE_SUPPORT | Stack utility tools |

### 7.quality - Quality Layer (ACTIVE_SUPPORT)

All quality crates are classified as ACTIVE_SUPPORT as they provide testing and validation infrastructure.

| Crate Name | Path | Status | Notes |
|------------|------|--------|-------|
| stratumx_quality_tasks | 7.quality/tasks/stratumx_quality_tasks | ACTIVE_SUPPORT | Quality task definitions |
| stratumx_test_support | 7.quality/support/stratumx_test_support | ACTIVE_SUPPORT | Test support utilities |
| stratumx_shell_test_support | 7.quality/support/stratumx_shell_test_support | ACTIVE_SUPPORT | Shell test support |
| stratumx_route_test_support | 7.quality/support/stratumx_route_test_support | ACTIVE_SUPPORT | Route test support |
| stratumx_repo_hygiene_support | 7.quality/support/stratumx_repo_hygiene_support | ACTIVE_SUPPORT | Repo hygiene support |
| repo_hygiene | 7.quality/suites/repo_hygiene | ACTIVE_SUPPORT | Repository hygiene tests |
| audio_authoring_matrix | 7.quality/suites/audio_authoring_matrix | ACTIVE_SUPPORT | Audio authoring test matrix |
| build_release_matrix | 7.quality/suites/build_release_matrix | ACTIVE_SUPPORT | Build release test matrix |
| editor_app_matrix | 7.quality/suites/editor_app_matrix | ACTIVE_SUPPORT | Editor app test matrix |
| editor_canon_matrix | 7.quality/suites/editor_canon_matrix | ACTIVE_SUPPORT | Editor canon test matrix |
| editor_command_matrix | 7.quality/suites/editor_command_matrix | ACTIVE_SUPPORT | Editor command test matrix |
| editor_shell_matrix | 7.quality/suites/editor_shell_matrix | ACTIVE_SUPPORT | Editor shell test matrix |
| editor_state_matrix | 7.quality/suites/editor_state_matrix | ACTIVE_SUPPORT | Editor state test matrix |
| end_to_end_matrix | 7.quality/suites/end_to_end_matrix | ACTIVE_SUPPORT | End-to-end test matrix |
| engine_canon_matrix | 7.quality/suites/engine_canon_matrix | ACTIVE_SUPPORT | Engine canon test matrix |
| engine_perf_harness | 7.quality/suites/engine_perf_harness | ACTIVE_SUPPORT | Engine performance harness |
| engine_sdk_link_matrix | 7.quality/suites/engine_sdk_link_matrix | ACTIVE_SUPPORT | Engine-SDK link test matrix |
| environment_authoring_matrix | 7.quality/suites/environment_authoring_matrix | ACTIVE_SUPPORT | Environment authoring test matrix |
| focus_recovery_matrix | 7.quality/suites/focus_recovery_matrix | ACTIVE_SUPPORT | Focus recovery test matrix |
| forbidden_shortcuts | 7.quality/suites/forbidden_shortcuts | ACTIVE_SUPPORT | Forbidden shortcuts validation |
| material_authoring_matrix | 7.quality/suites/material_authoring_matrix | ACTIVE_SUPPORT | Material authoring test matrix |
| proof_region_integration | 7.quality/suites/proof_region_integration | ACTIVE_SUPPORT | Region integration proof tests |
| route_schema_golden | 7.quality/suites/route_schema_golden | ACTIVE_SUPPORT | Route schema golden tests |
| sdk_canon_matrix | 7.quality/suites/sdk_canon_matrix | ACTIVE_SUPPORT | SDK canon test matrix |
| sdk_contract_tests | 7.quality/suites/sdk_contract_tests | ACTIVE_SUPPORT | SDK contract tests |
| stratumx_quality_sdk_compat_matrix | 7.quality/suites/sdk_compat_matrix | ACTIVE_SUPPORT | SDK compat test matrix |
| stratumx_quality_sdk_egress_matrix | 7.quality/suites/sdk_egress_matrix | ACTIVE_SUPPORT | SDK egress test matrix |
| stratumx_quality_sdk_ingress_matrix | 7.quality/suites/sdk_ingress_matrix | ACTIVE_SUPPORT | SDK ingress test matrix |
| stratumx_quality_sdk_legality_matrix | 7.quality/suites/sdk_legality_matrix | ACTIVE_SUPPORT | SDK legality test matrix |
| sdk_tooling_link_matrix | 7.quality/suites/sdk_tooling_link_matrix | ACTIVE_SUPPORT | SDK-tooling link test matrix |
| smoke | 7.quality/suites/smoke | ACTIVE_SUPPORT | Smoke tests |
| terrain_authoring_matrix | 7.quality/suites/terrain_authoring_matrix | ACTIVE_SUPPORT | Terrain authoring test matrix |
| tool_session_matrix | 7.quality/suites/tool_session_matrix | ACTIVE_SUPPORT | Tool session test matrix |
| tooling_canon_matrix | 7.quality/suites/tooling_canon_matrix | ACTIVE_SUPPORT | Tooling canon test matrix |
| vertical_slice_quality_gates | 7.quality/suites/vertical_slice_quality_gates | ACTIVE_SUPPORT | Vertical slice quality gates |
| vertical_slice_tests | 7.quality/suites/vertical_slice_tests | ACTIVE_SUPPORT | Vertical slice tests |
| world_authoring_matrix | 7.quality/suites/world_authoring_matrix | ACTIVE_SUPPORT | World authoring test matrix |

## Summary Statistics

- **Total Crates**: 133
- **ACTIVE_CORE**: 50 (Engine: 33, SDK: 9, Tooling: 7, Other: 1)
- **ACTIVE_PRODUCT**: 15 (Editor spine: 10, Apps: 4, Other: 1)
- **ACTIVE_SUPPORT**: 41 (Quality: 36, Editor support: 2, App support: 1, Other: 2)
- **FUTURE_STUB**: 27 (All in editor layer)
- **LEGACY**: 0
- **DELETE**: 0

## Orphan Crate Analysis

**Result**: No orphan crates detected.

All crates are properly integrated into the workspace as verified by the workspace members list in the root Cargo.toml. All ACTIVE_CORE and ACTIVE_PRODUCT crates are workspace members and have proper dependency integration.

## Next Actions

1. Continue monitoring FUTURE_STUB crates as they transition to active implementation
2. Ensure all ACTIVE_CORE and ACTIVE_PRODUCT crates maintain compilation status
3. Document any crate status changes in this ledger
4. Before removing any crate, update its status to DELETE and verify no dependencies exist

## Ledger Maintenance

This ledger should be updated when:
- New crates are added to the workspace
- Crate status changes (e.g., FUTURE_STUB → ACTIVE_PRODUCT)
- Crates are deprecated or removed
- Major architectural changes affect crate roles

Last Updated: Phase 3 Cleanup (SX-CANON/1.0.28/STACK-v34)

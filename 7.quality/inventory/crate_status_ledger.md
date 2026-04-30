# StratumX Crate Status Ledger

This ledger classifies active and future crates so placeholder crates do not pretend to be production-complete systems.

Status meanings:
- ACTIVE_CORE: required by engine/sdk/tooling/editor spine now.
- ACTIVE_PRODUCT: visible product/runtime surface used now.
- ACTIVE_SUPPORT: support crate used by active systems.
- FUTURE_STUB: intentionally thin future surface, not product-ready.
- LEGACY: kept only for compatibility/history, not active.
- DELETE: safe to remove after verifying no dependencies.
- `DOCS_ONLY_TARGET` is reserved in phase documentation, but the current workspace gates only parse the six statuses above, so planned editor crates stay classified as `FUTURE_STUB` until the gates grow that state.

## Editor classification ledger

| Crate path | Package role | Status | Product spine? | Reverse deps | Decision | Notes |
|---|---|---|---:|---:|---|---|
| 6.apps/editor/stratumx_editor_app | desktop editor host | ACTIVE_PRODUCT | yes | 2 | keep | Runnable thin host; wires editor crates but must not own editor truth. |
| 5.editor/l8.0-editor-shell | shell product surface | ACTIVE_PRODUCT | yes | 4 | keep | Live shell surface used by the desktop host today. |
| 5.editor/l7.0-editor-command-spine | command routing spine | ACTIVE_SUPPORT | yes | 3 | keep | Active command routing for the live editor, but not a user-facing surface by itself. |
| 5.editor/l8.1-viewport-system | viewport support | ACTIVE_SUPPORT | yes | 1 | keep | Imported by the app, but the current host still renders a shell-level placeholder viewport. |
| 5.editor/l8.10-diagnostics-surface | diagnostics support | ACTIVE_SUPPORT | yes | 3 | split | Live diagnostics types/state, while host keeps the thin diagnostics presentation glue. |
| 5.editor/l9.0-world-authoring-suite | world lifecycle support | ACTIVE_SUPPORT | yes | 2 | split | Live world/session support, not a product-complete authoring surface. |
| 5.editor/l9.2-terrain-landscape-authoring-suite | terrain authoring support | ACTIVE_SUPPORT | yes | 3 | split | Live terrain support behind host-owned panel glue; the crate desktop panel is not the active product path. |
| 5.editor/l9.6-weather-environment-authoring-suite | weather authoring support | ACTIVE_SUPPORT | yes | 1 | split | Live environment support behind host-owned controls; not a product-complete suite surface. |
| 5.editor/editor-state-containers | editor state containers | FUTURE_STUB | no | 2 | demote_future | Used by tests and future workspace state experiments, not by the live editor host. |
| 5.editor/l8.2-outliner-system | outliner surface | FUTURE_STUB | no | 0 | demote_future | Present in the workspace, but not wired into the runnable editor. |
| 5.editor/l8.3-content-browser-system | content browser surface | FUTURE_STUB | no | 0 | demote_future | Present in the workspace, but not wired into the runnable editor. |
| 5.editor/l8.4-inspector-system | inspector surface | FUTURE_STUB | no | 0 | demote_future | Present in the workspace, but not wired into the runnable editor. |
| 5.editor/l8.5-tool-context-system | tool context system | FUTURE_STUB | no | 1 | demote_future | Quality coverage still exercises it, but the live desktop host no longer imports it. |
| 5.editor/l8.6-overlay-and-gizmo-system | overlay and gizmo surface | FUTURE_STUB | no | 0 | demote_future | Tiny crate with no live host wiring and no editor reverse dependencies. |
| 5.editor/l8.7-workspace-layout-system | workspace layout system | FUTURE_STUB | no | 0 | split | Split queries/state helpers exist, but nothing in the runnable editor imports this crate yet. |
| 5.editor/l8.8-interaction-routing-system | interaction routing surface | FUTURE_STUB | no | 0 | demote_future | Tiny crate with no live host wiring and no editor reverse dependencies. |
| 5.editor/l8.9-assistant-surface | assistant surface | FUTURE_STUB | no | 0 | demote_future | Intentionally inactive surface with explicit stub reporting instead of fake success. |
| 5.editor/l8.11-build-release-surface | build and release surface | FUTURE_STUB | no | 0 | demote_future | Intentionally inactive surface with explicit stub reporting instead of fake success. |
| 5.editor/l9.1-scene-entity-authoring-suite | scene and entity authoring | FUTURE_STUB | no | 0 | demote_future | Real code exists, but there is no current product-spine import path. |
| 5.editor/l9.3-material-lookdev-authoring-suite | material authoring suite | FUTURE_STUB | no | 1 | demote_future | Backed by tests, but not imported by the active desktop host after this cleanup. |
| 5.editor/l9.4-destruction-fracture-authoring-suite | destruction authoring suite | FUTURE_STUB | no | 0 | demote_future | Planned suite with no current product-spine import path. |
| 5.editor/l9.5-simulation-ai-authoring-suite | simulation and AI authoring | FUTURE_STUB | no | 0 | demote_future | Planned suite with no current product-spine import path. |
| 5.editor/l9.7-animation-cinematics-authoring-suite | animation and cinematics authoring | FUTURE_STUB | no | 0 | demote_future | Planned suite with no current product-spine import path. |
| 5.editor/l9.8-audio-voice-authoring-suite | audio and voice authoring | FUTURE_STUB | no | 0 | demote_future | Non-trivial orphan crate with no live editor import path yet. |
| 5.editor/l9.9-ui-hud-authoring-suite | UI and HUD authoring | FUTURE_STUB | no | 0 | demote_future | Planned suite with no current product-spine import path. |
| 5.editor/l9.10-quest-event-logic-authoring-suite | quest and event logic authoring | FUTURE_STUB | no | 0 | demote_future | Planned suite with no current product-spine import path. |
| 5.editor/l9.11-build-validation-release-suite | build validation suite | FUTURE_STUB | no | 0 | demote_future | Planned suite with no current product-spine import path. |
| 5.editor/l10.0-project-bootstrap-service | project bootstrap service | FUTURE_STUB | no | 0 | demote_future | Split service with no live editor imports; must not be counted as current product functionality. |
| 5.editor/l10.1-import-export-pipeline-service | import/export pipeline service | FUTURE_STUB | no | 0 | demote_future | Planned service with no current product-spine import path. |
| 5.editor/l10.2-graph-authoring-service | graph authoring service | FUTURE_STUB | no | 0 | demote_future | Planned service with no current product-spine import path. |
| 5.editor/l10.3-automation-and-batch-service | automation and batch service | FUTURE_STUB | no | 0 | demote_future | Planned service with no current product-spine import path. |
| 5.editor/l10.4-script-and-hot-reload-service | script and hot reload service | FUTURE_STUB | no | 0 | demote_future | Planned service with no current product-spine import path. |
| 5.editor/l10.5-plugin-and-extension-host | plugin and extension host | FUTURE_STUB | no | 0 | demote_future | Tiny inactive crate that must stay classified as future work, not live extensibility. |
| 5.editor/l10.6-template-preset-and-scaffold-service | template and preset service | FUTURE_STUB | no | 0 | demote_future | Planned service with no current product-spine import path. |
| 5.editor/l10.7-package-market-and-dependency-service | package market service | FUTURE_STUB | no | 0 | demote_future | Tiny inactive crate that must stay classified as future work, not live package management. |
| 5.editor/l11.0-collaboration-session-surface | collaboration session surface | FUTURE_STUB | no | 0 | demote_future | Planned surface with no current product-spine import path. |
| 5.editor/l11.1-review-annotation-surface | review and annotation surface | FUTURE_STUB | no | 0 | demote_future | Planned surface with no current product-spine import path. |
| 5.editor/l11.2-asset-gate-and-approval-surface | asset gate surface | FUTURE_STUB | no | 0 | demote_future | Planned surface with no current product-spine import path. |
| 5.editor/l11.3-playtest-and-capture-operations | playtest and capture surface | FUTURE_STUB | no | 0 | demote_future | Tiny inactive crate that must stay classified as future work, not live playtest tooling. |
| 5.editor/l11.4-production-dashboard-and-traceability | production dashboard surface | FUTURE_STUB | no | 0 | demote_future | Planned surface with no current product-spine import path. |
| 5.editor/l11.5-learning-onboarding-and-help-surface | learning and help surface | FUTURE_STUB | no | 0 | demote_future | Planned surface with no current product-spine import path. |

## Editor orphan/future surfaces

| Crate | Status | Reason | Must not pretend |
|---|---|---|---|
| 5.editor/editor-state-containers | FUTURE_STUB | State container system. Not integrated into product spine. Used by tests. | Must not be reported as production-ready state management. |
| 5.editor/l8.2-outliner-system | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready outliner. |
| 5.editor/l8.3-content-browser-system | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready content browser. |
| 5.editor/l8.4-inspector-system | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready inspector. |
| 5.editor/l8.5-tool-context-system | FUTURE_STUB | Quality suites still exercise the APIs, but the live desktop app no longer imports this crate after the host cleanup. | Must not be reported as production-ready live tool-context routing or active stage state. |
| 5.editor/l8.6-overlay-and-gizmo-system | FUTURE_STUB | 14 LOC placeholder. Not integrated into product spine. | Must not be reported as production-ready overlay/gizmo. |
| 5.editor/l8.7-workspace-layout-system | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready workspace layout. |
| 5.editor/l8.8-interaction-routing-system | FUTURE_STUB | 14 LOC placeholder. Not integrated into product spine. | Must not be reported as production-ready interaction routing. |
| 5.editor/l8.9-assistant-surface | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready assistant. |
| 5.editor/l8.11-build-release-surface | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready build/release. |
| 5.editor/l9.1-scene-entity-authoring-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready scene/entity authoring. |
| 5.editor/l9.3-material-lookdev-authoring-suite | FUTURE_STUB | Material authoring logic exists and tests cover it, but the live editor host does not import it today. | Must not be reported as production-ready material/lookdev editing in the runnable editor. |
| 5.editor/l9.4-destruction-fracture-authoring-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready destruction/fracture. |
| 5.editor/l9.5-simulation-ai-authoring-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready simulation/AI. |
| 5.editor/l9.7-animation-cinematics-authoring-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready animation/cinematics. |
| 5.editor/l9.8-audio-voice-authoring-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready audio/voice. |
| 5.editor/l9.9-ui-hud-authoring-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready UI/HUD. |
| 5.editor/l9.10-quest-event-logic-authoring-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready quest/event logic. |
| 5.editor/l9.11-build-validation-release-suite | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready build validation. |
| 5.editor/l10.0-project-bootstrap-service | FUTURE_STUB | Bootstrap descriptors and services exist, but nothing in the active editor app imports this crate today. | Must not be reported as production-ready project bootstrap flow in the runnable editor. |
| 5.editor/l10.1-import-export-pipeline-service | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready import/export. |
| 5.editor/l10.2-graph-authoring-service | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready graph authoring. |
| 5.editor/l10.3-automation-and-batch-service | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready automation. |
| 5.editor/l10.4-script-and-hot-reload-service | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready script/hot-reload. |
| 5.editor/l10.5-plugin-and-extension-host | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready plugin host. |
| 5.editor/l10.6-template-preset-and-scaffold-service | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready templates/presets. |
| 5.editor/l10.7-package-market-and-dependency-service | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready package market. |
| 5.editor/l11.0-collaboration-session-surface | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready collaboration. |
| 5.editor/l11.1-review-annotation-surface | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready review/annotation. |
| 5.editor/l11.2-asset-gate-and-approval-surface | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready asset gate. |
| 5.editor/l11.3-playtest-and-capture-operations | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready playtest. |
| 5.editor/l11.4-production-dashboard-and-traceability | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready dashboard. |
| 5.editor/l11.5-learning-onboarding-and-help-surface | FUTURE_STUB | Not integrated into product spine. | Must not be reported as production-ready learning/onboarding. |

---

## Full crate inventory

| Layer | Path | Package | Status | Product integrated | Action |
|---|---|---|---|---:|---|
| 7.quality | 7.quality/suites/audio_authoring_matrix | audio_authoring_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/build_release_matrix | build_release_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/editor_app_matrix | editor_app_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/editor_canon_matrix | editor_canon_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/editor_command_matrix | editor_command_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5.9-editor-dto-law | editor_dto_law | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/editor_shell_matrix | editor_shell_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/editor_state_matrix | editor_state_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 5.editor | 5.editor/editor-state-containers | stratumx_editor_state_containers | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 7.quality | 7.quality/suites/end_to_end_matrix | end_to_end_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 2.engine | 2.engine/l3.1-synthesis-systems/acoustics | engine_acoustics | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l2-critical-simulation-families/agents | engine_agents | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l3.1-synthesis-systems/animation | engine_animation | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 7.quality | 7.quality/suites/engine_canon_matrix | engine_canon_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 2.engine | 2.engine/l3.2-resource-systems | engine_content | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-1-foundation | engine_core | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.2-ecs-assembly | engine_ecs | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.3-ecs-query | engine_ecs_query | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.4-ecs-registry | engine_ecs_registry | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l2-critical-simulation-families/field | engine_field | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l3.0-model-systems/generation | engine_generation | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.8-handle | engine_handle | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 3.sdk | 3.sdk/l5.10-engine-handle-refs | engine_handle_refs | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 2.engine | 2.engine/l-0.9-identity | engine_identity | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l3.1-synthesis-systems/imaging | engine_imaging | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l3.0-model-systems/inference | engine_inference | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l2-critical-simulation-families/kinetics | engine_kinetics | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l0.5-shared-world-property-substrate | engine_material | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l1.5-runtime-resource-services/memory-control | engine_memory_control | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l2.5-network-runtime-services/net-latency | engine_net_latency | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l2.5-network-runtime-services/net-sync | engine_net_sync | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l2.5-network-runtime-services/net-transport | engine_net_transport | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 7.quality | 7.quality/suites/engine_perf_harness | engine_perf_harness | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 2.engine | 2.engine/l1.5-runtime-resource-services/residency-control | engine_residency_control | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l1-runtime-kernel/runtime | engine_runtime | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l1-runtime-kernel/runtime-headless | engine_runtime_headless | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l1-runtime-kernel/runtime-realtime | engine_runtime_realtime | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 7.quality | 7.quality/suites/engine_sdk_link_matrix | engine_sdk_link_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 2.engine | 2.engine/l4-startup | engine_startup | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.6-storage-access | engine_storage_access | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.7-storage-layout | engine_storage_layout | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.5-storage-mutation | engine_storage_mutation | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l1.5-runtime-resource-services/stream-control | engine_stream_control | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l1.5-runtime-resource-services/transfer-control | engine_transfer_control | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l0-world-truth | engine_world | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.05-world-region | engine_world_region | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 2.engine | 2.engine/l-0.1-world-spatial | engine_world_spatial | ACTIVE_CORE | yes | keep active; validate through workspace gates |
| 7.quality | 7.quality/suites/environment_authoring_matrix | environment_authoring_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/focus_recovery_matrix | focus_recovery_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/forbidden_shortcuts | forbidden_shortcuts | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5.9-legality-gates | legality_gates | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5.3-link-egress-metrics | link_egress_metrics | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5.2-link-egress-observations | link_egress_observations | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5.1-link-ingress-controls | link_ingress_controls | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5.0-link-ingress-packets | link_ingress_packets | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/material_authoring_matrix | material_authoring_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/proof_region_integration | proof_region_integration | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/repo_hygiene | repo_hygiene | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/route_schema_golden | route_schema_golden | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/sdk_canon_matrix | sdk_canon_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5-compat | sdk_compat | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/sdk_contract_tests | sdk_contract_tests | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/sdk_tooling_link_matrix | sdk_tooling_link_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/smoke | smoke | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 6.apps | 6.apps/editor/stratumx_editor_app | stratumx_editor_app | ACTIVE_PRODUCT | yes | keep active; preserve product-spine behavior |
| 6.apps | 6.apps/engine/stratumx_engine_headless_app | stratumx_engine_headless_app | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 6.apps | 6.apps/engine/stratumx_engine_realtime_app | stratumx_engine_realtime_app | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/sdk_compat_matrix | stratumx_quality_sdk_compat_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/sdk_egress_matrix | stratumx_quality_sdk_egress_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/sdk_ingress_matrix | stratumx_quality_sdk_ingress_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/sdk_legality_matrix | stratumx_quality_sdk_legality_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/tasks/stratumx_quality_tasks | stratumx_quality_tasks | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/support/stratumx_repo_hygiene_support | stratumx_repo_hygiene_support | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/support/stratumx_route_test_support | stratumx_route_test_support | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/support/stratumx_shell_test_support | stratumx_shell_test_support | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 6.apps | 6.apps/stack/stratumx_stack_runtime_app | stratumx_stack_runtime_app | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 6.apps | 6.apps/stack/stratumx_stack_utility | stratumx_stack_utility | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/support/stratumx_test_support | stratumx_test_support | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 4.tooling | 4.tooling/l6.0-authority-core | stratumx_tooling_l6_0_authority_core | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 4.tooling | 4.tooling/l6.0-tool-session | stratumx_tooling_l6_0_tool_session | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 4.tooling | 4.tooling/l6.1-command-envelopes | stratumx_tooling_l6_1_command_envelopes | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 4.tooling | 4.tooling/l6.12-preview-runtime | stratumx_tooling_l6_12_preview_runtime | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 4.tooling | 4.tooling/l6.14-release-runtime | stratumx_tooling_l6_14_release_runtime | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 4.tooling | 4.tooling/l6.6-tool-diagnostics-events | stratumx_tooling_l6_6_tool_diagnostics_events | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 4.tooling | 4.tooling/l6.7-tool-evidence-capture | stratumx_tooling_l6_7_tool_evidence_capture | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 5.editor | 5.editor/l10.0-project-bootstrap-service | stratumx-editor-l10-0-project-bootstrap-service | FUTURE_STUB | no | keep in workspace; split complete but not imported by the live editor host |
| 5.editor | 5.editor/l10.1-import-export-pipeline-service | stratumx-editor-l10-1-import-export-pipeline-service | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l10.2-graph-authoring-service | stratumx-editor-l10-2-graph-authoring-service | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l10.3-automation-and-batch-service | stratumx-editor-l10-3-automation-and-batch-service | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l10.4-script-and-hot-reload-service | stratumx-editor-l10-4-script-and-hot-reload-service | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l10.5-plugin-and-extension-host | stratumx-editor-l10-5-plugin-and-extension-host | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l10.6-template-preset-and-scaffold-service | stratumx-editor-l10-6-template-preset-and-scaffold-service | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l10.7-package-market-and-dependency-service | stratumx-editor-l10-7-package-market-and-dependency-service | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l11.0-collaboration-session-surface | stratumx-editor-l11-0-collaboration-session-surface | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l11.1-review-annotation-surface | stratumx-editor-l11-1-review-annotation-surface | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l11.2-asset-gate-and-approval-surface | stratumx-editor-l11-2-asset-gate-and-approval-surface | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l11.3-playtest-and-capture-operations | stratumx-editor-l11-3-playtest-and-capture-operations | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l11.4-production-dashboard-and-traceability | stratumx-editor-l11-4-production-dashboard-and-traceability | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l11.5-learning-onboarding-and-help-surface | stratumx-editor-l11-5-learning-onboarding-and-help-surface | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l7.0-editor-command-spine | stratumx-editor-l7-0-editor-command-spine | ACTIVE_SUPPORT | yes | keep active; support the live editor command path |
| 5.editor | 5.editor/l8.0-editor-shell | stratumx-editor-l8-0-editor-shell | ACTIVE_PRODUCT | yes | keep active; preserve product-spine behavior |
| 5.editor | 5.editor/l8.1-viewport-system | stratumx-editor-l8-1-viewport-system | ACTIVE_SUPPORT | yes | keep active; support the live viewport host path |
| 5.editor | 5.editor/l8.10-diagnostics-surface | stratumx-editor-l8-10-diagnostics-surface | ACTIVE_SUPPORT | yes | keep active; support the live diagnostics host path |
| 5.editor | 5.editor/l8.11-build-release-surface | stratumx-editor-l8-11-build-release-surface | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l8.2-outliner-system | stratumx-editor-l8-2-outliner-system | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l8.3-content-browser-system | stratumx-editor-l8-3-content-browser-system | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l8.4-inspector-system | stratumx-editor-l8-4-inspector-system | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l8.5-tool-context-system | stratumx-editor-l8-5-tool-context-system | FUTURE_STUB | no | keep in workspace; quality-covered but not imported by the live editor host |
| 5.editor | 5.editor/l8.6-overlay-and-gizmo-system | stratumx-editor-l8-6-overlay-and-gizmo-system | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l8.7-workspace-layout-system | stratumx-editor-l8-7-workspace-layout-system | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l8.8-interaction-routing-system | stratumx-editor-l8-8-interaction-routing-system | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l8.9-assistant-surface | stratumx-editor-l8-9-assistant-surface | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.0-world-authoring-suite | stratumx-editor-l9-0-world-authoring-suite | ACTIVE_SUPPORT | yes | keep active; support live world/session host flows |
| 5.editor | 5.editor/l9.1-scene-entity-authoring-suite | stratumx-editor-l9-1-scene-entity-authoring-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.10-quest-event-logic-authoring-suite | stratumx-editor-l9-10-quest-event-logic-authoring-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.11-build-validation-release-suite | stratumx-editor-l9-11-build-validation-release-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.2-terrain-landscape-authoring-suite | stratumx-editor-l9-2-terrain-landscape-authoring-suite | ACTIVE_SUPPORT | yes | keep active; support terrain authoring through host-owned panel glue |
| 5.editor | 5.editor/l9.3-material-lookdev-authoring-suite | stratumx-editor-l9-3-material-lookdev-authoring-suite | FUTURE_STUB | no | keep in workspace; test-backed but not imported by the live editor host |
| 5.editor | 5.editor/l9.4-destruction-fracture-authoring-suite | stratumx-editor-l9-4-destruction-fracture-authoring-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.5-simulation-ai-authoring-suite | stratumx-editor-l9-5-simulation-ai-authoring-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.6-weather-environment-authoring-suite | stratumx-editor-l9-6-weather-environment-authoring-suite | ACTIVE_SUPPORT | yes | keep active; support environment authoring through host-owned controls |
| 5.editor | 5.editor/l9.7-animation-cinematics-authoring-suite | stratumx-editor-l9-7-animation-cinematics-authoring-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.8-audio-voice-authoring-suite | stratumx-editor-l9-8-audio-voice-authoring-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 5.editor | 5.editor/l9.9-ui-hud-authoring-suite | stratumx-editor-l9-9-ui-hud-authoring-suite | FUTURE_STUB | no | keep in workspace; do not count as product-complete |
| 7.quality | 7.quality/suites/terrain_authoring_matrix | terrain_authoring_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/tool_session_matrix | tool_session_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/tooling_canon_matrix | tooling_canon_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 3.sdk | 3.sdk/l5.8-transport-policies | transport_policies | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/vertical_slice_quality_gates | vertical_slice_quality_gates | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/vertical_slice_tests | vertical_slice_tests | ACTIVE_SUPPORT | yes | keep active; preserve support role |
| 7.quality | 7.quality/suites/world_authoring_matrix | world_authoring_matrix | ACTIVE_SUPPORT | yes | keep active; preserve support role |

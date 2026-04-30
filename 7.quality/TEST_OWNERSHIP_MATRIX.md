# StratumX Test Ownership Matrix

Updated: 2026-04-29

## Baseline

- `cargo fmt --all --check`: FAIL
  - Windows `os error 206` from `cargo fmt --all --check` (`The filename or extension is too long`).
- `cargo check --workspace`: PASS
- `cargo test --workspace`: FAIL
  - Compile failures stop in `editor_state_matrix` test targets.
- `cargo check -p stratumx_editor_state_containers`: PASS
- `cargo test -p editor_state_matrix --tests`: FAIL
- `cargo test -p stratumx-editor-l8-5-tool-context-system --tests`: PASS
- `cargo test -p stratumx-editor-l8-10-diagnostics-surface --tests`: PASS
- `cargo test -p stratumx-editor-l9-8-audio-voice-authoring-suite --tests`: PASS
  - Actual workspace package is `stratumx-editor-l9-8-audio-voice-authoring-suite`; the requested `l9_4` package name is stale.

## Failure Class Map

1. `EventBus` type drift
   - `editor_host_tests.rs`
   - `integration_service_coordination.rs`
   - `property_editor_host_delegation.rs`
   - `environment_authoring_service_tests.rs`
2. `Arc` mutability / `CacheLayer` ownership
   - `integration_error_recovery.rs`
   - `property_cache_invalidation_propagation.rs`
   - `property_cache_metrics_accuracy.rs`
   - `property_derived_state_rebuildability.rs`
3. `EnvironmentState` field drift
   - `environment_authoring_service_tests.rs`
4. `SessionState` API drift
   - `property_05_state_container_authority.rs`
   - `session_state_unit_tests.rs`
   - `session_state_property_tests.rs`
   - `session_state_serialization_tests.rs`
5. `StateModification` stale variants
   - `session_state_unit_tests.rs`
   - `session_state_property_tests.rs`
6. `EntityId` vs `Uuid` drift
   - `session_state_property_tests.rs`
   - `session_state_serialization_tests.rs`
   - `property_05_state_container_authority.rs`
7. `PanelId` / `String` drift
   - `property_query_layer_delegation.rs`
   - `workspace_persistence_tests.rs`
8. Diagnostic private/public import drift
   - `query_immutability.rs`
9. Query view missing/stale imports
   - `query_immutability.rs`
10. `AudioSource` / `AudioRegistryState` drift
   - `state_query_immutability_tests.rs`
11. `DockPosition` / `PanelGeometry` drift
   - `query_immutability.rs`
   - `workspace_persistence_tests.rs`
12. `Option<T>` misuse
   - `property_query_layer_delegation.rs`
   - `session_state_unit_tests.rs`
   - `property_derived_state_rebuildability.rs`
13. `usize` / `u64` mismatch
   - `property_cache_metrics_accuracy.rs`
   - `property_05_state_container_authority.rs`
14. proptest strategy return type mismatch
   - `session_state_property_tests.rs`
   - `session_state_serialization_tests.rs`
15. stale enum variants / validation codes
   - `session_state_property_tests.rs`
   - `session_state_unit_tests.rs`
16. stale test assumptions against FUTURE_STUB crates
   - `query_immutability.rs`
   - `session_state_*`
   - `state_query_immutability_tests.rs`
   - `workspace_persistence_tests.rs`
   - `property_05_state_container_authority.rs`
   - `terrain_command_executor_tests.rs`

## Ownership Matrix

| Test file | Crate under test | Owner API | Current failure class | Expected contract | Resolution | Why this path |
| --- | --- | --- | --- | --- | --- | --- |
| `7.quality/suites/editor_state_matrix/tests/editor_host_tests.rs` | `stratumx_editor_state_containers` | `EditorServices::new`, `EditorHost`, runtime service constructors | `EventBus` type drift | Editor-facing runtime services should accept a shared `Arc<dyn EventBus>` boundary, not require a concrete `Arc<BasicEventBus>` at call sites | `B` | The test is exercising a legitimate service-composition boundary. Fixing the constructor contract is a real compatibility surface, not a fake adapter. |
| `7.quality/suites/editor_state_matrix/tests/integration_service_coordination.rs` | `stratumx_editor_state_containers` | `EditorServices`, `EditorHost` | `EventBus` type drift | Same `Arc<dyn EventBus>` contract as above | `B` | Same public service boundary as `editor_host_tests.rs`; the fix should be unified, not duplicated in tests. |
| `7.quality/suites/editor_state_matrix/tests/property_editor_host_delegation.rs` | `stratumx_editor_state_containers` | `EditorServices`, `EditorHost` | `EventBus` type drift | Same `Arc<dyn EventBus>` contract as above | `B` | Property test is stale only because the public constructor is too concrete. One production fix should close all three targets. |
| `7.quality/suites/editor_state_matrix/tests/environment_authoring_service_tests.rs` | `stratumx_editor_state_containers` | `EnvironmentAuthoringService`, `EnvironmentState`, `BasicEventBus` | `EventBus` subscribe signature drift; `EnvironmentState` field drift | Tests must use `subscribe(event_type, callback)`, `configure_sky(&[f32; 3])`, and the real environment shape (`time_of_day`, `weather`) | `A` | The service and state model already define the current contract. The test is asserting an older future API (`ambient_light`, `weather_condition`, extra `configure_sky` arg). |
| `7.quality/suites/editor_state_matrix/tests/property_cache_invalidation_propagation.rs` | `stratumx_editor_state_containers` | `CacheLayer`, `CacheMetrics` | local mutability drift on `CacheLayer` | Local cache used for mutation-heavy tests must be an owned `mut CacheLayer`, not an immutable binding | `A` | This is a pure test ownership mistake. No production API change is needed to mutate a local cache value. |
| `7.quality/suites/editor_state_matrix/tests/integration_error_recovery.rs` | `stratumx_editor_state_containers` | `StateContainerSystem`, `CacheLayer`, `OwnershipViolation` | `Arc` mutability drift; stale assumption that `StateContainerSystem::new` returns iterable violations | Cache mutation must follow owned mutable cache or explicit locking; initialization/validation tests must match the actual `Result<_, String>` and `Result<(), OwnershipViolation>` contracts | `A` | The target is testing the current system incorrectly, not exposing a missing public surface. Realigning it keeps ownership semantics honest. |
| `7.quality/suites/editor_state_matrix/tests/property_cache_metrics_accuracy.rs` | `stratumx_editor_state_containers` | `CacheLayer`, `CacheMetrics`, terrain cache entries | `Arc` mutability drift; `usize/u64` mismatch; stale cache-entry assumptions | Metrics counters are `usize`, and tests must use real cache entry types instead of casting whole cache containers through trait objects | `A` | The property is valid, but the test encodes the wrong concrete types and count widths. |
| `7.quality/suites/editor_state_matrix/tests/property_derived_state_rebuildability.rs` | `stratumx_editor_state_containers` | `CacheLayer`, `TerrainPreviewCache`, `MaterialRegistryCache` | cache entry/container drift; `Option<T>` misuse; unsafe cast-based access | Tests must use real cache entry types and accessors (`TerrainChunkPreview`, material entries) without unsafe pointer casts or `Option` misuse | `A` | The current test is stale and uses forbidden unsafe assumptions. Rewriting it to the actual cache contract is the honest path. |
| `7.quality/suites/editor_state_matrix/tests/property_query_layer_delegation.rs` | `stratumx_editor_state_containers` | `QueryLayer`, `WorkspaceState`, `DiagnosticsState`, `WorldState` | `PanelId`/`String` drift; `Option<T>` misuse | Query results must be compared against the same domain types (`PanelId`, `Option<PanelId>`) and current collection ownership rules | `A` | The target is sound, but the direct-side expectations are still written against an older stringly-typed query API. |
| `7.quality/suites/editor_state_matrix/tests/query_immutability.rs` | Mixed: `stratumx_editor_state_containers`, `stratumx-editor-l8-5-tool-context-system`, `stratumx-editor-l8-10-diagnostics-surface` | project queries, workspace queries, diagnostics queries, world queries | private diagnostics imports; missing/stale view imports; `DockPosition`/`PanelGeometry` drift; stale FUTURE_STUB assumptions | Tests must import diagnostics types through public API, use actual owner crates for project/diagnostics views, and only require workspace/world views that really belong to `state_containers` | `D + E` | This file spans multiple real owners. One FUTURE_STUB crate should not pretend to own project snapshots, diagnostics lineage, and workspace docking summaries simultaneously. |
| `7.quality/suites/editor_state_matrix/tests/property_05_state_container_authority.rs` | Mixed: `stratumx_editor_state_containers` plus canonical session contract in `stratumx-editor-l8-5-tool-context-system` | `ProjectState`, `WorldState`, `SessionState`, `WorkspaceState`, `DiagnosticsState`, `MaterialRegistryState` | `SessionState` API drift; typed-event vs string-event drift; `Uuid`-based selection drift | Project/world/workspace/material/diagnostics authority checks stay on real state-container APIs; session authority checks must follow the canonical session contract; event assertions must match actual published event types or explicit compatibility APIs | `A + D` | The file is overloading `state_containers` with session semantics that already live in `l8.5`, while also assuming typed events where current owners publish strings. |
| `7.quality/suites/editor_state_matrix/tests/session_state_unit_tests.rs` | `stratumx-editor-l8-5-tool-context-system` | `SessionState`, `SelectionState`, `SelectionMode`, `ToolMode`, `StateModification`, `ValidationErrorCode` | `SessionState` API drift; `StateModification` drift; `Option<T>` misuse; stale validation codes | Tests must target the real session owner crate and use its enum-based contract, accessors, and error codes | `D` | The canonical session implementation already exists in `l8.5` with commands, queries, validation, and persistence. Keeping these assertions on the FUTURE_STUB duplicate would reward fake compatibility. |
| `7.quality/suites/editor_state_matrix/tests/session_state_property_tests.rs` | `stratumx-editor-l8-5-tool-context-system` | same as above | `EntityId` vs `Uuid`; proptest strategy drift; stale variants/codes | Strategies must generate `EntityId`, `SelectionMode`, and `ToolMode` directly, and only use real `ValidationErrorCode` / `StateModification` variants | `D` | This is the clearest case of a stale test targeting the wrong owner crate. The property logic is useful, but the imported surface is not. |
| `7.quality/suites/editor_state_matrix/tests/session_state_serialization_tests.rs` | `stratumx-editor-l8-5-tool-context-system` | `SessionState` persistence contract | `EntityId` vs `Uuid`; `WorldIdentity` shape drift; enum/value drift | Serialization properties must use the actual serializable session model from `l8.5` | `D` | The current test mixes `state_containers` shapes with the canonical session persistence surface. |
| `7.quality/suites/editor_state_matrix/tests/state_query_immutability_tests.rs` | `stratumx_editor_state_containers` | `StateQueries`, `SessionState`, `MaterialRegistryState`, `AudioRegistryState`, `DiagnosticsState` | trait signature drift; audio contract drift; stale session helper assumptions | Mock implementations must match the actual `StateQueries` trait signatures and current domain types returned by `state_containers` (`Vec<T>`, current audio structs, current session surface) | `A` | This file is testing the right trait but with stale mock signatures and an older audio/session vocabulary. |
| `7.quality/suites/editor_state_matrix/tests/terrain_command_executor_tests.rs` | `stratumx_editor_state_containers` | `runtime::terrain_command_executor::{TerrainCommandExecutor, RuntimeHostAccess}` | stale FUTURE_STUB implementation | Heightmap import must reject malformed `raw`/`r16` payloads, mutate the active terrain resolution/samples, and mark terrain GPU state dirty through the runtime host boundary | `B` | The test is exercising a real runtime contract. The failure came from a no-op stub in the crate under test, so the honest fix is to implement the minimal production behavior rather than weakening the test. |
| `7.quality/suites/editor_canon_matrix/tests/authoring_to_runtime_cycle.rs` | `stratumx_tooling_l6_0_tool_session` | `CommandExecutor`, `PacketExecutor`, `CommandLifecycleState`, vertical-slice routing | lifecycle contract drift; scene runtime shim | Submitted scene commands must advance through the real lifecycle (`Accepted -> Validated -> Running -> Success/RetryableFailure`) and route `SceneBootstrap` / `SceneFireTestShot` / `SceneReset` through the actual packet-executor/session boundary | `B` | The failing assertion exposed a real runtime contract bug, not a stale test. The compatibility surface belongs in the command executor and routing layer. |
| `7.quality/suites/editor_state_matrix/tests/workspace_persistence_tests.rs` | `stratumx_editor_state_containers` | `WorkspaceState`, `WorkspaceOwner`, `PanelGeometry`, file persistence | `PanelId`/dock drift; `PanelGeometry` constructor drift; stale `DockPosition::Floating`; stubbed file round-trip | Tests must align with the real geometry/docking split, and `WorkspaceState::{serialize_to_file, deserialize_from_file}` must become a real persistence contract | `A + B` | Part of the test is stale, but the persistence API itself is a public contract and cannot stay a no-op if we want honest green tests. |

## First Fix Batch

1. Unify runtime service constructors around `Arc<dyn EventBus>` and re-run the three editor-host/service targets.
2. Realign pure test-side cache ownership mistakes (`mut CacheLayer` vs `Arc`) before touching `CacheLayer` implementation.
3. Repoint session-state tests to `stratumx-editor-l8-5-tool-context-system` and convert their generators/assertions to the real session types.
4. Then tackle honest production implementations that are currently stubbed but are part of the tested contract:
   - `QueryLayer`
   - `CacheLayer`
   - `WorkspaceState` file persistence

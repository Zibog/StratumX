# STRATUMX GOLD CLEANUP EXECUTION MANUAL FOR CODE AGENT

**Purpose:** exact mechanical cleanup instructions for a code-writing agent.

**Do not implement features in this pass.**  
This pass is only about repository cleanliness, monolith splitting, test placement, crate status truth, and command-gate stabilization.

## Current observed repository facts

| Fact | Value |
|---|---:|
| Workspace packages | 130 |
| Workspace members | 130 |
| Packages outside workspace | 0 |
| Layer violations observed statically | 0 |
| Production Rust LOC excluding `7.quality` | ~65k |
| `7.quality` Rust LOC | ~103k |
| Current docs marker | `SX-CANON/1.0.28/STACK-v34` |
| README marker currently stale | `SX-CANON/1.0.24/STACK-v30` |
| Biggest monolith | `3.sdk/l5.9-legality-gates/src/command_gates.rs` at 1534 LOC |

## Mandatory final commands

Run these after cleanup:

```powershell
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
.\tools\doctor.ps1
.\tools\verify.ps1
.\tools\full.ps1
```

Bash equivalent:

```bash
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
./tools/doctor.sh
./tools/verify.sh
./tools/full.sh
```

## Absolute prohibitions

Do not add:

- Vulkan backend;
- DX12 backend;
- Metal backend;
- new graphics feature;
- new physics feature;
- new netcode feature;
- new material graph;
- new editor panel;
- new gameplay system.

Do only:

- split;
- move;
- classify;
- delete placeholder;
- preserve API;
- fix imports;
- update README/status;
- improve gates.

# 1. Current monolith inventory
| File | LOC |
|---|---:|
| `3.sdk/l5.9-legality-gates/src/command_gates.rs` | 1534 |
| `3.sdk/l5.2-link-egress-observations/src/editor_authoring_observations/types.rs` | 519 |
| `4.tooling/l6.0-authority-core/src/recovery/policies.rs` | 371 |
| `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs` | 362 |
| `2.engine/l0.5-shared-world-property-substrate/src/types.rs` | 351 |
| `4.tooling/l6.1-command-envelopes/src/promoted_commands/types.rs` | 329 |
| `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs` | 320 |
| `6.apps/editor/stratumx_editor_app/src/desktop_app/editor_app.rs` | 316 |
| `2.engine/l0.5-shared-world-property-substrate/src/queries.rs` | 299 |
| `4.tooling/l6.0-authority-core/src/conveyor.rs` | 290 |
| `2.engine/l0.5-shared-world-property-substrate/src/validation.rs` | 266 |
| `3.sdk/l5.0-link-ingress-packets/src/editor_authoring_ingress/diagnostics/types.rs` | 259 |
| `5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/types.rs` | 253 |
| `5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service.rs` | 246 |
| `4.tooling/l6.1-command-envelopes/src/promoted_commands/builders.rs` | 243 |
| `5.editor/l8.10-diagnostics-surface/src/diagnostics_types.rs` | 241 |
| `5.editor/l9.0-world-authoring-suite/src/world_session_service.rs` | 234 |
| `4.tooling/l6.1-command-envelopes/src/validation/rules.rs` | 230 |
| `5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view.rs` | 229 |
| `5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries.rs` | 224 |
| `4.tooling/l6.1-command-envelopes/src/validation/validators.rs` | 223 |
| `4.tooling/l6.0-authority-core/src/containers/audio/mod.rs` | 212 |
| `5.editor/l8.0-editor-shell/src/command_palette_state.rs` | 209 |
| `5.editor/l8.0-editor-shell/src/editor_shell.rs` | 206 |
| `5.editor/l9.6-weather-environment-authoring-suite/src/runtime/authoring_service.rs` | 205 |
| `5.editor/l9.2-terrain-landscape-authoring-suite/src/desktop/terrain_panel.rs` | 201 |
| `4.tooling/l6.14-release-runtime/src/first_result_verification/evidence.rs` | 201 |

# 2. Test-like files outside `7.quality`
| File | LOC | Tests | cfg(test) | Action |
|---|---:|---:|---:|---|
| `2.engine/l0.5-shared-world-property-substrate/src/queries.rs` | 299 | 7 | 1 | move/delete unless tiny pure unit |
| `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs` | 362 | 4 | 1 | move/delete unless tiny pure unit |
| `2.engine/l0.5-shared-world-property-substrate/src/types.rs` | 351 | 4 | 1 | move/delete unless tiny pure unit |
| `2.engine/l0.5-shared-world-property-substrate/src/validation.rs` | 266 | 5 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.0-authority-core/src/containers/audio/mod.rs` | 212 | 5 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.0-authority-core/src/containers/material_authority_container.rs` | 138 | 2 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.0-authority-core/src/conveyor.rs` | 290 | 4 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.0-authority-core/src/diagnostics.rs` | 175 | 3 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.0-authority-core/src/layer_purity.rs` | 75 | 2 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.0-authority-core/src/recovery/policies.rs` | 371 | 7 | 2 | move/delete unless tiny pure unit |
| `4.tooling/l6.0-authority-core/src/recovery/strategies.rs` | 80 | 1 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.1-command-envelopes/src/canonical_command_schema/mod.rs` | 71 | 2 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.1-command-envelopes/src/command_lifecycle/stages.rs` | 96 | 3 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs` | 320 | 7 | 2 | move/delete unless tiny pure unit |
| `4.tooling/l6.1-command-envelopes/src/promoted_commands/builders.rs` | 243 | 3 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.1-command-envelopes/src/validation/errors.rs` | 50 | 1 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.1-command-envelopes/src/validation/rules.rs` | 230 | 3 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.14-release-runtime/src/first_result_verification/checks.rs` | 122 | 3 | 1 | move/delete unless tiny pure unit |
| `4.tooling/l6.14-release-runtime/src/first_result_verification/evidence.rs` | 201 | 2 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_context.rs` | 179 | 0 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_dispatch_core.rs` | 69 | 2 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_enums.rs` | 196 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_execution.rs` | 92 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_focus.rs` | 175 | 9 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_ids.rs` | 76 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_preconditions.rs` | 96 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_queries.rs` | 74 | 1 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_registration/mod.rs` | 57 | 0 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_registration/tests.rs` | 10 | 1 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_registry.rs` | 170 | 4 | 8 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_registry_types.rs` | 129 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_validation/mod.rs` | 37 | 0 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/action_validation/tests.rs` | 10 | 1 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/canonical_action_api.rs` | 147 | 2 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/dispatch_audio.rs` | 141 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/dispatch_material.rs` | 144 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/editor_command_spine.rs` | 127 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/focus_recovery.rs` | 73 | 4 | 1 | move/delete unless tiny pure unit |
| `5.editor/l7.0-editor-command-spine/src/focus_routing.rs` | 189 | 8 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.0-editor-shell/src/command_palette_state.rs` | 209 | 2 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.0-editor-shell/src/command_spine/spine_lifecycle.rs` | 101 | 4 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.10-diagnostics-surface/src/diagnostics_cache.rs` | 110 | 1 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.3-content-browser-system/src/content_browser_cache.rs` | 102 | 1 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.5-tool-context-system/src/session_state/mod.rs` | 60 | 4 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.5-tool-context-system/src/state_graph/mod.rs` | 47 | 0 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.5-tool-context-system/src/state_queries.rs` | 181 | 3 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.7-workspace-layout-system/src/persistence/workspace_persistence_view.rs` | 180 | 5 | 1 | move/delete unless tiny pure unit |
| `5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries.rs` | 224 | 6 | 1 | move/delete unless tiny pure unit |
| `5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view.rs` | 229 | 5 | 1 | move/delete unless tiny pure unit |
| `5.editor/l9.0-world-authoring-suite/src/world_session_service.rs` | 234 | 4 | 1 | move/delete unless tiny pure unit |
| `5.editor/l9.3-material-lookdev-authoring-suite/src/material_cache.rs` | 114 | 2 | 1 | move/delete unless tiny pure unit |

# 3. Editor orphan packages to classify
| Path | Package |
|---|---|
| `5.editor/l10.0-project-bootstrap-service` | `stratumx-editor-l10-0-project-bootstrap-service` |
| `5.editor/l10.1-import-export-pipeline-service` | `stratumx-editor-l10-1-import-export-pipeline-service` |
| `5.editor/l10.2-graph-authoring-service` | `stratumx-editor-l10-2-graph-authoring-service` |
| `5.editor/l10.3-automation-and-batch-service` | `stratumx-editor-l10-3-automation-and-batch-service` |
| `5.editor/l10.4-script-and-hot-reload-service` | `stratumx-editor-l10-4-script-and-hot-reload-service` |
| `5.editor/l10.5-plugin-and-extension-host` | `stratumx-editor-l10-5-plugin-and-extension-host` |
| `5.editor/l10.6-template-preset-and-scaffold-service` | `stratumx-editor-l10-6-template-preset-and-scaffold-service` |
| `5.editor/l10.7-package-market-and-dependency-service` | `stratumx-editor-l10-7-package-market-and-dependency-service` |
| `5.editor/l11.0-collaboration-session-surface` | `stratumx-editor-l11-0-collaboration-session-surface` |
| `5.editor/l11.1-review-annotation-surface` | `stratumx-editor-l11-1-review-annotation-surface` |
| `5.editor/l11.2-asset-gate-and-approval-surface` | `stratumx-editor-l11-2-asset-gate-and-approval-surface` |
| `5.editor/l11.3-playtest-and-capture-operations` | `stratumx-editor-l11-3-playtest-and-capture-operations` |
| `5.editor/l11.4-production-dashboard-and-traceability` | `stratumx-editor-l11-4-production-dashboard-and-traceability` |
| `5.editor/l11.5-learning-onboarding-and-help-surface` | `stratumx-editor-l11-5-learning-onboarding-and-help-surface` |
| `5.editor/l8.11-build-release-surface` | `stratumx-editor-l8-11-build-release-surface` |
| `5.editor/l8.2-outliner-system` | `stratumx-editor-l8-2-outliner-system` |
| `5.editor/l8.3-content-browser-system` | `stratumx-editor-l8-3-content-browser-system` |
| `5.editor/l8.4-inspector-system` | `stratumx-editor-l8-4-inspector-system` |
| `5.editor/l8.6-overlay-and-gizmo-system` | `stratumx-editor-l8-6-overlay-and-gizmo-system` |
| `5.editor/l8.7-workspace-layout-system` | `stratumx-editor-l8-7-workspace-layout-system` |
| `5.editor/l8.8-interaction-routing-system` | `stratumx-editor-l8-8-interaction-routing-system` |
| `5.editor/l8.9-assistant-surface` | `stratumx-editor-l8-9-assistant-surface` |
| `5.editor/l9.1-scene-entity-authoring-suite` | `stratumx-editor-l9-1-scene-entity-authoring-suite` |
| `5.editor/l9.10-quest-event-logic-authoring-suite` | `stratumx-editor-l9-10-quest-event-logic-authoring-suite` |
| `5.editor/l9.11-build-validation-release-suite` | `stratumx-editor-l9-11-build-validation-release-suite` |
| `5.editor/l9.4-destruction-fracture-authoring-suite` | `stratumx-editor-l9-4-destruction-fracture-authoring-suite` |
| `5.editor/l9.5-simulation-ai-authoring-suite` | `stratumx-editor-l9-5-simulation-ai-authoring-suite` |
| `5.editor/l9.7-animation-cinematics-authoring-suite` | `stratumx-editor-l9-7-animation-cinematics-authoring-suite` |
| `5.editor/l9.8-audio-voice-authoring-suite` | `stratumx-editor-l9-8-audio-voice-authoring-suite` |
| `5.editor/l9.9-ui-hud-authoring-suite` | `stratumx-editor-l9-9-ui-hud-authoring-suite` |

# 4. Phase zero — branch and freeze

## Step 4.1 — Create branch

```powershell
git checkout -b cleanup/gold-repo-stabilization
```

## Step 4.2 — Create scratch directory

```powershell
New-Item -ItemType Directory _cleanup_scratch
```

Do not commit `_cleanup_scratch`.

## Step 4.3 — Feature freeze

Write this line into `1.docs/developer_docs/CODE_CLEANUP_STATUS_LEDGER.md` after creating it:

```markdown
Current phase: repository stabilization. Feature expansion is blocked until cleanup gates pass.
```

# 5. Root truth cleanup

## Step 5.1 — Update README marker

Open `README.md`.

Replace:

```text
SX-CANON/1.0.24/STACK-v30
```

with:

```text
SX-CANON/1.0.28/STACK-v34
```

Add/update facts:

```markdown
- Workspace packages: 130
- Workspace members: 130
- Production Rust LOC excluding `7.quality`: ~65k
- Quality Rust LOC: ~103k
- Canon docs: `SX-CANON/1.0.28/STACK-v34`
```

Add:

```markdown
## Workspace validation policy

`cargo build` may use `default-members`.
Full validation is always:

```bash
cargo check --workspace
cargo test --workspace
tools/full
```
```

## Step 5.2 — Clean root

Root allowed items:

```text
Cargo.toml
Cargo.lock
README.md
.gitignore
.github/
.git/
1.docs/
2.engine/
3.sdk/
4.tooling/
5.editor/
6.apps/
7.quality/
9.assets/
tools/
```

Move or delete:

```text
*.txt
*.log
*.orig
STRATUMX_*_PLAN.md
PATCH_NOTES_*.md
AUDIT_*.md
ROADMAP_*.md
```

## Step 5.3 — Create cleanup ledger

Create:

```text
1.docs/developer_docs/CODE_CLEANUP_STATUS_LEDGER.md
```

Insert:

```markdown
# Code Cleanup Status Ledger

Stack: SX-CANON/1.0.28/STACK-v34

| Gate | Required result | Status | Action |
|---|---|---|---|
| Workspace membership | active packages = workspace members | pending | run workspace validation |
| Layer dependencies | 0 upward violations | pending | run layer check |
| Test placement | 0 heavy tests outside `7.quality` | pending | move/delete tests |
| Monoliths | no production file >300 LOC | pending | split files |
| Root garbage | 0 stray files | pending | clean root |
| Orphan crates | all classified | pending | crate ledger |
| Stub crates | no fake active stubs | pending | demote or implement minimal role |
```

# 6. Workspace and tools honesty

## Step 6.1 — Validate workspace members

Run:

```powershell
.\tools\validate-workspace.ps1
```

If it fails, update root `Cargo.toml`.

Rule:

```text
Every active package must be in [workspace].members.
Future/legacy packages must be classified and either excluded intentionally or kept with FUTURE_STUB status.
```

## Step 6.2 — Document `default-members`

If `default-members` stays engine-only, document this in README.

Do not let users believe `cargo test` without `--workspace` validates all 130 packages.

## Step 6.3 — Doctor output

Ensure `tools/doctor` reports:

```text
workspace membership
layer dependencies
test placement
file size discipline
orphan crate classification
stub crate classification
root garbage
docs marker consistency
```

If a gate is not implemented, print `warn`, not fake `ok`.

# 7. Test cleanup

## Step 7.1 — Delete placeholder test files

Inspect:

```text
5.editor/l7.0-editor-command-spine/src/action_registration/tests.rs
5.editor/l7.0-editor-command-spine/src/action_validation/tests.rs
```

If they contain only placeholder assertions, delete them.

## Step 7.2 — Move inline tests out of production files

Priority files:

```text
5.editor/l7.0-editor-command-spine/src/action_registry.rs
5.editor/l7.0-editor-command-spine/src/action_focus.rs
5.editor/l7.0-editor-command-spine/src/focus_routing.rs
4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs
4.tooling/l6.0-authority-core/src/recovery/policies.rs
2.engine/l0.5-shared-world-property-substrate/src/queries.rs
2.engine/l0.5-shared-world-property-substrate/src/validation.rs
2.engine/l0.5-shared-world-property-substrate/src/runtime.rs
2.engine/l0.5-shared-world-property-substrate/src/types.rs
5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries.rs
5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view.rs
```

Move integration/property/domain behavior tests into:

```text
7.quality/suites/
```

Use existing suites where possible.

## Step 7.3 — Allowed local tests

Local `#[cfg(test)]` may remain only if:

- under 30 LOC total;
- pure unit invariant;
- no file system;
- no async;
- no integration;
- no property test;
- no editor/tooling route.

Everything else moves to `7.quality`.

## Step 7.4 — Run gate

```powershell
.\tools\check-test-placement.ps1
```

# 8. Crate status ledger

## Step 8.1 — Create ledger

Create:

```text
7.quality/inventory/crate_status_ledger.md
```

Use:

```markdown
# StratumX Crate Status Ledger

| Layer | Path | Package | Status | Product integrated | Action |
|---|---|---|---|---:|---|
```

Allowed statuses:

```text
ACTIVE_CORE
ACTIVE_PRODUCT
ACTIVE_SUPPORT
FUTURE_STUB
LEGACY
DELETE
```

## Step 8.2 — Active editor product spine

Mark as `ACTIVE_PRODUCT`:

```text
5.editor/l7.0-editor-command-spine
5.editor/l8.0-editor-shell
5.editor/l8.1-viewport-system
5.editor/l8.5-tool-context-system
5.editor/l8.10-diagnostics-surface
5.editor/l9.0-world-authoring-suite
5.editor/l9.2-terrain-landscape-authoring-suite
5.editor/l9.3-material-lookdev-authoring-suite
5.editor/l9.6-weather-environment-authoring-suite
5.editor/l10.0-project-bootstrap-service
6.apps/editor/stratumx_editor_app
```

## Step 8.3 — Classify orphan editor crates

Recommended action: mark all orphan editor crates as `FUTURE_STUB` unless they are wired into the active product spine in this cleanup. Do not wire them just to avoid the label.

For every `FUTURE_STUB`, add to crate root:

```rust
//! FUTURE_STUB.
//!
//! This crate is present as a canonical future surface.
//! It is not part of the active product spine yet.
//! It must not be counted as product-complete.
```

If the crate has no planned near-term use, mark `DELETE` and remove it from workspace in a separate deletion commit.

# 9. Required monolith splits

## Task 11. Split `3.sdk/l5.9-legality-gates/src/command_gates.rs`

**Current LOC:** 1534  
**Role:** SDK command legality domain dispatcher  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `3.sdk/l5.9-legality-gates/src/command_gates.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
3.sdk/l5.9-legality-gates/src/command_gates/
  mod.rs
  common.rs
  scene.rs
  material.rs
  terrain.rs
  actor.rs
  asset.rs
  sky_weather.rs
  ballistics.rs
  destruction.rs
  material_world.rs
  nav_door_inventory.rs
  population_npc.rs
  tactics.rs
  ecology.rs
  reason_chain.rs
  vertical_slice.rs
  animation.rs
```

### Required rules

- `common.rs` owns shared rejection helpers only.
- Domain files own only validation functions for that domain.
- `mod.rs` must `pub use` every domain module to keep external imports stable.
- Do not invent new validation semantics.
- Use existing `LegalityRejection`, `LegalityRejectionReason`, `LegalityVerdict`.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 12. Split `3.sdk/l5.2-link-egress-observations/src/editor_authoring_observations/types.rs`

**Current LOC:** 519  
**Role:** SDK editor observation enum aggregation  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `3.sdk/l5.2-link-egress-observations/src/editor_authoring_observations/types.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
3.sdk/l5.2-link-egress-observations/src/editor_authoring_observations/
  observation.rs
  observation_scene.rs
  observation_material.rs
  observation_terrain.rs
  observation_environment.rs
  observation_runtime.rs
  observation_diagnostics.rs
  observation_living.rs
  observation_animation.rs
  observation_audio.rs
  types.rs
```

### Required rules

- `types.rs` should no longer be the giant enum owner.
- Keep `EditorAuthoringObservation` name stable.
- Prefer sub-enums per domain if the package can absorb the API change; otherwise keep the top-level enum in `observation.rs` and move DTO helpers out.
- Update `mod.rs` re-exports.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 13. Split `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs`

**Current LOC:** 362  
**Role:** world property runtime operations  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
2.engine/l0.5-shared-world-property-substrate/src/runtime/
  mod.rs
  codec.rs
  update.rs
  merge.rs
  publication.rs
  persistence.rs
  errors.rs
```

### Required rules

- `codec.rs` owns `PersistenceCodec`.
- `update.rs` owns update order and field mutation routines.
- `merge.rs` owns partial substrate merge.
- `publication.rs` owns observation/output conversion.
- Use `Result<_, String>` only if the existing API already does; otherwise introduce an internal error type only if all call sites are updated.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 14. Split `2.engine/l0.5-shared-world-property-substrate/src/types.rs`

**Current LOC:** 351  
**Role:** world property core types  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `2.engine/l0.5-shared-world-property-substrate/src/types.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
2.engine/l0.5-shared-world-property-substrate/src/types/
  mod.rs
  property.rs
  storage.rs
  field_value.rs
  cell_field.rs
  object_local_field.rs
  surface_field.rs
  volume_field.rs
  substrate.rs
  conflict.rs
  update_order.rs
```

### Required rules

- Do not put behavior into type files.
- `mod.rs` must re-export all previous public types.
- Keep serde derives stable.
- Do not rename serialized enum variants.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 15. Split `2.engine/l0.5-shared-world-property-substrate/src/queries.rs`

**Current LOC:** 299  
**Role:** world property read/query surface  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `2.engine/l0.5-shared-world-property-substrate/src/queries.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
2.engine/l0.5-shared-world-property-substrate/src/queries/
  mod.rs
  field_queries.rs
  material_queries.rs
  surface_queries.rs
  volume_queries.rs
  debug_queries.rs
  errors.rs
```

### Required rules

- Query files must be read-only.
- No mutation in query modules.
- If a function mutates substrate, move it to runtime/update instead.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 16. Split `2.engine/l0.5-shared-world-property-substrate/src/validation.rs`

**Current LOC:** 266  
**Role:** world property validation  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `2.engine/l0.5-shared-world-property-substrate/src/validation.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
2.engine/l0.5-shared-world-property-substrate/src/validation/
  mod.rs
  schema.rs
  field_scope.rs
  material_pair.rs
  storage.rs
  conflict.rs
  errors.rs
```

### Required rules

- Validation returns existing error style unless you update all call sites.
- Keep schema validation separate from runtime validation.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 17. Split `4.tooling/l6.0-authority-core/src/recovery/policies.rs`

**Current LOC:** 371  
**Role:** tooling recovery policy table  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `4.tooling/l6.0-authority-core/src/recovery/policies.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
4.tooling/l6.0-authority-core/src/recovery/
  mod.rs
  targets.rs
  retry_policy.rs
  rollback_policy.rs
  focus_recovery.rs
  evidence_recovery.rs
  manual_intervention.rs
  policy_registry.rs
```

### Required rules

- `targets.rs` owns `RecoveryTarget`.
- policy-specific files own constructors and constants.
- Existing callers must import through `recovery::*` or stable re-exports.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 18. Split `4.tooling/l6.1-command-envelopes/src/promoted_commands/types.rs`

**Current LOC:** 329  
**Role:** promoted command type aggregation  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `4.tooling/l6.1-command-envelopes/src/promoted_commands/types.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
4.tooling/l6.1-command-envelopes/src/promoted_commands/
  mod.rs
  ids.rs
  payloads.rs
  domains.rs
  routing.rs
  disabled_reasons.rs
  lifecycle.rs
  serialization.rs
```

### Required rules

- Keep command IDs stable.
- Do not merge unrelated command payloads.
- Avoid editor-specific types in tooling.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 19. Split `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs`

**Current LOC:** 320  
**Role:** command lifecycle state tracker  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
4.tooling/l6.1-command-envelopes/src/command_lifecycle/
  mod.rs
  tracker.rs
  state.rs
  transitions.rs
  observations.rs
  timing.rs
  errors.rs
```

### Required rules

- `tracker.rs` should only own tracker orchestration.
- State enum goes to `state.rs`.
- Transition validation goes to `transitions.rs`.
- Observation publication goes to `observations.rs`.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 20. Split `6.apps/editor/stratumx_editor_app/src/desktop_app/editor_app.rs`

**Current LOC:** 316  
**Role:** editor desktop app host  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `6.apps/editor/stratumx_editor_app/src/desktop_app/editor_app.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
6.apps/editor/stratumx_editor_app/src/desktop_app/
  mod.rs
  editor_app.rs
  app_state.rs
  app_update.rs
  app_panels.rs
  app_viewport.rs
  app_commands.rs
  app_diagnostics.rs
```

### Required rules

- `6.apps` remains host glue only.
- If logic belongs to editor product, move it into `5.editor`.
- `editor_app.rs` should mostly contain the trait impl and call small helper modules.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 21. Split `5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/types.rs`

**Current LOC:** 253  
**Role:** editor bootstrap type aggregation  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/types.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/
  mod.rs
  project_descriptor.rs
  workspace_descriptor.rs
  startup_profile.rs
  bootstrap_request.rs
  bootstrap_result.rs
  bootstrap_errors.rs
```

### Required rules

- Keep only pure type definitions.
- Do not add bootstrap execution logic here.
- Existing public type names must remain re-exported.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 22. Split `5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service.rs`

**Current LOC:** 246  
**Role:** material authoring service orchestration  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/
  mod.rs
  service.rs
  material_session.rs
  preview_request.rs
  validation.rs
  diagnostics.rs
  cache.rs
```

### Required rules

- Service orchestration stays in `service.rs`.
- Preview request building moves to `preview_request.rs`.
- Validation moves to `validation.rs`.
- Diagnostics formatting moves to `diagnostics.rs`.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

## Task 23. Split `5.editor/l8.10-diagnostics-surface/src/diagnostics_types.rs`

**Current LOC:** 241  
**Role:** diagnostics type aggregation  
**Goal:** reduce this file below 200-230 LOC without changing public behavior.

### Exact action

1. Open `5.editor/l8.10-diagnostics-surface/src/diagnostics_types.rs`.
2. Identify all public structs, enums, functions, constants and tests.
3. Create the target module layout below.
4. Move code by responsibility only. Do not move random line ranges.
5. Preserve public imports by adding `pub use` in the local `mod.rs`.
6. Do not change behavior in this task.
7. If a test currently lives in this file, move it into `7.quality` unless it is a tiny pure-unit invariant approved by this roadmap.
8. Run:
   ```powershell
   cargo fmt --all
   cargo check -p <package-name>
   ```
9. Then run the layer-level gate.

### Target layout

```text
5.editor/l8.10-diagnostics-surface/src/diagnostics/
  mod.rs
  ids.rs
  severity.rs
  events.rs
  panels.rs
  recovery.rs
  packets.rs
```

### Required rules

- Diagnostics surface may define view models, not engine truth.
- Keep severity/event IDs stable.
- Update imports through re-export module.

### Done when

- Original file is gone or becomes a tiny `mod.rs`.
- Every new file is under 230 LOC.
- Package compiles.
- Public API remains reachable through the same module path when possible.
- No new layer dependencies are added.

# 10. Exact command gate function mapping

Move functions from `3.sdk/l5.9-legality-gates/src/command_gates.rs` exactly as below.

### SCENE COMMAND GATES → `src/command_gates/scene.rs`

- move `validate_scene_create_empty`
- move `validate_scene_create_entity_from_asset`
- move `validate_scene_set_transform`
- move `validate_scene_delete_entity`
- move `validate_scene_get_entity_details`

### MATERIAL COMMAND GATES → `src/command_gates/material.rs`

- move `validate_material_create_archetype`
- move `validate_material_create_stack`
- move `validate_material_stack_add_layer`
- move `validate_material_stack_remove_layer`
- move `validate_material_assign_stack`

### TERRAIN COMMAND GATES → `src/command_gates/terrain.rs`

- move `validate_terrain_create_patch`
- move `validate_terrain_paint_surface`

### ACTOR COMMAND GATES → `src/command_gates/actor.rs`

- move `validate_actor_spawn_preset`
- move `validate_actor_attach_weapon`
- move `validate_actor_set_active`

### ASSET COMMAND GATES → `src/command_gates/asset.rs`

- move `validate_asset_import`
- move `validate_asset_get_details`

### SKY/WEATHER COMMAND GATES → `src/command_gates/sky_weather.rs`

- move `validate_sky_set_time_of_day`
- move `validate_sky_set_day_of_year`
- move `validate_sky_set_latitude`
- move `validate_sky_set_normalized_value`
- move `validate_sky_set_rain`
- move `validate_sky_step_simulation`
- move `validate_storm_create`
- move `validate_storm_update`

### BALLISTICS COMMAND GATES → `src/command_gates/ballistics.rs`

- move `validate_ballistics_fire_active_actor`

### DESTRUCTION COMMAND GATES → `src/command_gates/destruction.rs`

- move `validate_destruction_set_terrain_material`
- move `validate_destruction_trigger_blast`
- move `validate_destruction_set_integrity`
- move `validate_destruction_set_support_type`
- move `validate_destruction_apply_support_damage`
- move `validate_destruction_reset_state`

### MATERIAL WORLD COMMAND GATES → `src/command_gates/material_world.rs`

- move `validate_material_world_set_barrel_water`
- move `validate_material_world_set_barrel_leak`
- move `validate_material_world_ignite_fire`
- move `validate_material_world_extinguish_fire`
- move `validate_material_world_set_wetness`
- move `validate_material_world_update`

### NAV/DOOR/INVENTORY COMMAND GATES → `src/command_gates/nav_door_inventory.rs`

- move `validate_nav_door_set_blocked`
- move `validate_nav_set_path`
- move `validate_inventory_add_item`
- move `validate_inventory_remove_item`
- move `validate_inventory_transfer`
- move `validate_inventory_equip_weapon`
- move `validate_world_load_state`
- move `validate_region_request`
- move `validate_region_complete_load`

### POPULATION/NPC COMMAND GATES → `src/command_gates/population_npc.rs`

- move `validate_npc_create_profile`
- move `validate_npc_trait_value`
- move `validate_npc_set_need`
- move `validate_npc_set_activity`
- move `validate_scarcity_increase`
- move `validate_npc_set_faction`

### TACTICS COMMAND GATES → `src/command_gates/tactics.rs`

- move `validate_tactics_create_squad`
- move `validate_tactics_set_cover`
- move `validate_tactics_invalidate_cover`
- move `validate_tactics_check_cover_valid`

### ECOLOGY COMMAND GATES → `src/command_gates/ecology.rs`

- move `validate_ecology_create_creature`
- move `validate_ecology_set_creature_state`

### REASON CHAIN COMMAND GATES → `src/command_gates/reason_chain.rs`

- move `validate_reason_chain_inspect_npc`

### VERTICAL SLICE COMMAND GATES → `src/command_gates/vertical_slice.rs`

- move `validate_vertical_slice_bootstrap_scene`
- move `validate_vertical_slice_reset_scene`
- move `validate_vertical_slice_fire_test_shot`
- move `validate_vertical_slice_assign_material`
- move `validate_vertical_slice_select_entity`

### ANIMATION COMMAND GATES → `src/command_gates/animation.rs`

- move `validate_animation_play_clip`
- move `validate_animation_stop_clip`
- move `validate_animation_set_speed`
- move `validate_animation_set_weight`
- move `validate_animation_set_ik_target`
- move `validate_animation_load_clip`
- move `validate_animation_create_state_machine`
- move `validate_animation_trigger_event`

## 10.1 Required command gate `mod.rs`

Create:

```text
3.sdk/l5.9-legality-gates/src/command_gates/mod.rs
```

Insert:

```rust
//! Command-specific legality gates split by domain.

pub mod common;
pub mod scene;
pub mod material;
pub mod terrain;
pub mod actor;
pub mod asset;
pub mod sky_weather;
pub mod ballistics;
pub mod destruction;
pub mod material_world;
pub mod nav_door_inventory;
pub mod population_npc;
pub mod tactics;
pub mod ecology;
pub mod reason_chain;
pub mod vertical_slice;
pub mod animation;

pub use scene::*;
pub use material::*;
pub use terrain::*;
pub use actor::*;
pub use asset::*;
pub use sky_weather::*;
pub use ballistics::*;
pub use destruction::*;
pub use material_world::*;
pub use nav_door_inventory::*;
pub use population_npc::*;
pub use tactics::*;
pub use ecology::*;
pub use reason_chain::*;
pub use vertical_slice::*;
pub use animation::*;
```

## 10.2 Safe command-gate migration sequence

```powershell
Copy-Item 3.sdk/l5.9-legality-gates/src/command_gates.rs _cleanup_scratch/command_gates.rs.bak
New-Item -ItemType Directory 3.sdk/l5.9-legality-gates/src/command_gates_new
```

Create all split files in `command_gates_new`.

Then:

```powershell
Remove-Item 3.sdk/l5.9-legality-gates/src/command_gates.rs
Rename-Item 3.sdk/l5.9-legality-gates/src/command_gates_new command_gates
cargo fmt --all
cargo check -p legality_gates
```

# 11. Editor active product spine

Create:

```text
5.editor/ACTIVE_EDITOR_PRODUCT_SPINE.md
```

Insert:

```markdown
# Active Editor Product Spine

## Active product crates

- `l7.0-editor-command-spine`
- `l8.0-editor-shell`
- `l8.1-viewport-system`
- `l8.5-tool-context-system`
- `l8.10-diagnostics-surface`
- `l9.0-world-authoring-suite`
- `l9.2-terrain-landscape-authoring-suite`
- `l9.3-material-lookdev-authoring-suite`
- `l9.6-weather-environment-authoring-suite`
- `l10.0-project-bootstrap-service`

## Rule

Future surfaces may compile, but they must not be counted as implemented editor product features until wired into this spine.
```

Check `6.apps/editor/stratumx_editor_app/Cargo.toml`.

It should depend on active spine crates only, unless a future surface is genuinely visible and functional.

# 12. Layer boundary rules

Allowed direction:

```text
7.quality -> all
6.apps -> 5.editor, 4.tooling, 3.sdk, 2.engine
5.editor -> 4.tooling, 3.sdk, 2.engine
4.tooling -> 3.sdk, 2.engine
3.sdk -> 2.engine only where explicitly allowed
2.engine -> no upper layer
```

Forbidden:

```text
2.engine -> 3.sdk
2.engine -> 4.tooling
2.engine -> 5.editor
3.sdk -> 4.tooling
3.sdk -> 5.editor
4.tooling -> 5.editor
5.editor -> 6.apps
```

Run:

```powershell
.\tools\check-layer-boundaries.ps1
```

# 13. Code-vs-canon status ledger

Create:

```text
1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md
```

Insert:

```markdown
# Code vs Canon Status Ledger

Stack: SX-CANON/1.0.28/STACK-v34

| Domain | Docs readiness | Code readiness | Status | Next implementation after cleanup |
|---|---:|---:|---|---|
| Graphics / Native Graphics Port | 92% | 40% | docs ahead | implement graphics port core/null/vulkan |
| Image Output / Showable Frame | 90% | 40% | docs ahead | framegraph + terrain/sky frame |
| Physics / Simulation | 88% | 50% | partial | substrate + terrain/destruction/hydrology runtime |
| Materials | 92% | 58% | partial | `.sxmat`, profile registry, shader key builder |
| Asset / DCC Pipeline | 88% | 30% | docs ahead | import quarantine + cook packages |
| Audio | 82% | 30% | docs ahead | event graph + bank + mixer seed |
| Netcode | 75% | 15% | early | authority + replication skeleton |
| Editor | 88% | 60% | partial | stitch active product spine |
| SDK | 85% | 78% | strong but monolithic | split command gates |
| Tooling | 85% | 73% | strong but needs split | recovery/commands/lifecycle split |
| Apps | 75% | 84% | near gold | split editor host file |
| Quality | 90% | 90% | strong | keep gates honest |
```

Add README link to this ledger.

# 14. Execution order

## A. Zero-risk cleanup

1. Update README.
2. Clean root.
3. Create cleanup status ledger.
4. Delete placeholder tests.
5. Create crate status ledger.
6. Classify orphan/stub crates.

Run:

```powershell
cargo fmt --all
cargo check --workspace
.\tools\doctor.ps1
```

## B. SDK cleanup

1. Split `command_gates.rs`.
2. Split `editor_authoring_observations/types.rs`.
3. Move tests.

Run:

```powershell
cargo fmt --all
cargo check -p legality_gates
cargo test --workspace
```

## C. Engine substrate cleanup

1. Split `runtime.rs`.
2. Split `types.rs`.
3. Split `queries.rs`.
4. Split `validation.rs`.

Run:

```powershell
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

## D. Tooling cleanup

1. Split recovery policies.
2. Split promoted command files.
3. Split command lifecycle tracker.

Run:

```powershell
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

## E. Editor and app cleanup

1. Split bootstrap types.
2. Split material authoring service.
3. Split diagnostics types.
4. Split editor app host.
5. Create active editor spine doc.

Run:

```powershell
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

## F. Final gates

```powershell
cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
.\tools\check-layer-boundaries.ps1
.\tools\check-test-placement.ps1
.\tools\check-file-size-discipline.ps1
.\tools\validate-workspace.ps1
.\tools\doctor.ps1
.\tools\verify.ps1
.\tools\full.ps1
```

# 15. File-size policy

Hard policy:

```text
<= 200 LOC: good
201-300 LOC: warning, split if touched
> 300 LOC: split required
> 500 LOC: urgent split required
```

Never split as:

```text
part1.rs
part2.rs
misc.rs
stuff.rs
helpers.rs
```

Always split by role:

```text
ids.rs
types.rs
state.rs
queries.rs
validation.rs
errors.rs
diagnostics.rs
serialization.rs
runtime.rs
persistence.rs
publication.rs
```

# 16. Templates

## 16.1 Module re-export template

```rust
//! Module description.

pub mod ids;
pub mod types;
pub mod validation;
pub mod errors;

pub use ids::*;
pub use types::*;
pub use validation::*;
pub use errors::*;
```

## 16.2 FUTURE_STUB header

```rust
//! FUTURE_STUB.
//!
//! This crate is part of the canonical future surface but is not wired into the
//! active product spine yet. It must not be counted as product-complete.
```

## 16.3 Cleanup ledger entry

```markdown
| Date | Action | Files | Reason |
|---|---|---|---|
```

## 16.4 Split ledger entry

```markdown
| Date | Split | Old file | New modules | Public API preserved |
|---|---|---|---|---|
```

# 17. Appendix — Full production monolith table
| File | LOC | Required action |
|---|---:|---|
| `3.sdk/l5.9-legality-gates/src/command_gates.rs` | 1534 | split now |
| `3.sdk/l5.2-link-egress-observations/src/editor_authoring_observations/types.rs` | 519 | split now |
| `4.tooling/l6.0-authority-core/src/recovery/policies.rs` | 371 | split now |
| `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs` | 362 | split now |
| `2.engine/l0.5-shared-world-property-substrate/src/types.rs` | 351 | split now |
| `4.tooling/l6.1-command-envelopes/src/promoted_commands/types.rs` | 329 | split now |
| `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs` | 320 | split now |
| `6.apps/editor/stratumx_editor_app/src/desktop_app/editor_app.rs` | 316 | split now |
| `2.engine/l0.5-shared-world-property-substrate/src/queries.rs` | 299 | split if touched / justify |
| `4.tooling/l6.0-authority-core/src/conveyor.rs` | 290 | split if touched / justify |
| `2.engine/l0.5-shared-world-property-substrate/src/validation.rs` | 266 | split if touched / justify |
| `3.sdk/l5.0-link-ingress-packets/src/editor_authoring_ingress/diagnostics/types.rs` | 259 | split if touched / justify |
| `5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/types.rs` | 253 | split if touched / justify |
| `5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service.rs` | 246 | split if touched / justify |
| `4.tooling/l6.1-command-envelopes/src/promoted_commands/builders.rs` | 243 | split if touched / justify |
| `5.editor/l8.10-diagnostics-surface/src/diagnostics_types.rs` | 241 | split if touched / justify |
| `5.editor/l9.0-world-authoring-suite/src/world_session_service.rs` | 234 | split if touched / justify |
| `4.tooling/l6.1-command-envelopes/src/validation/rules.rs` | 230 | split if touched / justify |
| `5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view.rs` | 229 | split if touched / justify |
| `5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries.rs` | 224 | split if touched / justify |
| `4.tooling/l6.1-command-envelopes/src/validation/validators.rs` | 223 | split if touched / justify |
| `4.tooling/l6.0-authority-core/src/containers/audio/mod.rs` | 212 | split if touched / justify |
| `5.editor/l8.0-editor-shell/src/command_palette_state.rs` | 209 | split if touched / justify |
| `5.editor/l8.0-editor-shell/src/editor_shell.rs` | 206 | split if touched / justify |
| `5.editor/l9.6-weather-environment-authoring-suite/src/runtime/authoring_service.rs` | 205 | split if touched / justify |
| `5.editor/l9.2-terrain-landscape-authoring-suite/src/desktop/terrain_panel.rs` | 201 | split if touched / justify |
| `4.tooling/l6.14-release-runtime/src/first_result_verification/evidence.rs` | 201 | split if touched / justify |

# 18. Appendix — All detected outside-quality tests
| File | LOC | Tests | cfg(test) | Required action |
|---|---:|---:|---:|---|
| `2.engine/l0.5-shared-world-property-substrate/src/queries.rs` | 299 | 7 | 1 | move/delete/justify |
| `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs` | 362 | 4 | 1 | move/delete/justify |
| `2.engine/l0.5-shared-world-property-substrate/src/types.rs` | 351 | 4 | 1 | move/delete/justify |
| `2.engine/l0.5-shared-world-property-substrate/src/validation.rs` | 266 | 5 | 1 | move/delete/justify |
| `4.tooling/l6.0-authority-core/src/containers/audio/mod.rs` | 212 | 5 | 1 | move/delete/justify |
| `4.tooling/l6.0-authority-core/src/containers/material_authority_container.rs` | 138 | 2 | 1 | move/delete/justify |
| `4.tooling/l6.0-authority-core/src/conveyor.rs` | 290 | 4 | 1 | move/delete/justify |
| `4.tooling/l6.0-authority-core/src/diagnostics.rs` | 175 | 3 | 1 | move/delete/justify |
| `4.tooling/l6.0-authority-core/src/layer_purity.rs` | 75 | 2 | 1 | move/delete/justify |
| `4.tooling/l6.0-authority-core/src/recovery/policies.rs` | 371 | 7 | 2 | move/delete/justify |
| `4.tooling/l6.0-authority-core/src/recovery/strategies.rs` | 80 | 1 | 1 | move/delete/justify |
| `4.tooling/l6.1-command-envelopes/src/canonical_command_schema/mod.rs` | 71 | 2 | 1 | move/delete/justify |
| `4.tooling/l6.1-command-envelopes/src/command_lifecycle/stages.rs` | 96 | 3 | 1 | move/delete/justify |
| `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs` | 320 | 7 | 2 | move/delete/justify |
| `4.tooling/l6.1-command-envelopes/src/promoted_commands/builders.rs` | 243 | 3 | 1 | move/delete/justify |
| `4.tooling/l6.1-command-envelopes/src/validation/errors.rs` | 50 | 1 | 1 | move/delete/justify |
| `4.tooling/l6.1-command-envelopes/src/validation/rules.rs` | 230 | 3 | 1 | move/delete/justify |
| `4.tooling/l6.14-release-runtime/src/first_result_verification/checks.rs` | 122 | 3 | 1 | move/delete/justify |
| `4.tooling/l6.14-release-runtime/src/first_result_verification/evidence.rs` | 201 | 2 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_context.rs` | 179 | 0 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_dispatch_core.rs` | 69 | 2 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_enums.rs` | 196 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_execution.rs` | 92 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_focus.rs` | 175 | 9 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_ids.rs` | 76 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_preconditions.rs` | 96 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_queries.rs` | 74 | 1 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_registration/mod.rs` | 57 | 0 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_registration/tests.rs` | 10 | 1 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_registry.rs` | 170 | 4 | 8 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_registry_types.rs` | 129 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_validation/mod.rs` | 37 | 0 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/action_validation/tests.rs` | 10 | 1 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/canonical_action_api.rs` | 147 | 2 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/dispatch_audio.rs` | 141 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/dispatch_material.rs` | 144 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/editor_command_spine.rs` | 127 | 3 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/focus_recovery.rs` | 73 | 4 | 1 | move/delete/justify |
| `5.editor/l7.0-editor-command-spine/src/focus_routing.rs` | 189 | 8 | 1 | move/delete/justify |
| `5.editor/l8.0-editor-shell/src/command_palette_state.rs` | 209 | 2 | 1 | move/delete/justify |
| `5.editor/l8.0-editor-shell/src/command_spine/spine_lifecycle.rs` | 101 | 4 | 1 | move/delete/justify |
| `5.editor/l8.10-diagnostics-surface/src/diagnostics_cache.rs` | 110 | 1 | 1 | move/delete/justify |
| `5.editor/l8.3-content-browser-system/src/content_browser_cache.rs` | 102 | 1 | 1 | move/delete/justify |
| `5.editor/l8.5-tool-context-system/src/session_state/mod.rs` | 60 | 4 | 1 | move/delete/justify |
| `5.editor/l8.5-tool-context-system/src/state_graph/mod.rs` | 47 | 0 | 1 | move/delete/justify |
| `5.editor/l8.5-tool-context-system/src/state_queries.rs` | 181 | 3 | 1 | move/delete/justify |
| `5.editor/l8.7-workspace-layout-system/src/persistence/workspace_persistence_view.rs` | 180 | 5 | 1 | move/delete/justify |
| `5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries.rs` | 224 | 6 | 1 | move/delete/justify |
| `5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view.rs` | 229 | 5 | 1 | move/delete/justify |
| `5.editor/l9.0-world-authoring-suite/src/world_session_service.rs` | 234 | 4 | 1 | move/delete/justify |
| `5.editor/l9.3-material-lookdev-authoring-suite/src/material_cache.rs` | 114 | 2 | 1 | move/delete/justify |

# 19. Appendix — Orphan editor crates
| Package | Path | Recommended status |
|---|---|---|
| `stratumx-editor-l10-0-project-bootstrap-service` | `5.editor/l10.0-project-bootstrap-service` | FUTURE_STUB |
| `stratumx-editor-l10-1-import-export-pipeline-service` | `5.editor/l10.1-import-export-pipeline-service` | FUTURE_STUB |
| `stratumx-editor-l10-2-graph-authoring-service` | `5.editor/l10.2-graph-authoring-service` | FUTURE_STUB |
| `stratumx-editor-l10-3-automation-and-batch-service` | `5.editor/l10.3-automation-and-batch-service` | FUTURE_STUB |
| `stratumx-editor-l10-4-script-and-hot-reload-service` | `5.editor/l10.4-script-and-hot-reload-service` | FUTURE_STUB |
| `stratumx-editor-l10-5-plugin-and-extension-host` | `5.editor/l10.5-plugin-and-extension-host` | FUTURE_STUB |
| `stratumx-editor-l10-6-template-preset-and-scaffold-service` | `5.editor/l10.6-template-preset-and-scaffold-service` | FUTURE_STUB |
| `stratumx-editor-l10-7-package-market-and-dependency-service` | `5.editor/l10.7-package-market-and-dependency-service` | FUTURE_STUB |
| `stratumx-editor-l11-0-collaboration-session-surface` | `5.editor/l11.0-collaboration-session-surface` | FUTURE_STUB |
| `stratumx-editor-l11-1-review-annotation-surface` | `5.editor/l11.1-review-annotation-surface` | FUTURE_STUB |
| `stratumx-editor-l11-2-asset-gate-and-approval-surface` | `5.editor/l11.2-asset-gate-and-approval-surface` | FUTURE_STUB |
| `stratumx-editor-l11-3-playtest-and-capture-operations` | `5.editor/l11.3-playtest-and-capture-operations` | FUTURE_STUB |
| `stratumx-editor-l11-4-production-dashboard-and-traceability` | `5.editor/l11.4-production-dashboard-and-traceability` | FUTURE_STUB |
| `stratumx-editor-l11-5-learning-onboarding-and-help-surface` | `5.editor/l11.5-learning-onboarding-and-help-surface` | FUTURE_STUB |
| `stratumx-editor-l8-11-build-release-surface` | `5.editor/l8.11-build-release-surface` | FUTURE_STUB |
| `stratumx-editor-l8-2-outliner-system` | `5.editor/l8.2-outliner-system` | FUTURE_STUB |
| `stratumx-editor-l8-3-content-browser-system` | `5.editor/l8.3-content-browser-system` | FUTURE_STUB |
| `stratumx-editor-l8-4-inspector-system` | `5.editor/l8.4-inspector-system` | FUTURE_STUB |
| `stratumx-editor-l8-6-overlay-and-gizmo-system` | `5.editor/l8.6-overlay-and-gizmo-system` | FUTURE_STUB |
| `stratumx-editor-l8-7-workspace-layout-system` | `5.editor/l8.7-workspace-layout-system` | FUTURE_STUB |
| `stratumx-editor-l8-8-interaction-routing-system` | `5.editor/l8.8-interaction-routing-system` | FUTURE_STUB |
| `stratumx-editor-l8-9-assistant-surface` | `5.editor/l8.9-assistant-surface` | FUTURE_STUB |
| `stratumx-editor-l9-1-scene-entity-authoring-suite` | `5.editor/l9.1-scene-entity-authoring-suite` | FUTURE_STUB |
| `stratumx-editor-l9-10-quest-event-logic-authoring-suite` | `5.editor/l9.10-quest-event-logic-authoring-suite` | FUTURE_STUB |
| `stratumx-editor-l9-11-build-validation-release-suite` | `5.editor/l9.11-build-validation-release-suite` | FUTURE_STUB |
| `stratumx-editor-l9-4-destruction-fracture-authoring-suite` | `5.editor/l9.4-destruction-fracture-authoring-suite` | FUTURE_STUB |
| `stratumx-editor-l9-5-simulation-ai-authoring-suite` | `5.editor/l9.5-simulation-ai-authoring-suite` | FUTURE_STUB |
| `stratumx-editor-l9-7-animation-cinematics-authoring-suite` | `5.editor/l9.7-animation-cinematics-authoring-suite` | FUTURE_STUB |
| `stratumx-editor-l9-8-audio-voice-authoring-suite` | `5.editor/l9.8-audio-voice-authoring-suite` | FUTURE_STUB |
| `stratumx-editor-l9-9-ui-hud-authoring-suite` | `5.editor/l9.9-ui-hud-authoring-suite` | FUTURE_STUB |

# 20. Micro-runbooks for each monolith

## Micro-runbook 1: `3.sdk/l5.9-legality-gates/src/command_gates.rs`

1. Open `3.sdk/l5.9-legality-gates/src/command_gates.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 1534
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 2: `3.sdk/l5.2-link-egress-observations/src/editor_authoring_observations/types.rs`

1. Open `3.sdk/l5.2-link-egress-observations/src/editor_authoring_observations/types.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 519
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 3: `4.tooling/l6.0-authority-core/src/recovery/policies.rs`

1. Open `4.tooling/l6.0-authority-core/src/recovery/policies.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 371
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 4: `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs`

1. Open `2.engine/l0.5-shared-world-property-substrate/src/runtime.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 362
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 5: `2.engine/l0.5-shared-world-property-substrate/src/types.rs`

1. Open `2.engine/l0.5-shared-world-property-substrate/src/types.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 351
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 6: `4.tooling/l6.1-command-envelopes/src/promoted_commands/types.rs`

1. Open `4.tooling/l6.1-command-envelopes/src/promoted_commands/types.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 329
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 7: `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs`

1. Open `4.tooling/l6.1-command-envelopes/src/command_lifecycle/tracker.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 320
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 8: `6.apps/editor/stratumx_editor_app/src/desktop_app/editor_app.rs`

1. Open `6.apps/editor/stratumx_editor_app/src/desktop_app/editor_app.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 316
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 9: `2.engine/l0.5-shared-world-property-substrate/src/queries.rs`

1. Open `2.engine/l0.5-shared-world-property-substrate/src/queries.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 299
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 10: `4.tooling/l6.0-authority-core/src/conveyor.rs`

1. Open `4.tooling/l6.0-authority-core/src/conveyor.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 290
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 11: `2.engine/l0.5-shared-world-property-substrate/src/validation.rs`

1. Open `2.engine/l0.5-shared-world-property-substrate/src/validation.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 266
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 12: `3.sdk/l5.0-link-ingress-packets/src/editor_authoring_ingress/diagnostics/types.rs`

1. Open `3.sdk/l5.0-link-ingress-packets/src/editor_authoring_ingress/diagnostics/types.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 259
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 13: `5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/types.rs`

1. Open `5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/types.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 253
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 14: `5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service.rs`

1. Open `5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 246
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 15: `4.tooling/l6.1-command-envelopes/src/promoted_commands/builders.rs`

1. Open `4.tooling/l6.1-command-envelopes/src/promoted_commands/builders.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 243
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 16: `5.editor/l8.10-diagnostics-surface/src/diagnostics_types.rs`

1. Open `5.editor/l8.10-diagnostics-surface/src/diagnostics_types.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 241
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 17: `5.editor/l9.0-world-authoring-suite/src/world_session_service.rs`

1. Open `5.editor/l9.0-world-authoring-suite/src/world_session_service.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 234
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 18: `4.tooling/l6.1-command-envelopes/src/validation/rules.rs`

1. Open `4.tooling/l6.1-command-envelopes/src/validation/rules.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 230
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 19: `5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view.rs`

1. Open `5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 229
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 20: `5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries.rs`

1. Open `5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 224
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 21: `4.tooling/l6.1-command-envelopes/src/validation/validators.rs`

1. Open `4.tooling/l6.1-command-envelopes/src/validation/validators.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 223
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 22: `4.tooling/l6.0-authority-core/src/containers/audio/mod.rs`

1. Open `4.tooling/l6.0-authority-core/src/containers/audio/mod.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 212
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 23: `5.editor/l8.0-editor-shell/src/command_palette_state.rs`

1. Open `5.editor/l8.0-editor-shell/src/command_palette_state.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 209
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 24: `5.editor/l8.0-editor-shell/src/editor_shell.rs`

1. Open `5.editor/l8.0-editor-shell/src/editor_shell.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 206
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 25: `5.editor/l9.6-weather-environment-authoring-suite/src/runtime/authoring_service.rs`

1. Open `5.editor/l9.6-weather-environment-authoring-suite/src/runtime/authoring_service.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 205
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 26: `5.editor/l9.2-terrain-landscape-authoring-suite/src/desktop/terrain_panel.rs`

1. Open `5.editor/l9.2-terrain-landscape-authoring-suite/src/desktop/terrain_panel.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 201
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

## Micro-runbook 27: `4.tooling/l6.14-release-runtime/src/first_result_verification/evidence.rs`

1. Open `4.tooling/l6.14-release-runtime/src/first_result_verification/evidence.rs`.
2. List every public symbol.
3. Group symbols by responsibility.
4. Create directory module.
5. Create `mod.rs`.
6. Move symbols.
7. Add `pub use` re-exports.
8. Delete old file or reduce it to a tiny module entry.
9. Run `cargo fmt --all`.
10. Run `cargo check --workspace`.
11. Fix imports only.
12. Run `cargo test --workspace`.
13. Add cleanup ledger entry.

Target:

```text
old LOC: 201
new largest file: <= 230 LOC
behavior change: none
API break: none unless approved
```

# 21. Final gold-clean checklist

- [ ] README marker is `SX-CANON/1.0.28/STACK-v34`.
- [ ] Root has no garbage files.
- [ ] `CODE_CLEANUP_STATUS_LEDGER.md` exists.
- [ ] `CODE_VS_CANON_STATUS_LEDGER.md` exists.
- [ ] `crate_status_ledger.md` exists.
- [ ] `ACTIVE_EDITOR_PRODUCT_SPINE.md` exists.
- [ ] Every active crate has a status.
- [ ] Every orphan crate is classified.
- [ ] Every future stub has `FUTURE_STUB` header.
- [ ] No heavy tests outside `7.quality`.
- [ ] `command_gates.rs` is split.
- [ ] SDK observation enum aggregation is split.
- [ ] Engine substrate files are split.
- [ ] Tooling recovery/command files are split.
- [ ] Editor heavy files are split.
- [ ] App host file is split.
- [ ] No production file above 500 LOC.
- [ ] No production file above 300 LOC without explicit justification.
- [ ] Workspace validation passes.
- [ ] Layer boundary check passes.
- [ ] Test placement check passes.
- [ ] File-size check passes.
- [ ] `cargo fmt --all --check` passes.
- [ ] `cargo check --workspace` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `cargo test --workspace` passes.
- [ ] `tools/doctor` passes.
- [ ] `tools/verify` passes.
- [ ] `tools/full` passes.

# 22. Stop condition

When all checklist items pass, stop.

Do not start Native Graphics Port work in this branch.

Commit cleanup:

```powershell
git add .
git commit -m "chore: stabilize repository and split cleanup monoliths"
```

Then create the next branch:

```powershell
git checkout -b feature/native-graphics-port-seed
```

Only in that next branch implement:

```text
Native Graphics Port core
Null backend
Vulkan backend seed
showable terrain/sky frame
asset/material pipeline seed
```
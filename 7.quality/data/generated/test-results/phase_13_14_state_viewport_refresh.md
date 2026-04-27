# Phase 13-14: State Matrix and Viewport Refresh Closure

**Date:** 2026-04-12
**Status:** ✅ Targeted closure complete for state ownership gates and viewport sync runtime

## Scope

- Repaired `editor_state_matrix` so selection setup uses the public `SessionState::apply_modification(...)` path instead of private selection internals.
- Replaced the `l8.1-viewport-system` `refresh_from_tooling()` stub with real extraction/sync behavior.
- Added viewport scene binding, camera updates, last-valid snapshot fallback, and sync diagnostics.
- Added crate-local unit and property tests for viewport synchronization.

## Code Changes

- `4.tooling/l6.0-tool-session/src/tooling_tool_session/runtime.rs`
  - Added `queued_selection()` read API so viewport sync can read tooling selection truth without cracking private storage.
- `5.editor/l8.1-viewport-system/src/editor_viewport_system.rs`
  - Added `SceneId`, `CameraTransform`, and `ViewportData`.
  - Implemented `extract_viewport_data()`, `refresh_from_tooling()`, `bind_to_scene()`, `update_camera()`, and last-valid-state preservation on sync failure.
- `5.editor/l8.1-viewport-system/tests/property_tests.rs`
  - Added property coverage for selection sync and bound-scene extraction.
- `5.editor/l8.1-viewport-system/tests/unit_tests.rs`
  - Added unit coverage for scene binding validation, camera validation, real selection sync, and degraded refresh recovery.
- `7.quality/suites/editor_state_matrix/tests/state_query_immutability_tests.rs`
  - Routed selection setup through `StateModification::AddToSelection`.
- `7.quality/suites/editor_state_matrix/tests/property_05_state_container_authority.rs`
  - Routed selection authority checks through the same public mutation path.

## Verification

Executed successfully:

```text
cargo test -p stratumx-editor-state-containers --tests
cargo test -p stratumx-editor-l7-0-editor-command-spine --tests
cargo test -p stratumx-editor-l8-1-viewport-system --tests
cargo test -p editor_command_matrix
cargo test -p editor_state_matrix
cargo test -p editor_shell_matrix
cargo test -p editor_app_matrix
cargo test -p editor_canon_matrix
cargo test -p end_to_end_matrix
```

## Operator Note

What now works that did not work before:

- `editor_state_matrix` no longer depends on private selection mutators and now validates the public ownership surface end-to-end.
- `l8.1-viewport-system` now synchronizes from real tooling selection/object data instead of returning `Ok(())`.
- A bound viewport scene and camera transform now survive normal refreshes and degrade safely to the last valid snapshot on sync failure.

## Remaining Follow-Up

- `tasks.md` still has broader Phase 13 app-host cleanup work (`3.x`) open; the editor app still contains direct execution seams in `desktop_app/command_flush.rs` and `editor_host/command_execution.rs`.
- Phase 14 shell completion (`6.x`) remains larger than this targeted closure and still needs a dedicated pass.

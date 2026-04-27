# Phase 13-14: Shell Host Closure and App Drift Removal

**Date:** 2026-04-12  
**Status:** ✅ Targeted closure complete for shell runtime ownership, dialog flows, docking, and app-level shortcut cleanup

## Scope

- Identified shell-owned state that was still duplicated in `6.apps/editor/stratumx_editor_app`.
- Moved project/open-world dialog state and file dialog ownership into `l8.0-editor-shell`.
- Added stage-strip and docking-manager surfaces to the shell crate and wired the desktop app to use them.
- Closed the terrain layer material shortcut by routing the command through envelopes, tooling classification, tooling execution, and app host flush.

## Drift And Shortcut Cases Closed

- `AppState::project_wizard`, `AppState::show_open_world_dialog`, and `AppState::world_path_input`
  - Removed from the app crate and replaced with `ProjectDialogsState` owned by `l8.0-editor-shell`.
- App-local file dialog helpers in `desktop_app/file_dialogs.rs`
  - Removed and replaced with shell-owned `show_file_dialog(FileDialogKind)`.
- Project creation and world-open UI flows
  - Moved from app-local dialog state into shell-owned dialog builders that emit canonical `PromotedCommand`s.
- Terrain layer material apply
  - Added `PromotedCommand::TerrainSetLayerMaterial` route handling through `l6.1-command-envelopes`, `l6.0-tool-session`, and `desktop_app/command_flush.rs`.

## Code Changes

- `5.editor/l8.0-editor-shell/src/shell_runtime.rs`
  - Added shell-owned `ProjectDialogsState`, `StageStrip`, `DockingManager`, `show_file_dialog()`, and `show_project_dialog()` orchestration.
  - Switched workspace layout persistence to docking-manager-backed layout snapshots.
- `5.editor/l8.0-editor-shell/src/layout_persistence.rs`
  - Extended persisted layout with focused panel, dock positions, and docking configuration.
- `5.editor/l8.0-editor-shell/src/docking_manager.rs`
  - Added panel registration, visibility control, docking state, and drag/snap behavior.
- `5.editor/l8.0-editor-shell/src/stage_strip.rs`
  - Added stage definitions for `World`, `Terrain`, `Environment`, `Simulation`, and `Capture`.
- `5.editor/l8.0-editor-shell/src/project_dialogs.rs`
  - Added shell-owned project/open-world dialog state, validation, and command building.
- `5.editor/l8.0-editor-shell/src/file_dialogs.rs`
  - Added typed file dialog entry points and file-type filters.
- `5.editor/l8.0-editor-shell/tests/unit_tests.rs`
  - Added coverage for stage activation, project dialog command submission, docking snap behavior, and docking persistence.
- `5.editor/l8.0-editor-shell/tests/property_tests.rs`
  - Added layout geometry round-trip property coverage.
- `4.tooling/l6.1-command-envelopes/src/promoted_commands.rs`
  - Added `TerrainSetLayerMaterial` and canonical route id `route.terrain.layer.material.set.v1`.
- `4.tooling/l6.0-tool-session/src/routing/command_domain.rs`
  - Classified `TerrainSetLayerMaterial` under the terrain domain.
- `4.tooling/l6.0-tool-session/src/command_executor.rs`
  - Routed `TerrainSetLayerMaterial` to the terrain executor.
- `4.tooling/l6.0-tool-session/src/terrain/executor_terrain.rs`
  - Added execution support for `TerrainSetLayerMaterial`.
- `6.apps/editor/stratumx_editor_app/src/desktop_app/app_state.rs`
  - Removed app-local project/open-world dialog state.
- `6.apps/editor/stratumx_editor_app/src/desktop_app/project_wizard_panel.rs`
  - Switched to shell-owned dialog state and shell-owned command submission.
- `6.apps/editor/stratumx_editor_app/src/desktop_app/open_world_dialog.rs`
  - Switched to shell-owned dialog state and shell-owned command submission.
- `6.apps/editor/stratumx_editor_app/src/desktop_app/shell_renderer.rs`
  - Added stage-strip rendering from shell-owned stage definitions.
- `6.apps/editor/stratumx_editor_app/src/desktop_app/terrain_panel.rs`
  - Switched browse actions to shell-owned file dialogs.
- `6.apps/editor/stratumx_editor_app/src/desktop_app/mod.rs`
  - Removed the app-local `file_dialogs` module.

## Verification

Executed successfully:

```text
cargo test -p stratumx-editor-l8-0-editor-shell --tests
cargo test -p stratumx_editor_app --tests
cargo test -p editor_shell_matrix
cargo test -p editor_app_matrix
cargo test -p editor_canon_matrix
cargo test -p end_to_end_matrix
```

## Operator Note

What now works that did not work before:

- Shell-owned dialogs, stage navigation, docking state, and layout persistence now live in `l8.0-editor-shell` instead of being split across the desktop app.
- The desktop app no longer owns duplicate project/open-world dialog state or duplicate file-dialog helpers.
- The terrain layer material button now follows a legal promoted-command route instead of stopping at an app-only gap.

## Remaining Follow-Up

- Later phases `10+` remain open and still need their own implementation waves.
- Legitimate app-local state still remains where it is composition-specific by design: GPU texture handles, egui-only camera/controller state, and presentation caches.

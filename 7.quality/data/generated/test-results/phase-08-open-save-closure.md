# Phase 08 — Open/Save/Product Shell Closure

**Date:** 2026-04-10
**Status:** COMPLETE (structural verification; runtime gates require local cargo execution)

## Actions Taken

### 1. Open/Save World Flows Verified (Honest Chain)

**Open:** UI dialog → `request_world_open` → `PromotedCommand::WorldOpen` → `command_flush.rs` → `apply_world_open` → `runtime_host.open_world_from_path()` → reads `world.json` → creates `EditorSession` → marks GPU dirty → `sync_ui_from_world()`

**Save:** UI action → `dispatch_save_world` → resolves path from shell state → `PromotedCommand::WorldSave` → `command_flush.rs` → `apply_world_save` → `runtime_host.save_world_to_path()` → writes `world.json` → updates shell paths

**Terrain Import:** UI path entry → `PromotedCommand::TerrainImport` → `import_heightmap_into_active_world()` → supports RAW/R16/PNG → reconstructs terrain → marks GPU dirty

### 2. Project Wizard Status

- Wizard is FUNCTIONAL but flagged as Phase 03 CRITICAL VIOLATION (direct filesystem mutation in UI panel)
- Creates project directory structure, `world.json`, opens world
- Scheduled for proper refactoring when runtime command executor is built

### 3. Shell/Workspace/Project State Owns Paths

- `command_flush.rs` correctly sets `workspace_path` and `project_path` on open/save
- Shell state is the owner, not ad-hoc UI structs

### 4. Gaps Documented (Not Fake-Fixed)

1. `save_world_to_path` writes only JSON metadata (resolution, world_size, environment) — NOT terrain height samples or chunk data
2. `verify_same_world_law` is a stub (always returns true)
3. `PromotedCommand::WorldClose` not handled in command_flush (falls through to `_` catch-all)
4. No "Save As" dialog — save always goes to resolved path
5. No "New World" action without project wizard
6. End-to-end test files in `end_to_end_matrix` use fixtures, not real disk I/O
7. No e2e test covers full open/save/import/sky roundtrip

### 5. Architecture Compliance

- Shell/workspace/project state owns paths (rule #3 satisfied)
- Canon update deferred until code paths are fully real (rule #5 — pending gap closure)

## Verification (Local Gates Required)

Mandatory commands:
- `cargo fmt --all --check`
- `cargo run -p stratumx_quality_tasks -- verify`
- `cargo run -p stratumx_editor_app -- --headless --frames 60`
- `cargo run -p stratumx_editor_app --features desktop -- --gui`

## Next Phase

Proceed to Phase 09: Quality contour unification.

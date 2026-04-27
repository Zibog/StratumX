# Phase 00 — Baseline Capture and Freeze

**Date:** 2026-04-10
**Status:** COMPLETE

## Baseline Metrics

### Root File Count
- Total root items: 51 (directories + files)
- Root markdown files (junk): 31 status/progress/summary files
- Root markdown files to keep: README.md, STRATUMX_GOLD_ROADMAP_SUPERPLAYBOOK_V3.md (this playbook)

### Workspace Members
- Total workspace members in Cargo.toml: 118
- Quality suites: 27 directories under 7.quality/suites/
- Suites missing Cargo.toml: editor_app_matrix, end_to_end_matrix

### Production Rust Files
- Under src/ across workspace: 758+ files
- Over 200 lines: 65 files
- Over 300 lines: 32 files  
- Over 500 lines: 3 files

### Editor Launch
- Canonical launch: `cargo run -p stratumx_editor_app --features desktop -- --gui`
- Headless: `cargo run -p stratumx_editor_app -- --headless --frames 60`
- Validation: `cargo run -p stratumx_editor_app` (runs 3 frames)
- main.rs is clean: distinguishes --gui/--headless/default paths

### Quality Entry Point
- Single: `cargo run -p stratumx_quality_tasks -- verify`
- Smoke: `cargo run -p stratumx_quality_tasks -- smoke`

### State-Container Files Requiring Split (Phase 05)
- cached_state_queries.rs
- cache_layer.rs
- state_graph.rs
- state_container_system.rs
- rebuildable_caches.rs
- diagnostics_owner.rs
- project_owner.rs
- workspace_owner.rs
- world_owner.rs

### Desktop Surface Files Requiring Classification (Phase 03)
- 24 files under 6.apps/editor/stratumx_editor_app/src/desktop_app/
- 3 marked REVIEW_CAREFULLY: app_state.rs, editor_app.rs, shell_actions.rs, shell_renderer.rs, update_loop.rs
- 1 marked LOCK_AS_SINGLE_LOWER_BOUND: canonical_routes.rs

### Tests-in-Source
- Zero tests in src/ directories (correct by policy)

### Known Issues at Baseline
1. Root has 31 progress/status markdown files polluting the root
2. editor_app_matrix suite has no Cargo.toml
3. end_to_end_matrix suite has no Cargo.toml
4. vertical_slice_quality_gates fate undecided
5. Several state-container files are too large or mix roles

## Commands Recorded

### Format Check
```
cargo fmt --all --check
```

### Verify
```
cargo run -p stratumx_quality_tasks -- verify
```

### Smoke
```
cargo run -p stratumx_quality_tasks -- smoke
```

### Editor Headless
```
cargo run -p stratumx_editor_app -- --headless --frames 60
```

### Editor GUI
```
cargo run -p stratumx_editor_app --features desktop -- --gui
```

## Freeze Declaration

Feature work is frozen until Phase 14 completes. Only cleanup, refactoring, and quality work proceeds.

No new fake truth, fake IDs, bypasses, or root junk will be introduced.

## Next Phase

Proceed to Phase 01: Root doc cleanup.

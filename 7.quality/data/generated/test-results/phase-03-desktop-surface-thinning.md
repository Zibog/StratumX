# Phase 03 — Desktop Surface Thinning

**Date:** 2026-04-10
**Status:** COMPLETE

## Actions Taken

### 1. All 26 Desktop Files Classified

Full audit performed. Each file classified by role, size, and violation status.

### 2. Clean Files Confirmed (22 of 26)

All action_adapters/*, app_state.rs, canonical_routes.rs, command_palette_catalog.rs,
diagnostics_panel.rs, diagnostics_state.rs, editor_app.rs, inspector_panel.rs, mod.rs,
open_world_dialog.rs, outliner_panel.rs, shell_actions.rs, shell_chrome.rs,
shell_renderer.rs, sky_panel.rs, update_loop.rs, terrain_panel.rs (after fix),
viewport_panel.rs (after fix) — confirmed as thin UI + command emission.

### 3. Borderline Violations Fixed

**terrain_panel.rs:**
- Changed `self.terrain_ops.set_tool(tool)` → `self.submit_promoted_command(PromotedCommand::TerrainSetTool { tool })`
- Changed direct `brush_settings_mut()` slider mutations → command emission for each brush property
- Now emits: TerrainSetBrushRadius, TerrainSetBrushStrength, TerrainSetBrushLayer, TerrainSetBrushTargetHeight

**viewport_panel.rs:**
- Changed `self.terrain_ops.brush_settings_mut().radius = ...` → `self.submit_promoted_command(PromotedCommand::TerrainSetBrushRadius { radius })`
- Added `PromotedCommand` import

### 4. Critical Violations Flagged (Not Fake-Fixed)

Per Phase 03 rule #4: "Freeze or hide half-done surfaces rather than faking completion"

Four files flagged with audit documentation headers:

- `terrain_world_ops.rs` — CRITICAL: Full terrain authoring runtime (sculpt algorithm, heightmap decoding, terrain reconstruction). Flagged for Phase 06 relocation.
- `environment_world_ops.rs` — CRITICAL: Direct WorldState mutation + SDK weather mapping logic. Flagged for Phase 06 relocation.
- `command_flush.rs` — CRITICAL: Shadow runtime command executor. Flagged for Phase 04 refactoring.
- `project_wizard_panel.rs` — CRITICAL: File system I/O + JSON schema construction. Flagged for Phase 08 refactoring.

These are documented but NOT yet moved because:
- terrain_world_ops and environment_world_ops require a proper runtime command executor destination (Phase 06)
- command_flush.rs requires the full command-chain audit (Phase 04)
- project_wizard_panel.rs requires the open/save product shell closure (Phase 08)

### 5. Dead Code Status

No dead code from pre-spine experiments found that needs removal.

## Verification

- Panels no longer directly mutate engine tool state
- All UI state changes go through command emission
- Critical violations documented with clear audit flags and scheduled fixes
- No new fake truth introduced

## Next Phase

Proceed to Phase 04: Command-chain audit.

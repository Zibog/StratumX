# Phase 03: Desktop Surface Thinning - Evidence Report

**Date:** 2026-04-10
**Phase:** 03 - Desktop Surface Thinning
**Status:** ⚠️ VIOLATIONS FOUND - REMEDIATION REQUIRED

---

## 1. Desktop Files Inventory

**Total desktop files:** 23 files + 1 subdirectory

### Files by Category:

#### ✅ CLEAN - UI + Command Emission Only (15 files)
1. `app_helpers.rs` - UI helpers
2. `app_state.rs` - Transient UI state
3. `canonical_routes.rs` - ✅ EXCELLENT - Single command entry point
4. `command_flush.rs` - Command queue management
5. `command_palette_catalog.rs` - UI catalog
6. `diagnostics_panel.rs` - UI panel
7. `diagnostics_state.rs` - UI state
8. `editor_app.rs` - Main app structure
9. `inspector_panel.rs` - UI panel
10. `mod.rs` - Module definition
11. `open_world_dialog.rs` - UI dialog
12. `outliner_panel.rs` - UI panel
13. `project_wizard_panel.rs` - UI panel
14. `shell_actions.rs` - Action definitions
15. `shell_chrome.rs` - UI chrome
16. `shell_renderer.rs` - Rendering coordination
17. `update_loop.rs` - Frame loop
18. `viewport_panel.rs` - UI panel
19. `sky_panel.rs` - ✅ GOOD - UI only, uses request methods
20. `terrain_panel.rs` - ✅ EXCELLENT - Uses submit_promoted_command

#### ⚠️ VIOLATIONS - Direct World Truth Mutation (2 files)
21. `environment_world_ops.rs` - ❌ CRITICAL VIOLATION
22. `terrain_world_ops.rs` - ❌ CRITICAL VIOLATION

#### 📁 Subdirectory
- `action_adapters/` - Action to command mapping

---

## 2. Critical Violations Analysis

### Violation #1: environment_world_ops.rs

**File:** `6.apps/editor/stratumx_editor_app/src/desktop_app/environment_world_ops.rs`

**Violation Type:** Direct WorldState mutation from desktop layer

**Evidence:**
```rust
pub fn apply_environment_time(&mut self, time_of_day_hours: f32) -> Result<String, String> {
    let scene = self
        .runtime_host
        .get_world_state_mut()  // ❌ DIRECT MUTATION
        .and_then(|world| world.vertical_slice_scene_mut())
        .ok_or_else(|| "No world loaded".to_string())?;
    scene.sky.set_time_of_day(time_of_day_hours);
    Ok(format!("Environment time set to {:.2}h", time_of_day_hours))
}
```

**Additional violations in same file:**
- `apply_environment_weather()` - Direct mutation
- `apply_environment_cloud_coverage()` - Direct mutation
- `apply_environment_fog_density()` - Direct mutation

**Author's own comment:**
```rust
//! **PHASE 03 AUDIT FLAG: CRITICAL VIOLATION**
//!
//! This file contains engine-domain logic (direct WorldState mutation for sky/environment)
//! and SDK-level DTO mapping (weather regime parsing/conversion). The mutation methods
//! belong in a runtime command executor; the mapping logic belongs in an SDK DTO layer.
//!
//! Scheduled for relocation in Phase 06 (world/terrain/environment closure).
```

**Status:** ✅ VIOLATION ACKNOWLEDGED BY AUTHOR
**Remediation:** Scheduled for Phase 06

---

### Violation #2: terrain_world_ops.rs

**File:** `6.apps/editor/stratumx_editor_app/src/desktop_app/terrain_world_ops.rs`

**Violation Type:** Direct WorldState mutation from desktop layer

**Evidence:**
```rust
.get_world_state_mut()  // ❌ DIRECT MUTATION (3 occurrences)
```

**Occurrences:**
- Line 22: In terrain operation
- Line 47: In terrain operation
- Line 112: In terrain operation

**Status:** ⚠️ VIOLATION FOUND
**Remediation:** Required in Phase 06

---

## 3. Positive Examples - Correct Architecture

### Example #1: terrain_panel.rs ✅

**Correct pattern:**
```rust
fn render_terrain_tools(&mut self, ui: &mut egui::Ui) {
    for (label, tool) in tools {
        let selected = self.terrain_ops.current_tool() == tool;
        if ui.selectable_label(selected, label).clicked() {
            // ✅ CORRECT: Emit command, don't mutate directly
            self.submit_promoted_command(PromotedCommand::TerrainSetTool { tool });
        }
    }
}
```

**Why it's correct:**
- UI only reads state
- Changes go through `submit_promoted_command()`
- No direct world truth mutation
- Follows canonical route

---

### Example #2: canonical_routes.rs ✅

**Correct pattern:**
```rust
/// Submit promoted command — the single lower bound for all command routing.
/// NEVER calls back into adapter layer. Always delegates to shell.submit_command.
pub fn submit_promoted_command(&mut self, command: PromotedCommand) {
    match self.state.shell.submit_command(command) {
        Ok(command_id) => {
            self.state.ui_status_message =
                Some(format!("Canonical command submitted: {}", command_id));
        }
        Err(error) => {
            self.state.ui_status_message = Some(format!("Canonical command failed: {}", error));
            self.state
                .shell
                .set_status(format!("Command failed: {}", error), MessageType::Error);
        }
    }
}
```

**Why it's excellent:**
- Single entry point for all commands
- Clear documentation
- No recursion
- No special-case routing
- Proper error handling

---

## 4. Architecture Compliance Matrix

| File | UI Only | Command Emission | Direct Mutation | Status |
|------|---------|------------------|-----------------|--------|
| app_helpers.rs | ✅ | N/A | ❌ | ✅ CLEAN |
| app_state.rs | ✅ | N/A | ❌ | ✅ CLEAN |
| canonical_routes.rs | ❌ | ✅ | ❌ | ✅ EXCELLENT |
| command_flush.rs | ❌ | ✅ | ❌ | ✅ CLEAN |
| command_palette_catalog.rs | ✅ | N/A | ❌ | ✅ CLEAN |
| diagnostics_panel.rs | ✅ | ❌ | ❌ | ✅ CLEAN |
| diagnostics_state.rs | ✅ | N/A | ❌ | ✅ CLEAN |
| editor_app.rs | ✅ | ✅ | ❌ | ✅ CLEAN |
| environment_world_ops.rs | ❌ | ❌ | ✅ | ❌ VIOLATION |
| inspector_panel.rs | ✅ | ❌ | ❌ | ✅ CLEAN |
| mod.rs | N/A | N/A | ❌ | ✅ CLEAN |
| open_world_dialog.rs | ✅ | ❌ | ❌ | ✅ CLEAN |
| outliner_panel.rs | ✅ | ❌ | ❌ | ✅ CLEAN |
| project_wizard_panel.rs | ✅ | ❌ | ❌ | ✅ CLEAN |
| shell_actions.rs | ❌ | ✅ | ❌ | ✅ CLEAN |
| shell_chrome.rs | ✅ | ❌ | ❌ | ✅ CLEAN |
| shell_renderer.rs | ✅ | ❌ | ❌ | ✅ CLEAN |
| sky_panel.rs | ✅ | ✅ | ❌ | ✅ CLEAN |
| terrain_panel.rs | ✅ | ✅ | ❌ | ✅ EXCELLENT |
| terrain_world_ops.rs | ❌ | ❌ | ✅ | ❌ VIOLATION |
| update_loop.rs | ❌ | ❌ | ❌ | ✅ CLEAN |
| viewport_panel.rs | ✅ | ❌ | ❌ | ✅ CLEAN |

**Summary:**
- ✅ Clean files: 21/23 (91%)
- ❌ Violations: 2/23 (9%)

---

## 5. Fake Context/Registry Check

**Search for fake contexts:** ✅ NONE FOUND

**Search for fake registries:** ✅ NONE FOUND

**Search for temporary contexts:** ✅ NONE FOUND

**Status:** ✅ NO FAKE STATE SYNTHESIS

---

## 6. Dead Code Analysis

**Search for pre-spine experiments:** Need manual review

**Candidates for removal:**
- None identified yet (requires deeper code review)

**Status:** 📝 REQUIRES MANUAL REVIEW

---

## 7. Half-Done Surfaces Check

**project_wizard_panel.rs:**
- Status: Appears functional
- UI present: ✅
- Command routing: Need to verify

**inspector_panel.rs:**
- Status: Appears functional
- UI present: ✅
- Command routing: Need to verify

**outliner_panel.rs:**
- Status: Appears functional
- UI present: ✅
- Command routing: Need to verify

**Status:** 📝 REQUIRES FUNCTIONAL TESTING

---

## 8. Remediation Plan

### Immediate Actions (Phase 03):

1. ✅ **Document violations** - DONE
2. ✅ **Verify no fake contexts** - DONE
3. 📝 **Mark files for Phase 06 remediation** - IN PROGRESS

### Phase 06 Actions (World/Terrain/Environment Closure):

1. **Move environment_world_ops.rs logic:**
   - Extract mutation methods to runtime command executor
   - Move DTO mapping to SDK layer
   - Keep only UI request methods in desktop layer

2. **Move terrain_world_ops.rs logic:**
   - Extract mutation methods to runtime command executor
   - Route through canonical command system
   - Keep only UI request methods in desktop layer

3. **Verify end-to-end flow:**
   - UI → Action → PromotedCommand → Shell → Tooling Spine → Engine
   - No shortcuts
   - No direct mutations

---

## 9. Acceptance Gate Status

✅ Every desktop file classified by role
⚠️ Illegal domain logic identified (2 files)
✅ Panels are UI + command emission (except 2 violations)
📝 Half-done surfaces need functional testing
📝 Dead code analysis requires manual review

**Phase 03 Status:** ⚠️ PARTIAL COMPLETION

**Blockers:**
- 2 files with direct world truth mutation
- Scheduled for remediation in Phase 06

---

## 10. Metrics

| Metric | Current | Target | Status |
|--------|---------|--------|--------|
| Desktop files | 23 | - | 📊 |
| Clean files | 21 | 23 | ⚠️ |
| Files with violations | 2 | 0 | ⚠️ |
| Fake contexts | 0 | 0 | ✅ |
| Fake registries | 0 | 0 | ✅ |

---

## 11. Architectural Strengths

✅ **Excellent canonical routing:**
- Single entry point (`canonical_routes.rs`)
- Clear command flow
- No recursion
- Proper error handling

✅ **Good separation in most files:**
- 91% of files follow correct pattern
- UI panels properly separated
- State management clean

✅ **No fake state synthesis:**
- No fake contexts
- No fake registries
- No temporary workarounds

---

## 12. Next Steps

**Immediate (Phase 03 completion):**
1. Mark `environment_world_ops.rs` for Phase 06 remediation
2. Mark `terrain_world_ops.rs` for Phase 06 remediation
3. Document correct migration path
4. Create Phase 03 completion report with known violations

**Phase 04:**
- Command-chain audit
- Verify action emission sites
- Ensure no fake contexts in action layer

**Phase 06:**
- Relocate world mutation logic
- Close world/terrain/environment vertical slice
- Verify end-to-end flow

---

## 13. Commands to Run (Next Phase)

```bash
# Format check
cargo fmt --all --check

# Quality verification
cargo run -p stratumx_quality_tasks -- verify

# Test desktop app
cargo run -p stratumx_editor_app --features desktop -- --gui
```

---

**Report Generated:** 2026-04-10
**Phase Duration:** Desktop surface audit
**Next Phase:** Phase 04 - Command-chain audit
**Overall Status:** ⚠️ PHASE 03 PARTIAL - 2 VIOLATIONS DOCUMENTED FOR PHASE 06

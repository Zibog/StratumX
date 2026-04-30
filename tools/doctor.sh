#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
expected_marker="$(tr -d '\r\n' < "$repo_root/1.docs/canonical/STACK_VERSION")"

check_orphan_crate_classification() {
  local ledger="$repo_root/7.quality/inventory/crate_status_ledger.md"
  [ -f "$ledger" ] || { echo "orphan crate classification failed: missing crate status ledger" >&2; return 1; }
  local paths=(
    "5.editor/l10.0-project-bootstrap-service"
    "5.editor/l10.1-import-export-pipeline-service"
    "5.editor/l10.2-graph-authoring-service"
    "5.editor/l10.3-automation-and-batch-service"
    "5.editor/l10.4-script-and-hot-reload-service"
    "5.editor/l10.5-plugin-and-extension-host"
    "5.editor/l10.6-template-preset-and-scaffold-service"
    "5.editor/l10.7-package-market-and-dependency-service"
    "5.editor/l11.0-collaboration-session-surface"
    "5.editor/l11.1-review-annotation-surface"
    "5.editor/l11.2-asset-gate-and-approval-surface"
    "5.editor/l11.3-playtest-and-capture-operations"
    "5.editor/l11.4-production-dashboard-and-traceability"
    "5.editor/l11.5-learning-onboarding-and-help-surface"
    "5.editor/l8.11-build-release-surface"
    "5.editor/l8.2-outliner-system"
    "5.editor/l8.3-content-browser-system"
    "5.editor/l8.4-inspector-system"
    "5.editor/l8.6-overlay-and-gizmo-system"
    "5.editor/l8.7-workspace-layout-system"
    "5.editor/l8.8-interaction-routing-system"
    "5.editor/l8.9-assistant-surface"
    "5.editor/l9.1-scene-entity-authoring-suite"
    "5.editor/l9.10-quest-event-logic-authoring-suite"
    "5.editor/l9.11-build-validation-release-suite"
    "5.editor/l9.4-destruction-fracture-authoring-suite"
    "5.editor/l9.5-simulation-ai-authoring-suite"
    "5.editor/l9.7-animation-cinematics-authoring-suite"
    "5.editor/l9.8-audio-voice-authoring-suite"
    "5.editor/l9.9-ui-hud-authoring-suite"
  )
  for path in "${paths[@]}"; do
    grep -F "| 5.editor | $path |" "$ledger" >/dev/null || {
      echo "orphan crate classification failed: missing ledger entry for $path" >&2
      return 1
    }
  done
  echo "OK: orphan crate classification"
}

check_stub_crate_classification() {
  local files=(
    "5.editor/l10.1-import-export-pipeline-service/src/editor_import_export_pipeline_service.rs"
    "5.editor/l10.2-graph-authoring-service/src/editor_graph_authoring_service.rs"
    "5.editor/l10.3-automation-and-batch-service/src/editor_automation_and_batch_service.rs"
    "5.editor/l10.4-script-and-hot-reload-service/src/editor_script_and_hot_reload_service.rs"
    "5.editor/l10.5-plugin-and-extension-host/src/editor_plugin_and_extension_host.rs"
    "5.editor/l10.6-template-preset-and-scaffold-service/src/editor_template_preset_and_scaffold_service.rs"
    "5.editor/l10.7-package-market-and-dependency-service/src/editor_package_market_and_dependency_service.rs"
    "5.editor/l11.0-collaboration-session-surface/src/editor_collaboration_session_surface.rs"
    "5.editor/l11.1-review-annotation-surface/src/editor_review_annotation_surface.rs"
    "5.editor/l11.2-asset-gate-and-approval-surface/src/editor_asset_gate_and_approval_surface.rs"
    "5.editor/l11.3-playtest-and-capture-operations/src/editor_playtest_and_capture_operations.rs"
    "5.editor/l11.4-production-dashboard-and-traceability/src/editor_production_dashboard_and_traceability.rs"
    "5.editor/l11.5-learning-onboarding-and-help-surface/src/editor_learning_onboarding_and_help_surface.rs"
    "5.editor/l8.11-build-release-surface/src/editor_build_release_surface.rs"
    "5.editor/l8.2-outliner-system/src/editor_outliner_system.rs"
    "5.editor/l8.3-content-browser-system/src/editor_content_browser_system.rs"
    "5.editor/l8.4-inspector-system/src/editor_inspector_system.rs"
    "5.editor/l8.6-overlay-and-gizmo-system/src/editor_overlay_and_gizmo_system.rs"
    "5.editor/l8.7-workspace-layout-system/src/editor_workspace_layout_system.rs"
    "5.editor/l8.8-interaction-routing-system/src/editor_interaction_routing_system.rs"
    "5.editor/l8.9-assistant-surface/src/editor_assistant_surface.rs"
    "5.editor/l9.1-scene-entity-authoring-suite/src/editor_scene_entity_authoring_suite.rs"
    "5.editor/l9.10-quest-event-logic-authoring-suite/src/editor_quest_event_logic_authoring_suite.rs"
    "5.editor/l9.11-build-validation-release-suite/src/editor_build_validation_release_suite.rs"
    "5.editor/l9.4-destruction-fracture-authoring-suite/src/lib.rs"
    "5.editor/l9.5-simulation-ai-authoring-suite/src/editor_simulation_ai_authoring_suite.rs"
    "5.editor/l9.7-animation-cinematics-authoring-suite/src/editor_animation_cinematics_authoring_suite.rs"
    "5.editor/l9.8-audio-voice-authoring-suite/src/lib.rs"
    "5.editor/l9.9-ui-hud-authoring-suite/src/editor_ui_hud_authoring_suite.rs"
  )
  for relative in "${files[@]}"; do
    local file="$repo_root/$relative"
    [ -f "$file" ] || { echo "stub crate classification failed: missing $relative" >&2; return 1; }
    local first_line
    first_line="$(head -n 1 "$file")"
    [ "$first_line" = "//! FUTURE_STUB." ] || {
      echo "stub crate classification failed: missing FUTURE_STUB header in $relative" >&2
      return 1
    }
  done
  echo "OK: stub crate classification"
}

check_root_garbage() {
  local allowed=(
    "Cargo.toml" "Cargo.lock" "README.md" ".gitignore" ".github" ".git"
    "1.docs" "2.engine" "3.sdk" "4.tooling" "5.editor" "6.apps"
    "7.quality" "9.assets" "tools" "target" "_cleanup_scratch" ".claude"
  )
  mapfile -t entries < <(find "$repo_root" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)
  for entry in "${entries[@]}"; do
    local seen=0
    for ok in "${allowed[@]}"; do
      if [ "$entry" = "$ok" ]; then
        seen=1
        break
      fi
    done
    if [ "$seen" -eq 0 ]; then
      echo "root garbage failed: unexpected root entry $entry" >&2
      return 1
    fi
  done
  [[ " ${entries[*]} " == *" _cleanup_scratch "* ]] && echo "WARN: root garbage includes temporary _cleanup_scratch"
  [[ " ${entries[*]} " == *" .claude "* ]] && echo "WARN: root garbage includes local .claude directory"
  echo "OK: root garbage"
}

check_docs_marker_consistency() {
  local files=(
    "$repo_root/README.md"
    "$repo_root/1.docs/developer_docs/CODE_CLEANUP_STATUS_LEDGER.md"
    "$repo_root/1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md"
  )
  for file in "${files[@]}"; do
    [ -f "$file" ] || { echo "docs marker consistency failed: missing $file" >&2; return 1; }
    grep -F "$expected_marker" "$file" >/dev/null || {
      echo "docs marker consistency failed: $file does not contain $expected_marker" >&2
      return 1
    }
  done
  echo "OK: docs marker consistency"
}

echo "=== StratumX Doctor - Health Check ==="
echo

echo "1. Workspace membership..."
bash "$repo_root/tools/validate-workspace.sh"
echo

echo "2. Layer dependencies..."
pwsh "$repo_root/tools/check-layer-boundaries.ps1"
echo

echo "3. Test placement..."
pwsh "$repo_root/tools/check-test-placement.ps1"
echo

echo "4. File size discipline..."
pwsh "$repo_root/tools/check-file-size-discipline.ps1"
echo

echo "5. Orphan crate classification..."
check_orphan_crate_classification
echo

echo "6. Stub crate classification..."
check_stub_crate_classification
echo

echo "7. Root garbage..."
check_root_garbage
echo

echo "8. Docs marker consistency..."
check_docs_marker_consistency
echo

echo "9. Workspace format gate..."
pwsh "$repo_root/tools/check-workspace-format.ps1"
echo

echo "10. cargo check --workspace..."
cargo check --workspace
echo

echo "11. cargo clippy --workspace --all-targets -- -D warnings..."
cargo clippy --workspace --all-targets -- -D warnings
echo

echo "12. cargo test --workspace..."
cargo test --workspace
echo

echo "13. Running verify..."
cargo run -p stratumx_quality_tasks -- verify
echo

echo "14. Running smoke..."
cargo run -p stratumx_quality_tasks -- smoke
echo

echo "=== All checks passed! ==="

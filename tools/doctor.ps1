$ErrorActionPreference = 'Stop'

function Invoke-Step {
    param(
        [string]$Title,
        [scriptblock]$Action
    )

    Write-Host $Title -ForegroundColor Cyan
    & $Action
    Write-Host ""
}

function Invoke-ExternalCheck {
    param(
        [string]$Title,
        [string]$Command,
        [string[]]$Arguments
    )

    Invoke-Step $Title {
        & $Command @Arguments
        if ($LASTEXITCODE -ne 0) {
            throw "$Title failed with exit code $LASTEXITCODE"
        }
    }
}

function Test-DocsMarkerConsistency {
    $expected = (Get-Content "$PSScriptRoot\..\1.docs\canonical\STACK_VERSION" -Raw).Trim()
    $files = @(
        "$PSScriptRoot\..\README.md",
        "$PSScriptRoot\..\1.docs\developer_docs\CODE_CLEANUP_STATUS_LEDGER.md",
        "$PSScriptRoot\..\1.docs\developer_docs\CODE_VS_CANON_STATUS_LEDGER.md"
    )

    foreach ($file in $files) {
        if (-not (Test-Path $file)) {
            throw "docs marker consistency failed: missing $file"
        }

        $content = Get-Content $file -Raw
        if ($content -notmatch [regex]::Escape($expected)) {
            throw "docs marker consistency failed: $file does not contain $expected"
        }
    }

    Write-Host "OK: docs marker consistency" -ForegroundColor Green
}

function Test-OrphanCrateClassification {
    $ledgerPath = "$PSScriptRoot\..\7.quality\inventory\crate_status_ledger.md"
    if (-not (Test-Path $ledgerPath)) {
        throw "orphan crate classification failed: missing crate status ledger"
    }

    $ledger = Get-Content $ledgerPath -Raw
    $requiredPaths = @(
        '5.editor/l10.0-project-bootstrap-service',
        '5.editor/l10.1-import-export-pipeline-service',
        '5.editor/l10.2-graph-authoring-service',
        '5.editor/l10.3-automation-and-batch-service',
        '5.editor/l10.4-script-and-hot-reload-service',
        '5.editor/l10.5-plugin-and-extension-host',
        '5.editor/l10.6-template-preset-and-scaffold-service',
        '5.editor/l10.7-package-market-and-dependency-service',
        '5.editor/l11.0-collaboration-session-surface',
        '5.editor/l11.1-review-annotation-surface',
        '5.editor/l11.2-asset-gate-and-approval-surface',
        '5.editor/l11.3-playtest-and-capture-operations',
        '5.editor/l11.4-production-dashboard-and-traceability',
        '5.editor/l11.5-learning-onboarding-and-help-surface',
        '5.editor/l8.11-build-release-surface',
        '5.editor/l8.2-outliner-system',
        '5.editor/l8.3-content-browser-system',
        '5.editor/l8.4-inspector-system',
        '5.editor/l8.6-overlay-and-gizmo-system',
        '5.editor/l8.7-workspace-layout-system',
        '5.editor/l8.8-interaction-routing-system',
        '5.editor/l8.9-assistant-surface',
        '5.editor/l9.1-scene-entity-authoring-suite',
        '5.editor/l9.10-quest-event-logic-authoring-suite',
        '5.editor/l9.11-build-validation-release-suite',
        '5.editor/l9.4-destruction-fracture-authoring-suite',
        '5.editor/l9.5-simulation-ai-authoring-suite',
        '5.editor/l9.7-animation-cinematics-authoring-suite',
        '5.editor/l9.8-audio-voice-authoring-suite',
        '5.editor/l9.9-ui-hud-authoring-suite'
    )

    $missing = @()
    foreach ($path in $requiredPaths) {
        if ($ledger -notmatch [regex]::Escape("| 5.editor | $path |")) {
            $missing += $path
        }
    }

    if ($missing.Count -gt 0) {
        throw "orphan crate classification failed: missing ledger entries for $($missing -join ', ')"
    }

    Write-Host "OK: orphan crate classification" -ForegroundColor Green
}

function Test-StubCrateClassification {
    $stubRoots = @(
        '5.editor/l10.1-import-export-pipeline-service/src/editor_import_export_pipeline_service.rs',
        '5.editor/l10.2-graph-authoring-service/src/editor_graph_authoring_service.rs',
        '5.editor/l10.3-automation-and-batch-service/src/editor_automation_and_batch_service.rs',
        '5.editor/l10.4-script-and-hot-reload-service/src/editor_script_and_hot_reload_service.rs',
        '5.editor/l10.5-plugin-and-extension-host/src/editor_plugin_and_extension_host.rs',
        '5.editor/l10.6-template-preset-and-scaffold-service/src/editor_template_preset_and_scaffold_service.rs',
        '5.editor/l10.7-package-market-and-dependency-service/src/editor_package_market_and_dependency_service.rs',
        '5.editor/l11.0-collaboration-session-surface/src/editor_collaboration_session_surface.rs',
        '5.editor/l11.1-review-annotation-surface/src/editor_review_annotation_surface.rs',
        '5.editor/l11.2-asset-gate-and-approval-surface/src/editor_asset_gate_and_approval_surface.rs',
        '5.editor/l11.3-playtest-and-capture-operations/src/editor_playtest_and_capture_operations.rs',
        '5.editor/l11.4-production-dashboard-and-traceability/src/editor_production_dashboard_and_traceability.rs',
        '5.editor/l11.5-learning-onboarding-and-help-surface/src/editor_learning_onboarding_and_help_surface.rs',
        '5.editor/l8.11-build-release-surface/src/editor_build_release_surface.rs',
        '5.editor/l8.2-outliner-system/src/editor_outliner_system.rs',
        '5.editor/l8.3-content-browser-system/src/editor_content_browser_system.rs',
        '5.editor/l8.4-inspector-system/src/editor_inspector_system.rs',
        '5.editor/l8.6-overlay-and-gizmo-system/src/editor_overlay_and_gizmo_system.rs',
        '5.editor/l8.7-workspace-layout-system/src/editor_workspace_layout_system.rs',
        '5.editor/l8.8-interaction-routing-system/src/editor_interaction_routing_system.rs',
        '5.editor/l8.9-assistant-surface/src/editor_assistant_surface.rs',
        '5.editor/l9.1-scene-entity-authoring-suite/src/editor_scene_entity_authoring_suite.rs',
        '5.editor/l9.10-quest-event-logic-authoring-suite/src/editor_quest_event_logic_authoring_suite.rs',
        '5.editor/l9.11-build-validation-release-suite/src/editor_build_validation_release_suite.rs',
        '5.editor/l9.4-destruction-fracture-authoring-suite/src/lib.rs',
        '5.editor/l9.5-simulation-ai-authoring-suite/src/editor_simulation_ai_authoring_suite.rs',
        '5.editor/l9.7-animation-cinematics-authoring-suite/src/editor_animation_cinematics_authoring_suite.rs',
        '5.editor/l9.8-audio-voice-authoring-suite/src/lib.rs',
        '5.editor/l9.9-ui-hud-authoring-suite/src/editor_ui_hud_authoring_suite.rs'
    )

    $missing = @()
    foreach ($relativePath in $stubRoots) {
        $fullPath = Join-Path "$PSScriptRoot\.." $relativePath
        if (-not (Test-Path $fullPath)) {
            $missing += $relativePath
            continue
        }

        $firstLine = Get-Content $fullPath -First 1
        if ($firstLine -ne '//! FUTURE_STUB.') {
            $missing += $relativePath
        }
    }

    if ($missing.Count -gt 0) {
        throw "stub crate classification failed: missing FUTURE_STUB header in $($missing -join ', ')"
    }

    Write-Host "OK: stub crate classification" -ForegroundColor Green
}

function Test-RootGarbage {
    $repoRoot = Resolve-Path "$PSScriptRoot\.."
    $allowed = @(
        'Cargo.toml',
        'Cargo.lock',
        'README.md',
        '.gitignore',
        '.github',
        '.git',
        '1.docs',
        '2.engine',
        '3.sdk',
        '4.tooling',
        '5.editor',
        '6.apps',
        '7.quality',
        '9.assets',
        'tools',
        'target',
        '_cleanup_scratch',
        '.claude'
    )

    $entries = Get-ChildItem $repoRoot -Force | Select-Object -ExpandProperty Name
    $extras = $entries | Where-Object { $allowed -notcontains $_ }
    if ($extras.Count -gt 0) {
        throw "root garbage failed: unexpected root entries $($extras -join ', ')"
    }

    if ($entries -contains '_cleanup_scratch') {
        Write-Host "WARN: root garbage includes temporary _cleanup_scratch" -ForegroundColor Yellow
    }
    if ($entries -contains '.claude') {
        Write-Host "WARN: root garbage includes local .claude directory" -ForegroundColor Yellow
    }

    Write-Host "OK: root garbage" -ForegroundColor Green
}

Write-Host "=== StratumX Doctor - Health Check ===" -ForegroundColor Cyan
Write-Host ""

Invoke-ExternalCheck "1. Workspace membership..." "$PSScriptRoot\validate-workspace.ps1" @()
Invoke-ExternalCheck "2. Layer dependencies..." "$PSScriptRoot\check-layer-boundaries.ps1" @()
Invoke-ExternalCheck "3. Test placement..." "$PSScriptRoot\check-test-placement.ps1" @()
Invoke-ExternalCheck "4. File size discipline..." "$PSScriptRoot\check-file-size-discipline.ps1" @()
Invoke-Step "5. Orphan crate classification..." { Test-OrphanCrateClassification }
Invoke-Step "6. Stub crate classification..." { Test-StubCrateClassification }
Invoke-Step "7. Root garbage..." { Test-RootGarbage }
Invoke-Step "8. Docs marker consistency..." { Test-DocsMarkerConsistency }
Invoke-ExternalCheck "9. Workspace format gate..." "$PSScriptRoot\check-workspace-format.ps1" @()
Invoke-ExternalCheck "10. cargo check --workspace..." "cargo" @("check", "--workspace")
Invoke-ExternalCheck "11. cargo clippy --workspace --all-targets -- -D warnings..." "cargo" @("clippy", "--workspace", "--all-targets", "--", "-D", "warnings")
Invoke-ExternalCheck "12. cargo test --workspace..." "cargo" @("test", "--workspace")
Invoke-ExternalCheck "13. Running verify..." "cargo" @("run", "-p", "stratumx_quality_tasks", "--", "verify")
Invoke-ExternalCheck "14. Running smoke..." "cargo" @("run", "-p", "stratumx_quality_tasks", "--", "smoke")

Write-Host "=== All checks passed! ===" -ForegroundColor Green

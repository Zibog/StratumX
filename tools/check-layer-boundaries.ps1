#!/usr/bin/env pwsh
# Check layer boundaries - verify architectural layer discipline

Write-Host "=== Layer Boundary Enforcement Check ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Checking for violations:"
Write-Host "  - Editor → Engine (must use SDK DTOs)"
Write-Host "  - Tooling → Editor (tooling should not depend on editor)"
Write-Host "  - Apps → Domain Logic (apps should only have bootstrap/wiring)"
Write-Host ""

cargo test -p forbidden_shortcuts --lib -- `
    editor_layer_does_not_import_engine_types_directly `
    tooling_layer_does_not_import_editor_types `
    apps_do_not_contain_domain_logic `
    --nocapture

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "PASS: Layer boundary checks passed!" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "FAIL: Layer boundary violations found!" -ForegroundColor Red
    exit 1
}

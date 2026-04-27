$ErrorActionPreference = 'Stop'
# Doctor - run diagnostic health check on the codebase

Write-Host "=== StratumX Doctor - Health Check ===" -ForegroundColor Cyan
Write-Host ""

Write-Host "1. Workspace Truth Validation..." -ForegroundColor Cyan
& "$PSScriptRoot\validate-workspace.ps1"
Write-Host ""

Write-Host "2. Root Cleanliness Check..." -ForegroundColor Cyan
cargo run -p stratumx_quality_tasks -- inventory
Write-Host ""

Write-Host "3. Running cargo fmt..." -ForegroundColor Cyan
cargo fmt --all --check
Write-Host ""

Write-Host "4. Running cargo clippy..." -ForegroundColor Cyan
cargo clippy --workspace --all-targets -- -D warnings
Write-Host ""

Write-Host "5. Running cargo test..." -ForegroundColor Cyan
cargo test --workspace
Write-Host ""

Write-Host "6. Running verify..." -ForegroundColor Cyan
cargo run -p stratumx_quality_tasks -- verify
Write-Host ""

Write-Host "7. Running smoke..." -ForegroundColor Cyan
cargo run -p stratumx_quality_tasks -- smoke
Write-Host ""

Write-Host "=== All checks passed! ===" -ForegroundColor Green

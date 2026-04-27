# Workspace Topology Validation Script (PowerShell)
# Validates that workspace.members matches active packages in filesystem

Write-Host "=== Workspace Truth Validation ===" -ForegroundColor Cyan
Write-Host ""

# Get workspace members from cargo metadata
Write-Host "Extracting workspace members from Cargo.toml..."
$metadata = cargo metadata --format-version=1 --no-deps | ConvertFrom-Json
$workspaceRoot = $metadata.workspace_root -replace '\\', '/'
$workspaceMembers = $metadata.packages | ForEach-Object { 
    $manifestPath = $_.manifest_path -replace '\\', '/'
    $manifestPath -replace "$workspaceRoot/", '' -replace '/Cargo.toml$', ''
} | Sort-Object
$workspaceCount = $workspaceMembers.Count

Write-Host "Found $workspaceCount packages in workspace metadata"
Write-Host ""

# Find all Cargo.toml files in active directories
Write-Host "Scanning filesystem for active packages..."
$filesystemPackages = @()
$directories = @("2.engine", "3.sdk", "4.tooling", "5.editor", "6.apps", "7.quality")

foreach ($dir in $directories) {
    if (Test-Path $dir) {
        $cargoFiles = Get-ChildItem -Path $dir -Filter "Cargo.toml" -Recurse -File
        foreach ($file in $cargoFiles) {
            $packagePath = $file.DirectoryName -replace '\\', '/' -replace '^.*?(?=2\.engine|3\.sdk|4\.tooling|5\.editor|6\.apps|7\.quality)', ''
            $filesystemPackages += $packagePath
        }
    }
}

$filesystemPackages = $filesystemPackages | Sort-Object
$filesystemCount = $filesystemPackages.Count

Write-Host "Found $filesystemCount packages in filesystem"
Write-Host ""

# Compare workspace members with filesystem packages
Write-Host "Checking for missing packages..."
$missingPackages = $filesystemPackages | Where-Object { $workspaceMembers -notcontains $_ }
$missingCount = $missingPackages.Count

if ($missingCount -gt 0) {
    Write-Host "❌ FAILED: Found $missingCount packages in filesystem not in workspace:" -ForegroundColor Red
    $missingPackages | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}

Write-Host "✅ All filesystem packages are in workspace" -ForegroundColor Green
Write-Host ""

# Check for workspace members that don't exist in filesystem
Write-Host "Checking for phantom workspace members..."
$phantomPackages = $workspaceMembers | Where-Object { $filesystemPackages -notcontains $_ }
$phantomCount = $phantomPackages.Count

if ($phantomCount -gt 0) {
    Write-Host "❌ FAILED: Found $phantomCount workspace members without Cargo.toml:" -ForegroundColor Red
    $phantomPackages | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}

Write-Host "✅ All workspace members exist in filesystem" -ForegroundColor Green
Write-Host ""

# Verify cargo metadata returns all expected packages
Write-Host "Verifying cargo metadata consistency..."
$metadataPackages = $metadata.packages | ForEach-Object { $_.name } | Sort-Object
$metadataCount = $metadataPackages.Count

if ($metadataCount -ne $workspaceCount) {
    Write-Host "❌ FAILED: Metadata package count ($metadataCount) doesn't match workspace count ($workspaceCount)" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Cargo metadata returns all $metadataCount packages" -ForegroundColor Green
Write-Host ""

# Check for workspace.exclude entries
Write-Host "Checking for workspace exclusions..."
$cargoToml = Get-Content "Cargo.toml" -Raw
if ($cargoToml -match 'exclude\s*=\s*\[(.*?)\]') {
    $excludeContent = $matches[1].Trim()
    if ($excludeContent -ne "") {
        Write-Host "⚠️  WARNING: Found workspace.exclude entries" -ForegroundColor Yellow
        Write-Host $excludeContent -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Consider removing exclusions for workspace truth integrity"
    } else {
        Write-Host "✅ No workspace exclusions (honest topology)" -ForegroundColor Green
    }
} else {
    Write-Host "✅ No workspace exclusions (honest topology)" -ForegroundColor Green
}

Write-Host ""
Write-Host "=== Workspace Truth Validation PASSED ===" -ForegroundColor Green

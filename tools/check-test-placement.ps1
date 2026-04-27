#!/usr/bin/env pwsh
# Check Test Placement - ensures heavy tests are in 7.quality
# Heavy tests include: integration tests, property tests, matrix tests

$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot/..

$misplacedTests = @()

# Find integration test files outside 7.quality
$integrationTests = Get-ChildItem -Path 2.engine,3.sdk,4.tooling,5.editor,6.apps -Recurse -Filter "*.rs" | 
    Where-Object { $_.FullName -match "\\tests\\" -and $_.FullName -notmatch "7\.quality" -and $_.FullName -notmatch "target" }

foreach ($test in $integrationTests) {
    $relativePath = $test.FullName.Replace((Get-Location).Path + "\", "")
    $misplacedTests += [PSCustomObject]@{
        Type = "Integration Test"
        Path = $relativePath
        Issue = "Heavy test outside 7.quality"
    }
}

# Find large test modules in src files (>100 lines of test code)
$srcFiles = Get-ChildItem -Path 2.engine,3.sdk,4.tooling,5.editor,6.apps -Recurse -Filter "*.rs" | 
    Where-Object { $_.FullName -notmatch "\\tests\\" -and $_.FullName -notmatch "7\.quality" -and $_.FullName -notmatch "target" }

foreach ($file in $srcFiles) {
    $content = Get-Content $file.FullName -Raw
    if ($content -match '#\[cfg\(test\)\]') {
        # Count lines in test sections
        $testBlocks = [regex]::Matches($content, '#\[cfg\(test\)\].*?(?=#\[cfg|\z)')
        foreach ($match in $testBlocks) {
            $lines = $match.Value.Split("`n").Count
            if ($lines -gt 100) {
                $relativePath = $file.FullName.Replace((Get-Location).Path + "\", "")
                $misplacedTests += [PSCustomObject]@{
                    Type = "Large Test Module"
                    Path = $relativePath
                    Issue = "Test module >100 LOC (should be in 7.quality)"
                }
            }
        }
    }
}

if ($misplacedTests.Count -gt 0) {
    Write-Host "❌ MISPLACED TESTS FOUND" -ForegroundColor Red
    Write-Host "Heavy tests should be in 7.quality/suites/" -ForegroundColor Yellow
    Write-Host ""
    $misplacedTests | Format-Table -AutoSize
    Write-Host ""
    Write-Host "Total misplaced: $($misplacedTests.Count)" -ForegroundColor Red
    exit 1
} else {
    Write-Host "✅ All tests properly placed in 7.quality" -ForegroundColor Green
    exit 0
}

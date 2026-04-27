#!/usr/bin/env pwsh
# File Size Discipline Checker
# Scans codebase for files exceeding 200 LOC without justification

$MAX_LOC = 200
$JUSTIFICATION_PATTERNS = @(
    "JUSTIFICATION:",
    "LOC_JUSTIFICATION:",
    "FILE_SIZE_JUSTIFICATION:"
)

function Count-LinesOfCode {
    param([string]$Content)
    
    $loc = 0
    $inBlockComment = $false
    
    foreach ($line in $Content -split "`n") {
        $trimmed = $line.Trim()
        
        # Handle block comments
        if ($trimmed.StartsWith("/*")) {
            $inBlockComment = $true
        }
        if ($inBlockComment) {
            if ($trimmed.EndsWith("*/") -or $trimmed.Contains("*/")) {
                $inBlockComment = $false
            }
            continue
        }
        
        # Skip blank lines and line comments
        if ([string]::IsNullOrWhiteSpace($trimmed) -or $trimmed.StartsWith("//")) {
            continue
        }
        
        $loc++
    }
    
    return $loc
}

function Test-HasJustification {
    param([string]$Content)
    
    foreach ($pattern in $JUSTIFICATION_PATTERNS) {
        if ($Content.Contains($pattern)) {
            return $true
        }
    }
    return $false
}

function Get-FileSizeViolations {
    param([string]$RepoRoot)
    
    $violations = @()
    $directories = @(
        "2.engine",
        "3.sdk",
        "4.tooling",
        "5.editor",
        "6.apps"
    )
    
    foreach ($dir in $directories) {
        $fullPath = Join-Path $RepoRoot $dir
        if (Test-Path $fullPath) {
            $files = Get-ChildItem -Path $fullPath -Filter "*.rs" -Recurse -File | 
                Where-Object { $_.FullName -notmatch "\\target\\" -and $_.FullName -notmatch "\\.git\\" }
            
            foreach ($file in $files) {
                $content = Get-Content -Path $file.FullName -Raw
                $loc = Count-LinesOfCode -Content $content
                
                if ($loc -gt $MAX_LOC) {
                    $hasJustification = Test-HasJustification -Content $content
                    $violations += [PSCustomObject]@{
                        Path = $file.FullName.Replace($RepoRoot + "\", "")
                        LOC = $loc
                        HasJustification = $hasJustification
                    }
                }
            }
        }
    }
    
    return $violations
}

function Format-Violations {
    param([array]$Violations)
    
    if ($Violations.Count -eq 0) {
        Write-Host "✓ File size discipline check passed: No violations found" -ForegroundColor Green
        return $true
    }
    
    $unjustified = $Violations | Where-Object { -not $_.HasJustification } | Sort-Object -Property LOC -Descending
    $justified = $Violations | Where-Object { $_.HasJustification } | Sort-Object -Property LOC -Descending
    
    Write-Host ""
    Write-Host "✗ File size discipline check failed: $($Violations.Count) files exceed $MAX_LOC LOC" -ForegroundColor Red
    Write-Host ""
    
    if ($unjustified.Count -gt 0) {
        Write-Host "Files requiring split or justification ($($unjustified.Count)):" -ForegroundColor Yellow
        foreach ($v in $unjustified) {
            Write-Host "  - $($v.Path) ($($v.LOC) LOC)" -ForegroundColor Yellow
        }
        Write-Host ""
    }
    
    if ($justified.Count -gt 0) {
        Write-Host "Files with justification ($($justified.Count)):" -ForegroundColor Cyan
        foreach ($v in $justified) {
            Write-Host "  - $($v.Path) ($($v.LOC) LOC) [JUSTIFIED]" -ForegroundColor Cyan
        }
        Write-Host ""
    }
    
    Write-Host "Remediation:" -ForegroundColor White
    Write-Host "  1. Split large files by role: ids.rs, types.rs, errors.rs, validation.rs, commands.rs, queries.rs, service.rs"
    Write-Host "  2. Add justification comment with pattern: // JUSTIFICATION: <reason>"
    Write-Host "  3. Document split in 1.docs/history/SANITATION_LEDGER.md"
    Write-Host ""
    
    return ($unjustified.Count -eq 0)
}

# Main execution
$repoRoot = Split-Path -Parent $PSScriptRoot
Write-Host "Checking file size discipline in: $repoRoot"
Write-Host ""

$violations = Get-FileSizeViolations -RepoRoot $repoRoot
$passed = Format-Violations -Violations $violations

if ($passed) {
    exit 0
} else {
    exit 1
}

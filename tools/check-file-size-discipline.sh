#!/usr/bin/env bash
# File Size Discipline Checker
# Scans codebase for files exceeding 200 LOC (warn) or 300 LOC (fail)

set -e

FAIL_LOC=300
WARN_LOC=200
JUSTIFICATION_PATTERNS=("JUSTIFICATION:" "LOC_JUSTIFICATION:" "FILE_SIZE_JUSTIFICATION:")

count_lines_of_code() {
    local content="$1"
    local loc=0
    local in_block_comment=false
    
    while IFS= read -r line; do
        local trimmed=$(echo "$line" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        
        # Handle block comments
        if [[ "$trimmed" == "/*"* ]]; then
            in_block_comment=true
        fi
        if [ "$in_block_comment" = true ]; then
            if [[ "$trimmed" == *"*/" ]]; then
                in_block_comment=false
            fi
            continue
        fi
        
        # Skip blank lines and line comments
        if [ -z "$trimmed" ] || [[ "$trimmed" == "//"* ]]; then
            continue
        fi
        
        ((loc++))
    done <<< "$content"
    
    echo "$loc"
}

has_justification() {
    local content="$1"
    for pattern in "${JUSTIFICATION_PATTERNS[@]}"; do
        if echo "$content" | grep -q "$pattern"; then
            return 0
        fi
    done
    return 1
}

# Get repository root
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "Checking file size discipline in: $REPO_ROOT"
echo ""

# Arrays to store violations
declare -a fail_files
declare -a fail_locs
declare -a warn_unjustified_files
declare -a warn_unjustified_locs
declare -a warn_justified_files
declare -a warn_justified_locs

# Scan directories
for dir in "2.engine" "3.sdk" "4.tooling" "5.editor" "6.apps"; do
    full_path="$REPO_ROOT/$dir"
    if [ -d "$full_path" ]; then
        while IFS= read -r -d '' file; do
            # Skip target and .git directories
            if [[ "$file" == *"/target/"* ]] || [[ "$file" == *"/.git/"* ]]; then
                continue
            fi
            
            content=$(cat "$file")
            loc=$(count_lines_of_code "$content")
            rel_path="${file#$REPO_ROOT/}"
            
            if [ "$loc" -gt "$FAIL_LOC" ]; then
                # Files exceeding 300 LOC always fail
                fail_files+=("$rel_path")
                fail_locs+=("$loc")
            elif [ "$loc" -gt "$WARN_LOC" ]; then
                # Files exceeding 200 LOC warn unless justified
                if has_justification "$content"; then
                    warn_justified_files+=("$rel_path")
                    warn_justified_locs+=("$loc")
                else
                    warn_unjustified_files+=("$rel_path")
                    warn_unjustified_locs+=("$loc")
                fi
            fi
        done < <(find "$full_path" -name "*.rs" -type f -print0)
    fi
done

# Calculate total violations and warnings
total_failures=${#fail_files[@]}
total_warnings=$((${#warn_unjustified_files[@]} + ${#warn_justified_files[@]}))

# Display failures (>300 LOC)
if [ "$total_failures" -gt 0 ]; then
    echo "✗ FAIL: $total_failures files exceed $FAIL_LOC LOC (must be split):"
    for i in "${!fail_files[@]}"; do
        echo "  - ${fail_files[$i]} (${fail_locs[$i]} LOC)"
    done
    echo ""
fi

# Display warnings (>200 LOC)
if [ "${#warn_unjustified_files[@]}" -gt 0 ]; then
    echo "⚠ WARN: ${#warn_unjustified_files[@]} files exceed $WARN_LOC LOC without justification:"
    for i in "${!warn_unjustified_files[@]}"; do
        echo "  - ${warn_unjustified_files[$i]} (${warn_unjustified_locs[$i]} LOC)"
    done
    echo ""
fi

if [ "${#warn_justified_files[@]}" -gt 0 ]; then
    echo "ℹ INFO: ${#warn_justified_files[@]} files exceed $WARN_LOC LOC with justification:"
    for i in "${!warn_justified_files[@]}"; do
        echo "  - ${warn_justified_files[$i]} (${warn_justified_locs[$i]} LOC) [JUSTIFIED]"
    done
    echo ""
fi

if [ "$total_failures" -eq 0 ] && [ "$total_warnings" -eq 0 ]; then
    echo "✓ File size discipline check passed: No violations found"
    exit 0
fi

echo "Remediation:"
echo "  1. Split large files by role: ids.rs, types.rs, errors.rs, validation.rs, commands.rs, queries.rs, service.rs"
echo "  2. Add justification comment with pattern: // JUSTIFICATION: <reason>"
echo "  3. Document split in 1.docs/history/SANITATION_LEDGER.md"
echo ""

if [ "$total_failures" -eq 0 ]; then
    echo "✓ No failures (warnings only)"
    exit 0
else
    echo "✗ File size discipline check failed: $total_failures files exceed $FAIL_LOC LOC"
    exit 1
fi

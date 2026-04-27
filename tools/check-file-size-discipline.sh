#!/usr/bin/env bash
# File Size Discipline Checker
# Scans codebase for files exceeding 200 LOC without justification

set -e

MAX_LOC=200
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
declare -a unjustified_files
declare -a unjustified_locs
declare -a justified_files
declare -a justified_locs

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
            
            if [ "$loc" -gt "$MAX_LOC" ]; then
                rel_path="${file#$REPO_ROOT/}"
                if has_justification "$content"; then
                    justified_files+=("$rel_path")
                    justified_locs+=("$loc")
                else
                    unjustified_files+=("$rel_path")
                    unjustified_locs+=("$loc")
                fi
            fi
        done < <(find "$full_path" -name "*.rs" -type f -print0)
    fi
done

# Calculate total violations
total_violations=$((${#unjustified_files[@]} + ${#justified_files[@]}))

if [ "$total_violations" -eq 0 ]; then
    echo "✓ File size discipline check passed: No violations found"
    exit 0
fi

# Display violations
echo "✗ File size discipline check failed: $total_violations files exceed $MAX_LOC LOC"
echo ""

if [ "${#unjustified_files[@]}" -gt 0 ]; then
    echo "Files requiring split or justification (${#unjustified_files[@]}):"
    for i in "${!unjustified_files[@]}"; do
        echo "  - ${unjustified_files[$i]} (${unjustified_locs[$i]} LOC)"
    done
    echo ""
fi

if [ "${#justified_files[@]}" -gt 0 ]; then
    echo "Files with justification (${#justified_files[@]}):"
    for i in "${!justified_files[@]}"; do
        echo "  - ${justified_files[$i]} (${justified_locs[$i]} LOC) [JUSTIFIED]"
    done
    echo ""
fi

echo "Remediation:"
echo "  1. Split large files by role: ids.rs, types.rs, errors.rs, validation.rs, commands.rs, queries.rs, service.rs"
echo "  2. Add justification comment with pattern: // JUSTIFICATION: <reason>"
echo "  3. Document split in 1.docs/history/SANITATION_LEDGER.md"
echo ""

if [ "${#unjustified_files[@]}" -eq 0 ]; then
    exit 0
else
    exit 1
fi

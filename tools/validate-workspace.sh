#!/usr/bin/env bash
# Workspace Topology Validation Script
# Validates that workspace.members matches active packages in filesystem
set -e

echo "=== Workspace Truth Validation ==="
echo ""

# Get workspace members from Cargo.toml
echo "Extracting workspace members from Cargo.toml..."
WORKSPACE_MEMBERS=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[].manifest_path' | sed 's|/Cargo.toml||' | sort)
WORKSPACE_COUNT=$(echo "$WORKSPACE_MEMBERS" | wc -l)

echo "Found $WORKSPACE_COUNT packages in workspace metadata"
echo ""

# Find all Cargo.toml files in active directories (2.engine, 3.sdk, 4.tooling, 5.editor, 6.apps, 7.quality)
echo "Scanning filesystem for active packages..."
FILESYSTEM_PACKAGES=$(find 2.engine 3.sdk 4.tooling 5.editor 6.apps 7.quality -name "Cargo.toml" -type f 2>/dev/null | sed 's|/Cargo.toml||' | sort)
FILESYSTEM_COUNT=$(echo "$FILESYSTEM_PACKAGES" | wc -l)

echo "Found $FILESYSTEM_COUNT packages in filesystem"
echo ""

# Compare workspace members with filesystem packages
echo "Checking for missing packages..."
MISSING_PACKAGES=$(comm -13 <(echo "$WORKSPACE_MEMBERS") <(echo "$FILESYSTEM_PACKAGES"))
MISSING_COUNT=$(echo "$MISSING_PACKAGES" | grep -c . || echo 0)

if [ "$MISSING_COUNT" -gt 0 ]; then
    echo "❌ FAILED: Found $MISSING_COUNT packages in filesystem not in workspace:"
    echo "$MISSING_PACKAGES"
    exit 1
fi

echo "✅ All filesystem packages are in workspace"
echo ""

# Check for workspace members that don't exist in filesystem
echo "Checking for phantom workspace members..."
PHANTOM_PACKAGES=$(comm -23 <(echo "$WORKSPACE_MEMBERS") <(echo "$FILESYSTEM_PACKAGES"))
PHANTOM_COUNT=$(echo "$PHANTOM_PACKAGES" | grep -c . || echo 0)

if [ "$PHANTOM_COUNT" -gt 0 ]; then
    echo "❌ FAILED: Found $PHANTOM_COUNT workspace members without Cargo.toml:"
    echo "$PHANTOM_PACKAGES"
    exit 1
fi

echo "✅ All workspace members exist in filesystem"
echo ""

# Verify cargo metadata returns all expected packages
echo "Verifying cargo metadata consistency..."
METADATA_PACKAGES=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[].name' | sort)
METADATA_COUNT=$(echo "$METADATA_PACKAGES" | wc -l)

if [ "$METADATA_COUNT" -ne "$WORKSPACE_COUNT" ]; then
    echo "❌ FAILED: Metadata package count ($METADATA_COUNT) doesn't match workspace count ($WORKSPACE_COUNT)"
    exit 1
fi

echo "✅ Cargo metadata returns all $METADATA_COUNT packages"
echo ""

# Check for workspace.exclude entries
echo "Checking for workspace exclusions..."
EXCLUDE_COUNT=$(grep -A 100 "^exclude = " Cargo.toml | grep -c '\"' || echo 0)

if [ "$EXCLUDE_COUNT" -gt 0 ]; then
    echo "⚠️  WARNING: Found $EXCLUDE_COUNT workspace.exclude entries"
    grep -A 100 "^exclude = " Cargo.toml | grep '\"'
    echo ""
    echo "Consider removing exclusions for workspace truth integrity"
else
    echo "✅ No workspace exclusions (honest topology)"
fi

echo ""
echo "=== Workspace Truth Validation PASSED ==="

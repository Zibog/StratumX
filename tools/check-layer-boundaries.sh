#!/usr/bin/env bash
# Check layer boundaries - verify architectural layer discipline
set -e

echo "=== Layer Boundary Enforcement Check ==="
echo ""
echo "Checking for violations:"
echo "  - Editor → Engine (must use SDK DTOs)"
echo "  - Tooling → Editor (tooling should not depend on editor)"
echo "  - Apps → Domain Logic (apps should only have bootstrap/wiring)"
echo ""

cargo test -p forbidden_shortcuts --lib -- \
    editor_layer_does_not_import_engine_types_directly \
    tooling_layer_does_not_import_editor_types \
    apps_do_not_contain_domain_logic \
    --nocapture

echo ""
echo "✓ Layer boundary checks passed!"

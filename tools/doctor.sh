#!/usr/bin/env bash
# Doctor - run diagnostic health check on the codebase
set -e

echo "=== StratumX Doctor - Health Check ==="
echo ""

echo "1. Workspace Truth Validation..."
bash tools/validate-workspace.sh
echo ""

echo "2. Root Cleanliness Check..."
cargo run -p stratumx_quality_tasks -- inventory
echo ""

echo "3. Running cargo fmt..."
cargo fmt --all --check
echo ""

echo "4. Running cargo clippy..."
cargo clippy --workspace --all-targets -- -D warnings
echo ""

echo "5. Running cargo test..."
cargo test --workspace
echo ""

echo "6. Running verify..."
cargo run -p stratumx_quality_tasks -- verify
echo ""

echo "7. Running smoke..."
cargo run -p stratumx_quality_tasks -- smoke
echo ""

echo "=== All checks passed! ==="

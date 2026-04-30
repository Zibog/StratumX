#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
bash tools/verify.sh
bash tools/smoke.sh
bash tools/full.sh
bash tools/metrics.sh
bash tools/evidence.sh

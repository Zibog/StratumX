#!/usr/bin/env bash
# Full - run full build, test, and quality pipeline
set -e
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p stratumx_quality_tasks -- full

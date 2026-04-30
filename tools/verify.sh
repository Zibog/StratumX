#!/usr/bin/env bash
# Verify - run formatting and verification checks
set -e
cargo fmt --all --check
cargo run -p stratumx_quality_tasks -- verify

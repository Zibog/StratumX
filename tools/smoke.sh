#!/usr/bin/env bash
# Smoke - run smoke tests for quality verification
set -e
cargo run -p stratumx_quality_tasks -- smoke

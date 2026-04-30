#!/usr/bin/env bash
# Bench - run benchmark suite
set -e
cargo run -p stratumx_quality_tasks -- bench

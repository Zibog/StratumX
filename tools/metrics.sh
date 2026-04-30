#!/usr/bin/env bash
# Metrics - collect and report code quality metrics
set -e
cargo run -p stratumx_quality_tasks -- metrics

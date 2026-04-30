#!/usr/bin/env bash
# Canon Coverage - measure canon coverage metrics
set -e
cargo run -p stratumx_quality_tasks -- canon-coverage

#!/usr/bin/env bash
# Evidence - generate evidence/proof pack from quality runs
set -e
cargo run -p stratumx_quality_tasks -- evidence

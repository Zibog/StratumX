#!/usr/bin/env bash
# Inventory - list all packages, crates, and layer topology
set -e
cargo run -p stratumx_quality_tasks -- inventory

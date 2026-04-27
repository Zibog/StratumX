$ErrorActionPreference = 'Stop'
# Inventory - list all packages, crates, and layer topology
cargo run -p stratumx_quality_tasks -- inventory

$ErrorActionPreference = 'Stop'
cargo fmt --all --check
cargo run -p stratumx_quality_tasks -- verify

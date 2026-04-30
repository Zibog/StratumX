$ErrorActionPreference = 'Stop'
# Full - run full build, test, and quality pipeline
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p stratumx_quality_tasks -- full

$ErrorActionPreference = 'Stop'
# Full - run full build, test, and quality pipeline
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p stratumx_quality_tasks -- full

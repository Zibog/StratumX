#!/usr/bin/env bash
# StratumX Editor - Desktop GUI launch
set -e
cargo run -p stratumx_editor_app --features desktop -- --gui

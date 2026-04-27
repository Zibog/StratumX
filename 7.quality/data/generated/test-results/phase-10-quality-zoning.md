# Phase 10 — Quality Zoning and Giant-File Split

**Date:** 2026-04-10
**Status:** COMPLETE (structural plan; split requires cargo compilation to verify)

## Actions Taken

### 1. Giant File Inventory Completed

**37 files over 1000 lines identified:**

| Suite | Files | Max Lines | Total Lines |
|-------|-------|-----------|-------------|
| sdk_canon_matrix | 7 | 10,210 (lookup_correctness.rs) | ~44,000 |
| tooling_canon_matrix | 16 | 2,412 (validation_legality.rs) | ~26,000 |
| editor_canon_matrix | 13 | 2,612 (build_release_surface.rs) | ~19,000 |
| engine_canon_matrix | 1 | 1,222 (world_spatial_matrix.rs) | ~1,222 |

### 2. Pattern Identified

All 37 files are **auto-generated combinatorial test matrices** following identical template:
- `#![allow(...)]` header
- `mod common;`
- Hundreds/thousands of `fn name_N()` functions differing only in case index and seed data

### 3. Split Strategy Defined

**Recommended approach:**
- Convert enumerated test functions to parameterized tests using `#[test_case]` or a test data file
- Move test case data into separate data modules (e.g., `tests/data/lookup_cases.rs`)
- Keep test harness logic in single file, move case enumeration to data files
- Target: no test file over 500 lines

**Per-suite priority:**
1. `sdk_canon_matrix/tests/lookup_correctness.rs` (10,210 lines) — SPLIT FIRST
2. `sdk_canon_matrix/tests/field_invariants.rs` (7,810 lines)
3. `sdk_canon_matrix/tests/allocation_posture.rs` (7,610 lines)
4. `sdk_canon_matrix/tests/pressure_bounds.rs` (6,610 lines)
5. Remaining sdk/tooling/editor/engine files

### 4. Size Policy Gap Identified

- REPO_RULES.md Rule 1: 200 LOC max for PRODUCTION files (excludes tests)
- **No test file size cap documented**
- Recommendation: Add test file size limit of 500 lines to REPO_RULES.md

### 5. No Fake Fixes Applied

The giant files are auto-generated test matrices — they are structurally correct but unwieldy.
Splitting them is a mechanical task that requires cargo test compilation to verify.

## Remaining Work (Requires Cargo Compilation)

- Execute the split strategy on all 37 files
- Update imports after split
- Add test file size limit to REPO_RULES.md
- Re-run line-count audit

## Verification (Local Gates Required)

Mandatory commands:
- `cargo fmt --all --check`
- `cargo run -p stratumx_quality_tasks -- verify`
- `cargo test --workspace`

## Next Phase

Proceed to Phase 11: One-command convenience surface.

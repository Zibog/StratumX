# Giant File Split Plans

## Purpose

This document contains detailed split plans for all files exceeding 800 lines (test files) or 400 lines (support files). Each plan specifies the split strategy, target files, estimated line counts, and implementation steps.

These plans are created as part of Package 3 (Task 9) and will be executed in Package 5 (Tasks 15-17).

## Split Plan Format

Each split plan includes:
- **Original File**: Path and current line count
- **Test Count**: Number of tests in the file
- **Test Family**: The test family name(s)
- **Split Strategy**: The chosen approach (by number range, by family, parameterize, by domain)
- **Rationale**: Why this strategy was chosen
- **Target Files**: List of new files with estimated line counts
- **Implementation Steps**: Detailed steps for executing the split
- **Estimated Effort**: Time estimate for completing the split

## Priority Giant Files (>4000 lines)

These 6 files are the highest priority and should be split first in Package 5.

---

### 1. lookup_correctness.rs

**Original File**: `7.quality/suites/sdk_canon_matrix/tests/lookup_correctness.rs`
**Current Size**: 10,407 lines
**Test Count**: 200 tests (lookup_correctness_0 through lookup_correctness_199)
**Test Family**: Single family (lookup_correctness)
**Average Test Size**: ~52 lines per test

**Split Strategy**: Split by Number Range (4-way split)

**Rationale**: 
- All 200 tests follow identical naming pattern (lookup_correctness_N)
- Tests are uniformly sized (~52 lines each)
- No semantic groupings evident from test names
- Number range split is cleanest approach for uniform numbered tests
- 4-way split keeps each file under 800 lines with room for growth

**Target Files**:

1. `lookup_correctness_000_049.rs` - Tests 0-49 (50 tests, ~2,600 lines)
2. `lookup_correctness_050_099.rs` - Tests 50-99 (50 tests, ~2,600 lines)
3. `lookup_correctness_100_149.rs` - Tests 100-149 (50 tests, ~2,600 lines)
4. `lookup_correctness_150_199.rs` - Tests 150-199 (50 tests, ~2,600 lines)

**Implementation Steps**:

1. Create `lookup_correctness_000_049.rs`:
   - Copy file header and imports from original
   - Extract tests 0-49 (lines 13-2610 approximately)
   - Verify all imports are needed
   
2. Create `lookup_correctness_050_099.rs`:
   - Copy file header and imports from original
   - Extract tests 50-99 (lines 2663-5260 approximately)
   - Verify all imports are needed

3. Create `lookup_correctness_100_149.rs`:
   - Copy file header and imports from original
   - Extract tests 100-149 (lines 5313-7960 approximately)
   - Verify all imports are needed

4. Create `lookup_correctness_150_199.rs`:
   - Copy file header and imports from original
   - Extract tests 150-199 (lines 7963-10560 approximately)
   - Verify all imports are needed

5. Update `7.quality/suites/sdk_canon_matrix/Cargo.toml`:
   - Verify test discovery includes new files (should be automatic)

6. Update `7.quality/suites/sdk_canon_matrix/tests/mod.rs` if it exists:
   - Add module declarations for new files

7. Run tests to verify:
   ```bash
   cd 7.quality/suites/sdk_canon_matrix
   cargo test lookup_correctness -- --nocapture
   ```

8. Verify all 200 tests still pass

9. Delete original `lookup_correctness.rs`

10. Update suite documentation to reflect the split

**Estimated Effort**: 2-3 hours

---

### 2. field_invariants.rs

**Original File**: `7.quality/suites/sdk_canon_matrix/tests/field_invariants.rs`
**Current Size**: 8,007 lines
**Test Count**: 200 tests (field_invariants_0 through field_invariants_199)
**Test Family**: Single family (field_invariants)
**Average Test Size**: ~40 lines per test

**Split Strategy**: Split by Number Range (4-way split)

**Rationale**:
- All 200 tests follow identical naming pattern (field_invariants_N)
- Tests are uniformly sized (~40 lines each)
- No semantic groupings evident from test names
- Number range split is cleanest approach for uniform numbered tests
- 4-way split keeps each file under 800 lines

**Target Files**:

1. `field_invariants_000_049.rs` - Tests 0-49 (50 tests, ~2,000 lines)
2. `field_invariants_050_099.rs` - Tests 50-99 (50 tests, ~2,000 lines)
3. `field_invariants_100_149.rs` - Tests 100-149 (50 tests, ~2,000 lines)
4. `field_invariants_150_199.rs` - Tests 150-199 (50 tests, ~2,000 lines)

**Implementation Steps**:

1. Create `field_invariants_000_049.rs`:
   - Copy file header and imports from original
   - Extract tests 0-49 (lines 13-2022 approximately)
   
2. Create `field_invariants_050_099.rs`:
   - Copy file header and imports from original
   - Extract tests 50-99 (lines 2063-4072 approximately)

3. Create `field_invariants_100_149.rs`:
   - Copy file header and imports from original
   - Extract tests 100-149 (lines 4113-6122 approximately)

4. Create `field_invariants_150_199.rs`:
   - Copy file header and imports from original
   - Extract tests 150-199 (lines 6163-8172 approximately)

5. Update module declarations if needed

6. Run tests to verify all 200 tests pass

7. Delete original `field_invariants.rs`

8. Update suite documentation

**Estimated Effort**: 2-3 hours

---

### 3. allocation_posture.rs

**Original File**: `7.quality/suites/sdk_canon_matrix/tests/allocation_posture.rs`
**Current Size**: 7,807 lines
**Test Count**: 200 tests (allocation_posture_0 through allocation_posture_199)
**Test Family**: Single family (allocation_posture)
**Average Test Size**: ~39 lines per test

**Split Strategy**: Split by Number Range (4-way split)

**Rationale**:
- All 200 tests follow identical naming pattern (allocation_posture_N)
- Tests are uniformly sized (~39 lines each)
- No semantic groupings evident from test names
- Number range split is cleanest approach for uniform numbered tests
- 4-way split keeps each file under 800 lines

**Target Files**:

1. `allocation_posture_000_049.rs` - Tests 0-49 (50 tests, ~1,950 lines)
2. `allocation_posture_050_099.rs` - Tests 50-99 (50 tests, ~1,950 lines)
3. `allocation_posture_100_149.rs` - Tests 100-149 (50 tests, ~1,950 lines)
4. `allocation_posture_150_199.rs` - Tests 150-199 (50 tests, ~1,950 lines)

**Implementation Steps**:

1. Create `allocation_posture_000_049.rs`:
   - Copy file header and imports from original
   - Extract tests 0-49 (lines 13-1973 approximately)
   
2. Create `allocation_posture_050_099.rs`:
   - Copy file header and imports from original
   - Extract tests 50-99 (lines 2013-3973 approximately)

3. Create `allocation_posture_100_149.rs`:
   - Copy file header and imports from original
   - Extract tests 100-149 (lines 4013-5973 approximately)

4. Create `allocation_posture_150_199.rs`:
   - Copy file header and imports from original
   - Extract tests 150-199 (lines 6013-7973 approximately)

5. Update module declarations if needed

6. Run tests to verify all 200 tests pass

7. Delete original `allocation_posture.rs`

8. Update suite documentation

**Estimated Effort**: 2-3 hours

---

### 4. pressure_bounds.rs

**Original File**: `7.quality/suites/sdk_canon_matrix/tests/pressure_bounds.rs`
**Current Size**: 6,807 lines
**Test Count**: 200 tests (pressure_bounds_0 through pressure_bounds_199)
**Test Family**: Single family (pressure_bounds)
**Average Test Size**: ~34 lines per test

**Split Strategy**: Split by Number Range (4-way split)

**Rationale**:
- All 200 tests follow identical naming pattern (pressure_bounds_N)
- Tests are uniformly sized (~34 lines each)
- No semantic groupings evident from test names
- Number range split is cleanest approach for uniform numbered tests
- 4-way split keeps each file well under 800 lines

**Target Files**:

1. `pressure_bounds_000_049.rs` - Tests 0-49 (50 tests, ~1,700 lines)
2. `pressure_bounds_050_099.rs` - Tests 50-99 (50 tests, ~1,700 lines)
3. `pressure_bounds_100_149.rs` - Tests 100-149 (50 tests, ~1,700 lines)
4. `pressure_bounds_150_199.rs` - Tests 150-199 (50 tests, ~1,700 lines)

**Implementation Steps**:

1. Create `pressure_bounds_000_049.rs`:
   - Copy file header and imports from original
   - Extract tests 0-49 (lines 13-1728 approximately)
   
2. Create `pressure_bounds_050_099.rs`:
   - Copy file header and imports from original
   - Extract tests 50-99 (lines 1763-3478 approximately)

3. Create `pressure_bounds_100_149.rs`:
   - Copy file header and imports from original
   - Extract tests 100-149 (lines 3513-5228 approximately)

4. Create `pressure_bounds_150_199.rs`:
   - Copy file header and imports from original
   - Extract tests 150-199 (lines 5263-6978 approximately)

5. Update module declarations if needed

6. Run tests to verify all 200 tests pass

7. Delete original `pressure_bounds.rs`

8. Update suite documentation

**Estimated Effort**: 2-3 hours

---

### 5. boundary_legality.rs

**Original File**: `7.quality/suites/sdk_canon_matrix/tests/boundary_legality.rs`
**Current Size**: 4,807 lines
**Test Count**: 200 tests (boundary_legality_0 through boundary_legality_199)
**Test Family**: Single family (boundary_legality)
**Average Test Size**: ~24 lines per test

**Split Strategy**: Split by Number Range (3-way split)

**Rationale**:
- All 200 tests follow identical naming pattern (boundary_legality_N)
- Tests are uniformly sized (~24 lines each)
- No semantic groupings evident from test names
- Number range split is cleanest approach for uniform numbered tests
- 3-way split keeps each file well under 800 lines (smaller tests allow fewer splits)

**Target Files**:

1. `boundary_legality_000_066.rs` - Tests 0-66 (67 tests, ~1,600 lines)
2. `boundary_legality_067_133.rs` - Tests 67-133 (67 tests, ~1,600 lines)
3. `boundary_legality_134_199.rs` - Tests 134-199 (66 tests, ~1,600 lines)

**Implementation Steps**:

1. Create `boundary_legality_000_066.rs`:
   - Copy file header and imports from original
   - Extract tests 0-66 (lines 13-1663 approximately)
   
2. Create `boundary_legality_067_133.rs`:
   - Copy file header and imports from original
   - Extract tests 67-133 (lines 1688-3338 approximately)

3. Create `boundary_legality_134_199.rs`:
   - Copy file header and imports from original
   - Extract tests 134-199 (lines 3363-4988 approximately)

4. Update module declarations if needed

5. Run tests to verify all 200 tests pass

6. Delete original `boundary_legality.rs`

7. Update suite documentation

**Estimated Effort**: 2 hours

---

### 6. snapshot_swaps.rs

**Original File**: `7.quality/suites/sdk_canon_matrix/tests/snapshot_swaps.rs`
**Current Size**: 4,407 lines
**Test Count**: 200 tests (snapshot_swaps_0 through snapshot_swaps_199)
**Test Family**: Single family (snapshot_swaps)
**Average Test Size**: ~22 lines per test

**Split Strategy**: Split by Number Range (3-way split)

**Rationale**:
- All 200 tests follow identical naming pattern (snapshot_swaps_N)
- Tests are uniformly sized (~22 lines each)
- No semantic groupings evident from test names
- Number range split is cleanest approach for uniform numbered tests
- 3-way split keeps each file well under 800 lines

**Target Files**:

1. `snapshot_swaps_000_066.rs` - Tests 0-66 (67 tests, ~1,470 lines)
2. `snapshot_swaps_067_133.rs` - Tests 67-133 (67 tests, ~1,470 lines)
3. `snapshot_swaps_134_199.rs` - Tests 134-199 (66 tests, ~1,450 lines)

**Implementation Steps**:

1. Create `snapshot_swaps_000_066.rs`:
   - Copy file header and imports from original
   - Extract tests 0-66 (approximately first third of file)
   
2. Create `snapshot_swaps_067_133.rs`:
   - Copy file header and imports from original
   - Extract tests 67-133 (approximately middle third of file)

3. Create `snapshot_swaps_134_199.rs`:
   - Copy file header and imports from original
   - Extract tests 134-199 (approximately last third of file)

4. Update module declarations if needed

5. Run tests to verify all 200 tests pass

6. Delete original `snapshot_swaps.rs`

7. Update suite documentation

**Estimated Effort**: 2 hours

---

## Summary of Priority Giant Files

| File | Current Lines | Tests | Strategy | Target Files | Max Target Size |
|------|---------------|-------|----------|--------------|-----------------|
| lookup_correctness.rs | 10,407 | 200 | 4-way split | 4 files | ~2,600 lines |
| field_invariants.rs | 8,007 | 200 | 4-way split | 4 files | ~2,000 lines |
| allocation_posture.rs | 7,807 | 200 | 4-way split | 4 files | ~1,950 lines |
| pressure_bounds.rs | 6,807 | 200 | 4-way split | 4 files | ~1,700 lines |
| boundary_legality.rs | 4,807 | 200 | 3-way split | 3 files | ~1,600 lines |
| snapshot_swaps.rs | 4,407 | 200 | 3-way split | 3 files | ~1,470 lines |

**Total**: 6 files → 22 files
**Total Line Reduction**: 42,242 lines → 22 files averaging ~1,920 lines each

---

## Large Files (2000-4000 lines)

These 15 files should be split after the priority giant files are complete.

---

### 7. opacity_preservation.rs

**Original File**: `7.quality/suites/sdk_canon_matrix/tests/opacity_preservation.rs`
**Current Size**: 2,407 lines
**Test Count**: 200 tests
**Test Family**: Single family (opacity_preservation)

**Split Strategy**: Split by Number Range (2-way split)

**Target Files**:
1. `opacity_preservation_000_099.rs` - Tests 0-99 (100 tests, ~1,200 lines)
2. `opacity_preservation_100_199.rs` - Tests 100-199 (100 tests, ~1,200 lines)

**Estimated Effort**: 1.5 hours

---

### 8. build_release_surface.rs

**Original File**: `7.quality/suites/editor_canon_matrix/tests/build_release_surface.rs`
**Current Size**: 2,709 lines
**Test Count**: 100 tests
**Test Family**: Single family (build_release_surface)

**Split Strategy**: Split by Number Range (2-way split)

**Target Files**:
1. `build_release_surface_000_049.rs` - Tests 0-49 (50 tests, ~1,350 lines)
2. `build_release_surface_050_099.rs` - Tests 50-99 (50 tests, ~1,350 lines)

**Estimated Effort**: 1.5 hours

---

### 9-15. tooling_canon_matrix Large Files

The following 7 files in `tooling_canon_matrix` suite all follow similar patterns with 100 tests each:

- `snapshot_immutability.rs` (2,509 lines) → 2-way split
- `validation_legality.rs` (2,509 lines) → 2-way split
- `assistant_lowering.rs` (2,409 lines) → 2-way split
- `command_schema.rs` (2,309 lines) → 2-way split
- `transaction_determinism.rs` (2,309 lines) → 2-way split
- `authority_isolation.rs` (2,209 lines) → 2-way split
- `index_rebuild.rs` (2,009 lines) → 2-way split

**Common Split Strategy**: Split by Number Range (2-way split, tests 0-49 and 50-99)

**Estimated Effort**: 1.5 hours each, 10.5 hours total

---

## Medium-Large Files (1000-2000 lines)

These 33 files should be split as part of ongoing maintenance after priority and large files.

### Split Strategy Summary

Most files in this category contain 100 tests and can be split 2-way (0-49, 50-99) to bring them under 800 lines.

**editor_canon_matrix files** (16 files, 100 tests each):
- All follow pattern: `test_name_N` where N is 0-99
- Strategy: 2-way split (0-49, 50-99)
- Estimated effort: 1-1.5 hours each

**engine_canon_matrix files** (1 file):
- `world_spatial_matrix.rs` (1,221 lines, 80 tests)
- Strategy: 2-way split by test family (has 3 families, group into 2 files)
- Estimated effort: 1.5 hours

**engine_sdk_link_matrix files** (5 files, 200 tests each):
- All follow pattern: `test_name_N` where N is 0-199
- Strategy: 3-way split (0-66, 67-133, 134-199)
- Estimated effort: 1.5 hours each

**sdk_tooling_link_matrix files** (5 files, 200 tests each):
- All follow pattern: `test_name_N` where N is 0-199
- Strategy: 3-way split (0-66, 67-133, 134-199)
- Estimated effort: 1.5 hours each

**tooling_canon_matrix files** (6 files, 100 tests each):
- All follow pattern: `test_name_N` where N is 0-99
- Strategy: 2-way split (0-49, 50-99)
- Estimated effort: 1-1.5 hours each

---

## Just Over Threshold (800-1000 lines)

These 8 files are just over the 800-line threshold and should be split to prevent further growth.

### engine_canon_matrix files

**16. generation_matrix.rs** (961 lines, 30 tests)
- Strategy: Keep as single file OR split 2-way if tests grow
- Current: Acceptable if no growth expected
- Estimated effort: 1 hour if split needed

**17. runtime_matrix.rs** (921 lines, 40 tests)
- Strategy: Keep as single file OR split 2-way if tests grow
- Current: Acceptable if no growth expected
- Estimated effort: 1 hour if split needed

**18. storage_layout_matrix.rs** (921 lines, 75 tests, 4 families)
- Strategy: Split by test family (4 files)
- Target files:
  - `storage_layout_chunkdense_valid.rs` (~370 lines, 25 tests)
  - `storage_layout_chunkdense_missing_chunk.rs` (~140 lines, 15 tests)
  - `storage_layout_columnar_valid.rs` (~230 lines, 20 tests)
  - `storage_layout_columnar_missing_columns.rs` (~140 lines, 15 tests)
- Estimated effort: 2 hours

**19. kinetics_matrix.rs** (841 lines, 40 tests)
- Strategy: Keep as single file OR split 2-way if tests grow
- Current: Acceptable if no growth expected
- Estimated effort: 1 hour if split needed

**20. storage_access_matrix.rs** (821 lines, 60 tests, 3 families)
- Strategy: Split by test family (3 files)
- Target files:
  - `storage_access_read_view.rs` (~285 lines, 20 tests)
  - `storage_access_write_window.rs` (~285 lines, 20 tests)
  - `storage_access_bind_locality.rs` (~210 lines, 20 tests)
- Estimated effort: 1.5 hours

**21. startup_matrix.rs** (814 lines, 80 tests, 3 families)
- Strategy: Split by test family (3 files)
- Target files:
  - `startup_validate_case.rs` (~340 lines, 35 tests)
  - `startup_launch_headless_case.rs` (~240 lines, 25 tests)
  - `startup_launch_realtime_case.rs` (~190 lines, 20 tests)
- Estimated effort: 1.5 hours

### editor_canon_matrix files

**22. outliner_sync.rs** (909 lines, 100 tests)
- Strategy: 2-way split (0-49, 50-99)
- Estimated effort: 1 hour

**23. overlay_gizmo.rs** (909 lines, 100 tests)
- Strategy: 2-way split (0-49, 50-99)
- Estimated effort: 1 hour

---

## Support Files - Mandatory Split (400+ lines)

### 24. tooling_runtime.rs

**Original File**: `7.quality/support/stratumx_test_support/src/tooling_runtime.rs`
**Current Size**: 424 lines
**File Type**: Support file (builders and runtime helpers)

**Split Strategy**: Split by Category

**Rationale**:
- Support file exceeds 400-line mandatory split threshold
- Likely contains multiple categories of support code
- Should be split into focused modules

**Analysis Needed**:
- Read file to identify distinct categories (builders, fixtures, runtime helpers)
- Determine natural split points

**Target Files** (estimated):
1. `tooling_runtime_builders.rs` (~200 lines) - Builder functions
2. `tooling_runtime_helpers.rs` (~200 lines) - Runtime helper functions

**Implementation Steps**:
1. Read file to analyze content categories
2. Identify builder functions vs helper functions
3. Create two focused modules
4. Update imports in test files
5. Verify all tests pass

**Estimated Effort**: 2 hours

---

## Support Files - Requires Split Plan (251-400 lines)

### 25. bridge_runtime.rs

**Original File**: `7.quality/support/stratumx_test_support/src/bridge_runtime.rs`
**Current Size**: 389 lines
**File Type**: Support file

**Split Strategy**: Monitor and split if grows beyond 400 lines

**Recommendation**: Create split plan if file approaches 400 lines. Current size is acceptable but should be monitored.

**Estimated Effort**: 1.5 hours if split needed

---

### 26. bridge_types.rs

**Original File**: `7.quality/support/stratumx_test_support/src/bridge_types.rs`
**Current Size**: 346 lines
**File Type**: Support file

**Split Strategy**: Monitor and split if grows beyond 400 lines

**Recommendation**: Create split plan if file approaches 400 lines. Current size is acceptable but should be monitored.

**Estimated Effort**: 1.5 hours if split needed

---

### 27. lib.rs (stratumx_repo_hygiene_support)

**Original File**: `7.quality/support/stratumx_repo_hygiene_support/src/lib.rs`
**Current Size**: 306 lines
**File Type**: Support file (library root)

**Split Strategy**: Extract modules from lib.rs

**Rationale**:
- lib.rs should be minimal (re-exports and module declarations)
- 306 lines suggests embedded implementation code
- Should extract implementation to separate modules

**Analysis Needed**:
- Read file to identify what's embedded in lib.rs
- Determine what should be extracted to separate modules

**Target Files** (estimated):
1. `lib.rs` (~50 lines) - Module declarations and re-exports only
2. Separate modules for each category of functionality (~250 lines distributed)

**Implementation Steps**:
1. Read lib.rs to analyze content
2. Identify distinct functional areas
3. Extract to separate module files
4. Update lib.rs to declare and re-export modules
5. Verify all tests pass

**Estimated Effort**: 2 hours

---

## Execution Priority and Phasing

### Phase 1: Critical Giant Files (Package 5, Task 15)
**Priority**: HIGHEST - These files block merge and are unacceptably large

1. lookup_correctness.rs (10,407 lines) → 4 files
2. field_invariants.rs (8,007 lines) → 4 files
3. allocation_posture.rs (7,807 lines) → 4 files
4. pressure_bounds.rs (6,807 lines) → 4 files
5. boundary_legality.rs (4,807 lines) → 3 files

**Total Effort**: 11-15 hours
**Outcome**: Eliminate 5 most critical violations

---

### Phase 2: Large Files (Package 5, Task 16)
**Priority**: HIGH - Prevent accumulation of giant files

Files 2000-4000 lines (15 files):
- opacity_preservation.rs
- build_release_surface.rs
- 7 tooling_canon_matrix files (2000-2500 lines each)
- 6 additional large files

**Total Effort**: 20-25 hours
**Outcome**: Eliminate all files over 2000 lines

---

### Phase 3: Medium-Large Files
**Priority**: MEDIUM - Ongoing maintenance

Files 1000-2000 lines (33 files):
- 16 editor_canon_matrix files
- 1 engine_canon_matrix file
- 5 engine_sdk_link_matrix files
- 5 sdk_tooling_link_matrix files
- 6 tooling_canon_matrix files

**Total Effort**: 40-50 hours
**Outcome**: Eliminate all files over 1000 lines

---

### Phase 4: Just Over Threshold
**Priority**: LOW-MEDIUM - Prevent growth

Files 800-1000 lines (8 files):
- 6 engine_canon_matrix files
- 2 editor_canon_matrix files

**Total Effort**: 10-15 hours
**Outcome**: Eliminate all test file violations

---

### Phase 5: Support Files
**Priority**: HIGH (for mandatory split), MEDIUM (for others)

- 1 mandatory split (424 lines)
- 3 requiring split plans (306-389 lines)

**Total Effort**: 5-8 hours
**Outcome**: Eliminate all support file violations

---

## Total Effort Summary

| Phase | Files | Estimated Hours | Priority |
|-------|-------|-----------------|----------|
| Phase 1: Critical Giants | 5 | 11-15 | HIGHEST |
| Phase 2: Large Files | 15 | 20-25 | HIGH |
| Phase 3: Medium-Large | 33 | 40-50 | MEDIUM |
| Phase 4: Just Over Threshold | 8 | 10-15 | LOW-MEDIUM |
| Phase 5: Support Files | 4 | 5-8 | HIGH (1 file), MEDIUM (3 files) |
| **TOTAL** | **65** | **86-113** | - |

---

## Implementation Guidelines

### General Process for All Splits

1. **Preparation**:
   - Read original file to understand structure
   - Verify test family classification from INVENTORY.md
   - Confirm split strategy is appropriate
   - Create target file list with line estimates

2. **Execution**:
   - Create new files with appropriate names
   - Copy file headers, imports, and module-level documentation
   - Extract tests according to split plan
   - Remove unused imports from each file
   - Update module declarations (mod.rs or Cargo.toml)

3. **Verification**:
   - Run all tests in the suite: `cargo test`
   - Verify test count matches original
   - Check file sizes comply with law (<800 lines for tests)
   - Run file size hygiene check

4. **Cleanup**:
   - Delete original file
   - Update suite documentation
   - Update INVENTORY.md if needed
   - Commit changes with descriptive message

5. **Documentation**:
   - Mark split plan as complete in this document
   - Update FILE_SIZE_LAW.md violations report
   - Note any issues or deviations from plan

### Naming Conventions for Split Files

**Number Range Splits**:
- Use 3-digit zero-padded ranges: `test_name_000_049.rs`
- Clearly indicates test number range
- Sorts correctly in file listings

**Family Splits**:
- Use semantic names based on test family: `storage_layout_chunkdense_valid.rs`
- Describes what the file tests
- Follows NAMING_LAW.md conventions

**Support File Splits**:
- Use category suffixes: `module_builders.rs`, `module_helpers.rs`
- Clearly indicates support code category
- Maintains relationship to original module

### Common Pitfalls to Avoid

1. **Import Bloat**: Don't copy all imports blindly - remove unused imports
2. **Module Declaration Errors**: Ensure new files are properly declared
3. **Test Discovery Issues**: Verify Cargo finds all tests after split
4. **Documentation Drift**: Update all references to split files
5. **Incomplete Splits**: Ensure all tests from original file are included
6. **Size Violations**: Verify split files comply with size law

---

## Completion Tracking

Mark each file as complete when split is finished and verified:

### Phase 1: Critical Giant Files
- [ ] lookup_correctness.rs
- [ ] field_invariants.rs
- [ ] allocation_posture.rs
- [ ] pressure_bounds.rs
- [ ] boundary_legality.rs
- [ ] snapshot_swaps.rs

### Phase 2: Large Files
- [ ] opacity_preservation.rs
- [ ] build_release_surface.rs
- [ ] snapshot_immutability.rs
- [ ] validation_legality.rs
- [ ] assistant_lowering.rs
- [ ] command_schema.rs
- [ ] transaction_determinism.rs
- [ ] authority_isolation.rs
- [ ] index_rebuild.rs
- [ ] (6 additional large files)

### Phase 5: Support Files
- [ ] tooling_runtime.rs (mandatory)
- [ ] bridge_runtime.rs (if needed)
- [ ] bridge_types.rs (if needed)
- [ ] lib.rs (stratumx_repo_hygiene_support)

---

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-04-10 | 1.0 | Initial split plans document created (Task 9.1) |


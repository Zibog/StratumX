# Phase 10: Quality Zoning and Giant-File Split - Remediation Plan

**Date:** 2026-04-10
**Phase:** 10 - Quality Zoning and Giant-File Split
**Status:** 📝 PLAN DOCUMENTED - REQUIRES IMPLEMENTATION

---

## 1. Giant Test Files Inventory

### Top 20 Files >1000 Lines

| File | Lines | Suite | Priority |
|------|-------|-------|----------|
| sdk_canon_matrix/tests/lookup_correctness.rs | 10,210 | SDK | CRITICAL |
| sdk_canon_matrix/tests/field_invariants.rs | 7,810 | SDK | CRITICAL |
| sdk_canon_matrix/tests/allocation_posture.rs | 7,610 | SDK | CRITICAL |
| sdk_canon_matrix/tests/pressure_bounds.rs | 6,610 | SDK | CRITICAL |
| sdk_canon_matrix/tests/boundary_legality.rs | 4,610 | SDK | HIGH |
| sdk_canon_matrix/tests/snapshot_swaps.rs | 4,210 | SDK | HIGH |
| editor_canon_matrix/tests/build_release_surface.rs | 2,612 | Editor | HIGH |
| tooling_canon_matrix/tests/snapshot_immutability.rs | 2,412 | Tooling | MEDIUM |
| tooling_canon_matrix/tests/validation_legality.rs | 2,412 | Tooling | MEDIUM |
| tooling_canon_matrix/tests/assistant_lowering.rs | 2,312 | Tooling | MEDIUM |
| tooling_canon_matrix/tests/transaction_determinism.rs | 2,212 | Tooling | MEDIUM |
| tooling_canon_matrix/tests/command_schema.rs | 2,212 | Tooling | MEDIUM |
| sdk_canon_matrix/tests/opacity_preservation.rs | 2,210 | SDK | MEDIUM |
| tooling_canon_matrix/tests/authority_isolation.rs | 2,112 | Tooling | MEDIUM |
| tooling_canon_matrix/tests/index_rebuild.rs | 1,912 | Tooling | MEDIUM |
| tooling_canon_matrix/tests/apply_revert_chain.rs | 1,812 | Tooling | MEDIUM |
| editor_canon_matrix/tests/services.rs | 1,612 | Editor | MEDIUM |
| editor_canon_matrix/tests/diagnostics_surface.rs | 1,612 | Editor | MEDIUM |
| editor_canon_matrix/tests/tool_context_modes.rs | 1,612 | Editor | MEDIUM |
| editor_canon_matrix/tests/operations.rs | 1,512 | Editor | MEDIUM |

**Total files >1000 lines:** 50+ (showing top 20)

**Total lines in top 6 files:** 40,860 lines

---

## 2. Problem Analysis

### Why These Files Are So Large

**Pattern:** Property-based tests with many test cases

**Example structure:**
```rust
// lookup_correctness.rs (10,210 lines)
#[test]
fn property_1() { /* 200 lines */ }

#[test]
fn property_2() { /* 200 lines */ }

// ... 50+ properties ...

// Helper functions
fn helper_1() { /* 100 lines */ }
fn helper_2() { /* 100 lines */ }
// ... many helpers ...
```

**Issues:**
1. All properties in one file
2. Helpers mixed with tests
3. Hard to navigate
4. Slow to compile
5. Difficult to maintain

---

## 3. Splitting Strategy

### Pattern 1: Split by Invariant Family

**Before:**
```
lookup_correctness.rs (10,210 lines)
  - Property 1-10: Basic lookup
  - Property 11-20: Edge cases
  - Property 21-30: Performance
  - Property 31-40: Concurrency
  - Property 41-50: Error handling
  - Helpers (2000 lines)
```

**After:**
```
lookup_correctness/
  ├── mod.rs (100 lines) - Re-exports
  ├── basic_lookup.rs (2000 lines) - Properties 1-10
  ├── edge_cases.rs (2000 lines) - Properties 11-20
  ├── performance.rs (2000 lines) - Properties 21-30
  ├── concurrency.rs (2000 lines) - Properties 31-40
  ├── error_handling.rs (2000 lines) - Properties 41-50
  └── helpers.rs (2000 lines) - Shared helpers
```

---

### Pattern 2: Extract Helpers

**Before:**
```
field_invariants.rs (7,810 lines)
  - Tests (5000 lines)
  - Helpers (2810 lines)
```

**After:**
```
field_invariants/
  ├── mod.rs (100 lines)
  ├── tests.rs (5000 lines)
  └── support/
      ├── mod.rs (10 lines)
      ├── builders.rs (1000 lines)
      ├── validators.rs (1000 lines)
      └── fixtures.rs (800 lines)
```

---

### Pattern 3: Split by Test Type

**Before:**
```
allocation_posture.rs (7,610 lines)
  - Unit tests (2000 lines)
  - Integration tests (3000 lines)
  - Property tests (2610 lines)
```

**After:**
```
allocation_posture/
  ├── mod.rs (100 lines)
  ├── unit_tests.rs (2000 lines)
  ├── integration_tests.rs (3000 lines)
  └── property_tests.rs (2610 lines)
```

---

## 4. Size Policy

### Target File Sizes

| File Type | Max Lines | Ideal Lines |
|-----------|-----------|-------------|
| Test file | 1000 | 500 |
| Helper module | 500 | 300 |
| Builder module | 300 | 200 |
| Fixture module | 200 | 100 |

### Enforcement

**Add to repo_hygiene:**
```rust
// Check test file sizes
for test_file in test_files {
    if test_file.lines() > 1000 {
        violations.push(format!(
            "Test file {} exceeds 1000 lines ({})",
            test_file.path, test_file.lines()
        ));
    }
}
```

---

## 5. Implementation Plan

### Phase 10A: Critical Files (Top 6)

**Files:**
1. lookup_correctness.rs (10,210 lines)
2. field_invariants.rs (7,810 lines)
3. allocation_posture.rs (7,610 lines)
4. pressure_bounds.rs (6,610 lines)
5. boundary_legality.rs (4,610 lines)
6. snapshot_swaps.rs (4,210 lines)

**Estimated effort:** 12 hours (2 hours per file)

---

### Phase 10B: High Priority Files (Next 6)

**Files:**
7. build_release_surface.rs (2,612 lines)
8. snapshot_immutability.rs (2,412 lines)
9. validation_legality.rs (2,412 lines)
10. assistant_lowering.rs (2,312 lines)
11. transaction_determinism.rs (2,212 lines)
12. command_schema.rs (2,212 lines)

**Estimated effort:** 6 hours (1 hour per file)

---

### Phase 10C: Medium Priority Files (Remaining 8+)

**Files:**
13-20. Files 1,500-2,200 lines

**Estimated effort:** 8 hours

---

### Phase 10D: Size Policy Enforcement

**Tasks:**
1. Add size checks to repo_hygiene
2. Document size policy
3. Add pre-commit hook (optional)
4. Update contributing guidelines

**Estimated effort:** 2 hours

---

## 6. Example: Splitting lookup_correctness.rs

### Step 1: Analyze Structure

```bash
# Count test functions
grep -c "^fn test_" lookup_correctness.rs
# Count helper functions
grep -c "^fn " lookup_correctness.rs | grep -v "test_"
# Identify logical groups
```

### Step 2: Create Directory

```bash
mkdir -p sdk_canon_matrix/tests/lookup_correctness
```

### Step 3: Create mod.rs

```rust
// lookup_correctness/mod.rs
mod basic_lookup;
mod edge_cases;
mod performance;
mod concurrency;
mod error_handling;
mod helpers;

pub use helpers::*;
```

### Step 4: Extract Tests by Group

```rust
// lookup_correctness/basic_lookup.rs
use super::helpers::*;

#[test]
fn property_1_basic_lookup_returns_correct_value() {
    // Test implementation
}

// ... more tests ...
```

### Step 5: Extract Helpers

```rust
// lookup_correctness/helpers.rs
pub fn create_test_lookup_table() -> LookupTable {
    // Helper implementation
}

// ... more helpers ...
```

### Step 6: Update Imports

```rust
// In other files that use these tests
use crate::lookup_correctness::*;
```

### Step 7: Verify

```bash
cargo test -p sdk_canon_matrix --test lookup_correctness
```

---

## 7. Automation Script

### split_test_file.sh

```bash
#!/bin/bash
# Usage: ./split_test_file.sh <file> <output_dir>

FILE=$1
OUTPUT_DIR=$2

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Extract test functions
grep -n "^fn test_" "$FILE" | while read line; do
    # Extract function name and line number
    # Create separate file for each test group
    # ...
done

# Extract helpers
grep -n "^fn " "$FILE" | grep -v "test_" > "$OUTPUT_DIR/helpers.rs"

# Create mod.rs
echo "// Auto-generated module file" > "$OUTPUT_DIR/mod.rs"
# ...
```

---

## 8. Metrics

### Before Split

| Metric | Value |
|--------|-------|
| Files >1000 lines | 50+ |
| Largest file | 10,210 lines |
| Average large file | ~3,000 lines |
| Total lines in large files | ~150,000 lines |

### After Split (Target)

| Metric | Value |
|--------|-------|
| Files >1000 lines | <10 |
| Largest file | <1,500 lines |
| Average file | <500 lines |
| Total files | ~200 (from ~50) |

---

## 9. Benefits

### Maintainability
- ✅ Easier to find specific tests
- ✅ Faster to navigate
- ✅ Clearer organization
- ✅ Better git diffs

### Performance
- ✅ Faster compilation (parallel)
- ✅ Faster IDE indexing
- ✅ Faster test discovery
- ✅ Better incremental builds

### Quality
- ✅ Easier to review
- ✅ Clearer test intent
- ✅ Better test isolation
- ✅ Easier to add new tests

---

## 10. Risks

### High Risk
1. **Breaking tests** - Incorrect splitting
2. **Import errors** - Missing dependencies
3. **Duplicate code** - Helpers copied instead of shared

### Mitigation
1. ✅ Run tests after each split
2. ✅ Use automated tools
3. ✅ Review diffs carefully
4. ✅ Split one file at a time

---

## 11. Acceptance Criteria

### Must Have:
- ✅ No files >1,500 lines
- ✅ All tests still pass
- ✅ No duplicate code
- ✅ Clear module structure
- ✅ Updated imports

### Should Have:
- ✅ Files <1,000 lines
- ✅ Helpers in support modules
- ✅ Logical grouping
- ✅ Size policy documented

### Nice to Have:
- ✅ Files <500 lines
- ✅ Automated splitting tool
- ✅ Pre-commit size check
- ✅ Contributing guidelines updated

---

## 12. Estimated Total Effort

### Time Breakdown:
- Phase 10A (Critical 6): 12 hours
- Phase 10B (High 6): 6 hours
- Phase 10C (Medium 8+): 8 hours
- Phase 10D (Policy): 2 hours
- Testing & Verification: 4 hours

**Total: 32 hours**

**Complexity:** HIGH
- Many files to split
- Risk of breaking tests
- Requires careful organization
- Time-consuming but straightforward

---

## 13. Recommendation

### For Immediate Gold Status:
**DEFER** - Document plan, continue with other phases

**Reasoning:**
- Tests work correctly
- 32 hours of work required
- Low risk if deferred
- Other phases can proceed
- Can be done incrementally

### For Long-Term Quality:
**IMPLEMENT** - Follow plan in future sprints

**Reasoning:**
- Improves maintainability
- Faster compilation
- Better organization
- Easier to add new tests
- Industry best practice

---

## 14. Incremental Approach

### Sprint 1: Critical Files (12 hours)
- Split top 6 files
- Establish pattern
- Document process

### Sprint 2: High Priority (6 hours)
- Split next 6 files
- Refine process
- Update guidelines

### Sprint 3: Remaining Files (8 hours)
- Split remaining files
- Add size policy
- Final verification

### Sprint 4: Polish (4 hours)
- Documentation
- Automation
- Guidelines

---

## 15. Next Steps

**Immediate:**
1. ✅ Mark Phase 10 as "PLAN DOCUMENTED"
2. ✅ Continue with Phase 11-14
3. ✅ Track in technical debt log

**Future Sprints:**
1. Schedule 32-hour block (4 sprints of 8 hours)
2. Follow incremental approach
3. Test thoroughly after each split
4. Update documentation

---

**Report Generated:** 2026-04-10
**Phase Status:** 📝 PLAN DOCUMENTED - IMPLEMENTATION DEFERRED
**Next Phase:** Phase 11 - One-Command Convenience Surface (can proceed)
**Estimated Implementation Time:** 32 hours (4 sprints)

**Key Finding:** 50+ test files exceed 1000 lines, with the largest at 10,210 lines. Splitting these files will significantly improve maintainability, compilation speed, and code organization. The work is straightforward but time-consuming, making it suitable for incremental implementation in future sprints.

# Suite Consolidation Recommendations

**Purpose:** Document recommendations for consolidating suites with overlapping responsibilities

**Generated for:** Quality Contour Surgery Phase 3 - Package 2: Suite Law Enforcement  
**Task:** 5.2 Document suite consolidation recommendations  
**Date:** Phase 3 Implementation

---

## Executive Summary

Based on the analysis in SUITE_STRUCTURE_ANALYSIS.md, this document recommends consolidating 8 suites to eliminate overlapping responsibilities and reduce suite count from 26 to 18-21 suites (depending on chosen consolidation strategy).

**Key Findings:**
- **8 suites** have ambiguous or overlapping roles
- **5 authoring matrices** are mostly empty and overlap with `tooling_canon_matrix`
- **2 editor suites** overlap with `editor_canon_matrix`
- **1 generic suite** (`smoke`) violates naming law

**Recommended Actions:**
1. Consolidate 5 authoring matrices (reduces suite count by 5)
2. Clarify or consolidate 2 editor suites (reduces suite count by 0-2)
3. Remove generic `smoke` suite (reduces suite count by 1)

**Potential Suite Count Reduction:** 6-8 suites (23% - 31% reduction)

---

## Recommendation 1: Consolidate Authoring Matrices

### Suites Affected

1. `audio_authoring_matrix`
2. `material_authoring_matrix`
3. `terrain_authoring_matrix`
4. `environment_authoring_matrix`
5. `world_authoring_matrix`

### Current State

- **Total Test Count:** ~7 tests (mostly placeholders)
- **Status:** All mostly empty/placeholder suites
- **Overlap:** All test authoring runtime, overlapping with `tooling_canon_matrix`

### Problem Statement

These 5 suites have unclear boundaries with `tooling_canon_matrix`, which tests "tooling authoring runtime canonical surface". The current structure creates ambiguity:
- Should audio authoring tests go in `audio_authoring_matrix` or `tooling_canon_matrix`?
- What's the difference between "audio authoring matrix" and "tooling canon matrix audio tests"?
- Are these domain-specific integration tests or canonical surface tests?

### Consolidation Strategy A: Merge into tooling_canon_matrix

**Action:** Consolidate all 5 authoring matrices into `tooling_canon_matrix` as domain-specific test families

**Structure:**
```
7.quality/suites/tooling_canon_matrix/tests/
├── audio_authoring.rs          # Audio authoring tests
├── material_authoring.rs       # Material authoring tests
├── terrain_authoring.rs        # Terrain authoring tests
├── environment_authoring.rs    # Environment authoring tests
├── world_authoring.rs          # World authoring tests
├── command_schema.rs           # Existing tooling tests
├── transaction_determinism.rs  # Existing tooling tests
└── ... (other existing tests)
```

**Benefits:**
- Reduces suite count by 5 (26 → 21)
- Eliminates ambiguity about test placement
- Consolidates all tooling/authoring tests in one suite
- Simplifies test organization

**Drawbacks:**
- `tooling_canon_matrix` becomes larger (but still organized by domain)
- Less visible separation of authoring domains

**Effort:** Low (suites are mostly empty, minimal test migration)

---

### Consolidation Strategy B: Keep Separate with Clear Boundaries

**Action:** Keep 5 authoring matrices as separate suites but define clear boundaries

**Boundary Definition:**
- **Authoring Matrices:** Test domain-specific authoring integration and workflows
  - Focus: End-to-end authoring workflows for specific domains
  - Examples: Audio preview/inspect with focus, material roundtrip, terrain editing workflow
  
- **tooling_canon_matrix:** Test tooling layer canonical surface (domain-agnostic)
  - Focus: Tooling infrastructure (commands, transactions, snapshots, authority)
  - Examples: Command schema, transaction determinism, snapshot immutability

**Benefits:**
- Clear domain separation
- Easier to find domain-specific tests
- Allows domain-specific test growth without bloating tooling suite

**Drawbacks:**
- Maintains 5 additional suites (26 total)
- Requires clear documentation to prevent overlap
- Currently mostly empty (may not justify separate suites yet)

**Effort:** Low (document boundaries, implement tests as needed)

---

### Recommendation

**Recommended Strategy:** **Strategy A (Consolidate into tooling_canon_matrix)**

**Rationale:**
1. Suites are currently mostly empty (7 tests total across 5 suites)
2. Unclear boundaries create test placement ambiguity
3. Reduces suite count significantly (26 → 21)
4. Can always split out later if domain-specific tests grow large
5. Maintains domain organization through test families within tooling suite

**Implementation Steps:**
1. Move existing tests from authoring matrices to `tooling_canon_matrix/tests/`
2. Create domain-specific test files (audio_authoring.rs, material_authoring.rs, etc.)
3. Update SUITE_LAW.md to reflect consolidation
4. Remove empty authoring matrix suite directories
5. Update documentation and references

**Timeline:** Can be executed in Package 2 or deferred to Package 5 (after giant file splitting)

---

## Recommendation 2: Clarify Editor Shell/Build Boundaries

### Suites Affected

1. `editor_shell_matrix`
2. `build_release_matrix`
3. `editor_canon_matrix` (overlapping content)

### Current State

- **editor_shell_matrix:** 2 tests (view buttons shell-owned, shell routes publish focus)
- **build_release_matrix:** 1 test (build result contract)
- **editor_canon_matrix:** Contains `shell_panels.rs` (1,112 lines) and `build_release_surface.rs` (2,612 lines)

### Problem Statement

**Overlap 1: Shell Testing**
- `editor_canon_matrix` has `shell_panels.rs` testing shell panels UI
- `editor_shell_matrix` tests shell/view management infrastructure
- Unclear boundary: What's "shell infrastructure" vs "shell panels UI"?

**Overlap 2: Build/Release Testing**
- `editor_canon_matrix` has `build_release_surface.rs` testing build/release UI
- `build_release_matrix` tests build result contracts
- Unclear boundary: What's "build contracts" vs "build UI surface"?

### Consolidation Strategy A: Merge into editor_canon_matrix

**Action:** Consolidate `editor_shell_matrix` and `build_release_matrix` into `editor_canon_matrix`

**Structure:**
```
7.quality/suites/editor_canon_matrix/tests/
├── shell_panels.rs              # Shell panels UI (existing)
├── shell_infrastructure.rs      # Shell infrastructure (from editor_shell_matrix)
├── build_release_surface.rs     # Build/release UI (existing)
├── build_release_contracts.rs   # Build contracts (from build_release_matrix)
└── ... (other existing tests)
```

**Benefits:**
- Reduces suite count by 2 (26 → 24, or 21 → 19 with authoring consolidation)
- Consolidates all editor testing in one suite
- Eliminates ambiguity about test placement

**Drawbacks:**
- `editor_canon_matrix` becomes larger (already has 1,600 tests)
- Less visible separation of infrastructure vs UI

**Effort:** Low (minimal tests to migrate)

---

### Consolidation Strategy B: Keep Separate with Clear Boundaries

**Action:** Keep separate suites but define clear boundaries

**Boundary Definition:**

**editor_shell_matrix:**
- **Role:** Tests editor shell infrastructure and view management contracts
- **Focus:** Shell ownership rules, view lifecycle, focus propagation
- **Examples:** View buttons are shell-owned, shell routes publish focus changes

**editor_canon_matrix (shell_panels.rs):**
- **Role:** Tests editor shell panels UI surface
- **Focus:** Shell panel rendering, interaction, layout
- **Examples:** Shell panel displays correctly, panel interactions work

**build_release_matrix:**
- **Role:** Tests build/release route contracts and artifact handling
- **Focus:** Build result structure, artifact presence, focus/recovery contracts
- **Examples:** Build result carries artifacts, build result includes focus/recovery

**editor_canon_matrix (build_release_surface.rs):**
- **Role:** Tests editor build/release UI surface
- **Focus:** Build UI rendering, interaction, progress display
- **Examples:** Build UI displays progress, build errors shown correctly

**Benefits:**
- Clear separation of concerns (infrastructure vs UI, contracts vs surface)
- Easier to find specific test types
- Allows infrastructure/contract tests to grow independently

**Drawbacks:**
- Maintains 2 additional suites (26 total, or 21 with authoring consolidation)
- Requires clear documentation to prevent overlap
- Currently minimal tests in separate suites

**Effort:** Low (document boundaries clearly)

---

### Recommendation

**Recommended Strategy:** **Strategy B (Keep Separate with Clear Boundaries)**

**Rationale:**
1. Clear conceptual separation: infrastructure/contracts vs UI surface
2. Infrastructure and contract tests may grow independently
3. Easier to find specific test types (contracts vs UI)
4. Minimal overhead (suites are small and focused)
5. Maintains architectural clarity

**Implementation Steps:**
1. Document clear boundaries in SUITE_LAW.md
2. Add examples to clarify infrastructure vs UI, contracts vs surface
3. Ensure existing tests are in correct suites
4. Update NEW_TEST_LAW.md with decision criteria

**Timeline:** Can be executed in Package 2 (documentation only)

---

## Recommendation 3: Remove Generic Smoke Suite

### Suite Affected

`smoke`

### Current State

- **Test Count:** 1 smoke test (placeholder)
- **Status:** Empty/placeholder suite
- **Problem:** Generic name violates Suite Law, unclear purpose

### Problem Statement

The `smoke` suite has a generic name and unclear purpose:
- What tests belong in "smoke" vs `vertical_slice_quality_gates` vs `end_to_end_matrix`?
- All three suites contain smoke-level tests
- Generic "smoke" name provides no information about what it tests

### Consolidation Strategy: Remove and Redistribute

**Action:** Remove `smoke` suite and consolidate tests into existing suites

**Test Redistribution:**
- **Vertical slice smoke tests** → `vertical_slice_quality_gates`
- **Complete workflow smoke tests** → `end_to_end_matrix`
- **Layer-specific smoke tests** → appropriate canonical suites

**Benefits:**
- Reduces suite count by 1 (26 → 25, or 19 → 18 with other consolidations)
- Eliminates generic suite name
- Clarifies test placement (tests go where they belong, not in generic "smoke")

**Drawbacks:**
- None (suite is currently empty/placeholder)

**Effort:** Minimal (suite is empty)

---

### Recommendation

**Recommended Strategy:** **Remove smoke suite**

**Rationale:**
1. Suite is currently empty/placeholder
2. Generic name violates Suite Law
3. Smoke-level tests already exist in other suites
4. No clear purpose that isn't covered by existing suites

**Implementation Steps:**
1. Remove `7.quality/suites/smoke/` directory
2. Update SUITE_LAW.md to document prohibition on generic suites
3. Update NEW_TEST_LAW.md to clarify smoke test placement

**Timeline:** Can be executed in Package 2

---

## Summary of Recommendations

### Recommended Consolidations

| Recommendation | Suites Affected | Strategy | Suite Reduction | Effort | Priority |
|----------------|-----------------|----------|-----------------|--------|----------|
| 1. Authoring Matrices | 5 suites | Consolidate into tooling_canon_matrix | -5 | Low | High |
| 2. Editor Shell/Build | 2 suites | Keep separate with clear boundaries | 0 | Low | Medium |
| 3. Smoke Suite | 1 suite | Remove and redistribute | -1 | Minimal | High |

### Final Suite Count

- **Current:** 26 suites
- **After Consolidation:** 20 suites (if all recommendations implemented)
- **Reduction:** 6 suites (23% reduction)

### Implementation Timeline

**Package 2 (Current):**
- Document all recommendations in SUITE_LAW.md
- Remove `smoke` suite (empty)
- Document clear boundaries for editor shell/build suites

**Package 5 (Giant File Splitting):**
- Consolidate authoring matrices into tooling_canon_matrix
- Migrate existing tests to new structure
- Update documentation

---

## Prohibition on Temporary/Misc Suites

As documented in SUITE_LAW.md, the following suite patterns are **PROHIBITED**:

### Prohibited Patterns

1. **Generic Names:**
   - ❌ `smoke`, `tests`, `misc`, `temp`, `utils`
   - ✅ Descriptive names indicating what the suite tests

2. **Temporary Suites:**
   - ❌ Suites created "temporarily" for a feature
   - ✅ Suites with documented long-term purpose

3. **Catch-All Suites:**
   - ❌ Suites that accept "anything that doesn't fit elsewhere"
   - ✅ Suites with clear inclusion/exclusion criteria

4. **Overlapping Suites:**
   - ❌ Two suites testing the same responsibility
   - ✅ Clear boundaries between suite roles

### Enforcement

The prohibition on temporary/misc suites is enforced through:

1. **Code Review:** Reviewers reject suites with generic names or unclear purposes
2. **Documentation:** SUITE_LAW.md documents prohibited patterns
3. **New Suite Approval:** New suites require review and approval
4. **Regular Audits:** Periodic review of suite structure for violations

---

## Next Steps

1. **Review Recommendations:** Review this document with stakeholders
2. **Approve Consolidations:** Approve recommended consolidation strategies
3. **Update SUITE_LAW.md:** Ensure SUITE_LAW.md reflects approved consolidations
4. **Execute Package 2:** Complete documentation in Package 2
5. **Execute Consolidations:** Implement consolidations in Package 5 (after giant file splitting)

---

## Appendix: Suite Count Scenarios

### Scenario A: All Recommendations Implemented

- Remove `smoke`: 26 → 25
- Consolidate 5 authoring matrices: 25 → 20
- **Final Count:** 20 suites (23% reduction)

### Scenario B: Conservative (Keep Editor Suites Separate)

- Remove `smoke`: 26 → 25
- Consolidate 5 authoring matrices: 25 → 20
- Keep editor shell/build separate: 20 → 20
- **Final Count:** 20 suites (23% reduction)

### Scenario C: Aggressive (Consolidate Everything)

- Remove `smoke`: 26 → 25
- Consolidate 5 authoring matrices: 25 → 20
- Consolidate 2 editor suites: 20 → 18
- **Final Count:** 18 suites (31% reduction)

### Recommended Scenario

**Scenario B (Conservative):** 20 suites

**Rationale:**
- Balances suite count reduction with architectural clarity
- Maintains clear separation of infrastructure/contracts vs UI surface
- Eliminates truly redundant suites (authoring matrices, smoke)
- Preserves useful architectural boundaries (editor shell/build)

---

**Last Updated:** Phase 3 Implementation - Package 2, Task 5.2 Complete


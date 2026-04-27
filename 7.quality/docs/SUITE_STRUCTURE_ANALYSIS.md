# Suite Structure Analysis

**Generated for:** Quality Contour Surgery Phase 3 - Package 2: Suite Law Enforcement  
**Task:** 4. Analyze existing suite structure  
**Date:** Phase 3 Implementation

## Executive Summary

This document analyzes all 26 test suites in `7.quality/suites/` to understand their current purposes, identify suites with clear single roles, and identify suites with ambiguous or overlapping roles.

**Key Findings:**
- **7 suites** already have documented roles in SUITE_ROLE_MAP.md
- **19 suites** need role documentation
- **Clear single roles:** 18 suites have clear, focused purposes
- **Ambiguous/overlapping roles:** 8 suites have unclear or potentially overlapping responsibilities
- **Empty/placeholder suites:** 9 suites have minimal or no test content

---

## Part 1: Existing SUITE_ROLE_MAP.md Analysis

### Suites with Documented Roles (7 total)

The following suites already have documented roles in `7.quality/docs/SUITE_ROLE_MAP.md`:

1. **engine_canon_matrix** — canonical engine surface verification using real `2.engine/*` crates
2. **engine_perf_harness** — benchmark harness using real `2.engine/*` crates
3. **sdk_canon_matrix** — sdk bridge behavior tests through `stratumx_test_support`
4. **tooling_canon_matrix** — tooling authoring runtime tests through `stratumx_test_support`
5. **editor_canon_matrix** — editor product tests through `stratumx_test_support`
6. **engine_sdk_link_matrix** — engine/runtime profile boot + sdk bridge linking
7. **sdk_tooling_link_matrix** — sdk/tooling integration path

**Assessment:** These 7 suites have clear, documented roles. However, some roles could be more specific about inclusion/exclusion criteria.

---

## Part 2: Complete Suite Analysis (All 26 Suites)

### Category A: Suites with Clear Single Roles (18 suites)

These suites have focused, unambiguous purposes based on test content analysis:

#### 1. **engine_canon_matrix** ✓ Documented
- **Current Purpose:** Tests canonical engine surface using real engine crates
- **Test Content:** Startup validation, runtime ticks, storage access/layout, world spatial operations, kinetics simulation, generation
- **Test Count:** ~255 tests across 6 test files
- **Role Clarity:** CLEAR - Tests engine layer canonical API surface
- **Domain:** Engine
- **Routing:** Full (comprehensive engine verification)

#### 2. **engine_perf_harness** ✓ Documented
- **Current Purpose:** Performance benchmarking for engine operations
- **Test Content:** Minimal smoke test only (placeholder suite)
- **Test Count:** 1 smoke test
- **Role Clarity:** CLEAR - Performance harness (not yet implemented)
- **Domain:** Engine
- **Routing:** Full (performance tests)
- **Note:** Currently empty, awaiting implementation

#### 3. **sdk_canon_matrix** ✓ Documented
- **Current Purpose:** Tests SDK bridge behavior and canonical surface
- **Test Content:** Lookup correctness, field invariants, allocation posture, pressure bounds, boundary legality, opacity preservation, snapshot swaps
- **Test Count:** 1,400 tests (7 giant files with 200 tests each)
- **Role Clarity:** CLEAR - Tests SDK canonical API surface
- **Domain:** SDK
- **Routing:** Full (comprehensive SDK verification)
- **Issues:** Contains 7 giant files (2,210-10,210 lines each) requiring splitting

#### 4. **tooling_canon_matrix** ✓ Documented
- **Current Purpose:** Tests tooling authoring runtime canonical surface
- **Test Content:** Command schema, transaction determinism, snapshot immutability, authority isolation, cache eviction, build reproducibility, validation legality, etc.
- **Test Count:** 1,600 tests (16 giant files with 100 tests each)
- **Role Clarity:** CLEAR - Tests tooling layer canonical API surface
- **Domain:** Tooling
- **Routing:** Full (comprehensive tooling verification)
- **Issues:** Contains 16 giant files (1,012-2,412 lines each) requiring splitting

#### 5. **editor_canon_matrix** ✓ Documented
- **Current Purpose:** Tests editor product canonical surface
- **Test Content:** Assistant surface, build/release surface, content browser sync, diagnostics surface, domain suites, inspector sync, interaction routing, operations, outliner sync, overlay gizmo, services, shell panels, tool context modes, viewport navigation, workspace layout, world scene suites
- **Test Count:** 1,600 tests (16 giant files with 100 tests each)
- **Role Clarity:** CLEAR - Tests editor layer canonical API surface
- **Domain:** Editor
- **Routing:** Full (comprehensive editor verification)
- **Issues:** Contains 16 giant files (812-2,612 lines each) requiring splitting

#### 6. **engine_sdk_link_matrix** ✓ Documented
- **Current Purpose:** Tests engine-to-SDK linking and compatibility
- **Test Content:** Compatibility surface, observation batches, projection alignment, runtime export, startup bridge
- **Test Count:** 1,000 tests (5 giant files with 200 tests each)
- **Role Clarity:** CLEAR - Tests engine/SDK integration boundary
- **Domain:** Engine + SDK integration
- **Routing:** Full (integration verification)
- **Issues:** Contains 5 giant files (802 lines each) requiring splitting

#### 7. **sdk_tooling_link_matrix** ✓ Documented
- **Current Purpose:** Tests SDK-to-tooling linking and integration
- **Test Content:** Artifact alignment, assistant path, command lowering, preview build/release, workspace bridge
- **Test Count:** 1,000 tests (5 giant files with 200 tests each)
- **Role Clarity:** CLEAR - Tests SDK/tooling integration boundary
- **Domain:** SDK + Tooling integration
- **Routing:** Full (integration verification)
- **Issues:** Contains 5 giant files (802 lines each) requiring splitting

#### 8. **editor_state_matrix**
- **Current Purpose:** Tests editor state management architecture
- **Test Content:** Property-based tests for state container authority, ownership validation, cache invalidation, derived state rebuildability, domain service encapsulation, query layer delegation, state graph acyclicity, etc.
- **Test Count:** 31 test files (mix of property tests and integration tests)
- **Role Clarity:** CLEAR - Tests editor state management patterns and invariants
- **Domain:** Editor
- **Routing:** Full (state architecture verification)

#### 9. **editor_app_matrix**
- **Current Purpose:** Tests editor desktop application integration
- **Test Content:** Desktop app initialization, editor host, GPU viewport renderer, viewport camera controller
- **Test Count:** 4 test files
- **Role Clarity:** CLEAR - Tests editor application-level integration
- **Domain:** Editor
- **Routing:** Smoke (application integration)

#### 10. **tool_session_matrix**
- **Current Purpose:** Tests tooling session transaction semantics
- **Test Content:** Property tests for transaction all-or-nothing, audio/material reversibility, preconditions
- **Test Count:** 5 test files (property-based tests)
- **Role Clarity:** CLEAR - Tests tooling transaction and session invariants
- **Domain:** Tooling
- **Routing:** Full (transaction verification)

#### 11. **end_to_end_matrix**
- **Current Purpose:** Tests complete product workflows end-to-end
- **Test Content:** Open world/save world cycles, startup reference seed chain, terrain/sky roundtrip
- **Test Count:** 3 test files
- **Role Clarity:** CLEAR - Tests complete user workflows across all layers
- **Domain:** Cross-stack integration
- **Routing:** Smoke (critical path verification)

#### 12. **repo_hygiene**
- **Current Purpose:** Tests repository code quality and architectural rules
- **Test Content:** No allow attributes, no BOM in rust files, no debug modules in release, no empty dirs, file size limits, no inline tests in src, no placeholder stubs, ownership validation, state classification, layer separation, etc.
- **Test Count:** 50+ test files
- **Role Clarity:** CLEAR - Enforces repository hygiene and architectural rules
- **Domain:** Meta (repository quality)
- **Routing:** Verify (fast hygiene checks on every commit)

#### 13. **vertical_slice_quality_gates**
- **Current Purpose:** Tests vertical slice demo quality gates
- **Test Content:** Smoke tests for vertical slice boot, fire shot roundtrip, DTO extraction
- **Test Count:** 3 test files
- **Role Clarity:** CLEAR - Quality gates for vertical slice demo
- **Domain:** Integration
- **Routing:** Smoke (vertical slice validation)

#### 14. **proof_region_integration**
- **Current Purpose:** Tests proof/evidence focus and history through route chains
- **Test Content:** Tests that proof contour keeps evidence focus through multiple button routes
- **Test Count:** 1 test file
- **Role Clarity:** CLEAR - Tests proof/evidence integration across routes
- **Domain:** Integration
- **Routing:** Smoke (proof system integration)

#### 15. **focus_recovery_matrix**
- **Current Purpose:** Tests focus/retry/recovery contracts in canonical routes
- **Test Content:** Tests that canonical routes emit focus_target, retry_target, recovery_anchor
- **Test Count:** 1 test file
- **Role Clarity:** CLEAR - Tests focus recovery contracts
- **Domain:** Integration
- **Routing:** Full (focus contract verification)

#### 16. **forbidden_shortcuts**
- **Current Purpose:** Tests that forbidden architectural shortcuts are not present
- **Test Content:** Scans UI files for forbidden shortcuts (direct engine access, bypassing services, etc.)
- **Test Count:** 1 test file
- **Role Clarity:** CLEAR - Enforces architectural boundaries
- **Domain:** Meta (architecture enforcement)
- **Routing:** Verify (architectural rule enforcement)

#### 17. **route_schema_golden**
- **Current Purpose:** Tests route manifest and envelope JSON schema stability
- **Test Content:** Tests that manifest JSON shape is stable, envelope JSON shape is stable
- **Test Count:** 1 test file
- **Role Clarity:** CLEAR - Golden tests for route schema stability
- **Domain:** Integration
- **Routing:** Verify (schema stability check)

#### 18. **editor_command_matrix**
- **Current Purpose:** Tests editor command routing completeness and correctness
- **Test Content:** Tests that manifest has full phase4 superset, every route has canonical closure fields
- **Test Count:** 1 test file (lib.rs with tests)
- **Role Clarity:** CLEAR - Tests command routing completeness
- **Domain:** Editor
- **Routing:** Verify (command routing verification)

---

### Category B: Suites with Ambiguous or Overlapping Roles (8 suites)

These suites have unclear purposes or potentially overlap with other suites:

#### 19. **smoke**
- **Current Purpose:** UNCLEAR - Appears to be a general smoke test suite
- **Test Content:** Minimal smoke test only (placeholder)
- **Test Count:** 1 smoke test
- **Role Clarity:** AMBIGUOUS - Name suggests general smoke tests, but unclear what belongs here vs other smoke-level tests
- **Domain:** Unknown
- **Routing:** Smoke (presumably)
- **Issues:**
  - Generic "smoke" name violates naming law
  - Unclear what tests belong here vs `vertical_slice_quality_gates`, `end_to_end_matrix`, or other smoke-level tests
  - Currently empty/placeholder
- **Recommendation:** CONSOLIDATE or CLARIFY - Either define specific smoke test criteria or consolidate into other suites

#### 20. **editor_shell_matrix**
- **Current Purpose:** Tests editor shell/view management
- **Test Content:** Tests that view buttons are shell-owned, shell routes publish focus changes
- **Test Count:** 1 test file (lib.rs with tests)
- **Role Clarity:** AMBIGUOUS - Overlaps with `editor_canon_matrix` which also tests shell panels
- **Domain:** Editor
- **Routing:** Full
- **Issues:**
  - Potential overlap with `editor_canon_matrix` (which has `shell_panels.rs`)
  - Unclear boundary between "shell matrix" and "canon matrix"
- **Recommendation:** CLARIFY - Define clear boundary with `editor_canon_matrix` or consolidate

#### 21. **audio_authoring_matrix**
- **Current Purpose:** Tests audio authoring routes and ownership
- **Test Content:** Tests that audio routes are first-class and owner-backed, preview/inspect routes execute with focus
- **Test Count:** 1 test file (lib.rs with tests)
- **Role Clarity:** PARTIALLY CLEAR - Tests audio authoring, but unclear if this is canonical surface or integration
- **Domain:** Authoring
- **Routing:** Full
- **Issues:**
  - Currently minimal tests (placeholder suite)
  - Unclear relationship to `tooling_canon_matrix` (which tests authoring runtime)
- **Recommendation:** CLARIFY - Define as either canonical audio surface or audio authoring integration

#### 22. **material_authoring_matrix**
- **Current Purpose:** UNCLEAR - Presumably tests material authoring
- **Test Content:** Minimal smoke test only (placeholder)
- **Test Count:** 1 smoke test
- **Role Clarity:** AMBIGUOUS - Currently empty, unclear purpose
- **Domain:** Authoring
- **Routing:** Full (presumably)
- **Issues:**
  - Currently empty/placeholder
  - Unclear relationship to `tooling_canon_matrix`
- **Recommendation:** CLARIFY or REMOVE - Define specific role or remove if redundant

#### 23. **terrain_authoring_matrix**
- **Current Purpose:** UNCLEAR - Presumably tests terrain authoring
- **Test Content:** Minimal smoke test only (placeholder)
- **Test Count:** 1 smoke test
- **Role Clarity:** AMBIGUOUS - Currently empty, unclear purpose
- **Domain:** Authoring
- **Routing:** Full (presumably)
- **Issues:**
  - Currently empty/placeholder
  - Unclear relationship to `tooling_canon_matrix`
- **Recommendation:** CLARIFY or REMOVE - Define specific role or remove if redundant

#### 24. **environment_authoring_matrix**
- **Current Purpose:** UNCLEAR - Presumably tests environment authoring
- **Test Content:** Minimal smoke test only (placeholder)
- **Test Count:** 1 smoke test
- **Role Clarity:** AMBIGUOUS - Currently empty, unclear purpose
- **Domain:** Authoring
- **Routing:** Full (presumably)
- **Issues:**
  - Currently empty/placeholder
  - Unclear relationship to `tooling_canon_matrix`
- **Recommendation:** CLARIFY or REMOVE - Define specific role or remove if redundant

#### 25. **world_authoring_matrix**
- **Current Purpose:** UNCLEAR - Presumably tests world authoring
- **Test Content:** Minimal smoke test only (placeholder)
- **Test Count:** 1 smoke test
- **Role Clarity:** AMBIGUOUS - Currently empty, unclear purpose
- **Domain:** Authoring
- **Routing:** Full (presumably)
- **Issues:**
  - Currently empty/placeholder
  - Unclear relationship to `tooling_canon_matrix`
- **Recommendation:** CLARIFY or REMOVE - Define specific role or remove if redundant

#### 26. **build_release_matrix**
- **Current Purpose:** Tests build/release route contracts
- **Test Content:** Tests that build result contract carries release artifacts, focus, and recovery
- **Test Count:** 1 test file (lib.rs with tests)
- **Role Clarity:** AMBIGUOUS - Overlaps with `editor_canon_matrix` which has `build_release_surface.rs`
- **Domain:** Tooling/Editor
- **Routing:** Full
- **Issues:**
  - Potential overlap with `editor_canon_matrix` (which has `build_release_surface.rs` with 2,612 lines)
  - Unclear boundary between build matrix and editor canon matrix
- **Recommendation:** CLARIFY - Define clear boundary with `editor_canon_matrix` or consolidate

---

## Part 3: Overlapping Responsibilities Analysis

### Identified Overlaps

#### Overlap 1: Editor Shell Testing
- **Suites:** `editor_shell_matrix` vs `editor_canon_matrix`
- **Issue:** `editor_canon_matrix` has `shell_panels.rs` (1,112 lines), while `editor_shell_matrix` tests shell/view management
- **Recommendation:** Consolidate into `editor_canon_matrix` or clearly separate "shell infrastructure" from "shell panels UI"

#### Overlap 2: Build/Release Testing
- **Suites:** `build_release_matrix` vs `editor_canon_matrix`
- **Issue:** `editor_canon_matrix` has `build_release_surface.rs` (2,612 lines), while `build_release_matrix` tests build contracts
- **Recommendation:** Consolidate into `editor_canon_matrix` or clearly separate "build contracts" from "build UI surface"

#### Overlap 3: Authoring Domain Fragmentation
- **Suites:** `audio_authoring_matrix`, `material_authoring_matrix`, `terrain_authoring_matrix`, `environment_authoring_matrix`, `world_authoring_matrix` vs `tooling_canon_matrix`
- **Issue:** 5 separate authoring suites (mostly empty) vs `tooling_canon_matrix` which tests "tooling authoring runtime"
- **Recommendation:** CONSOLIDATE - Either:
  1. Consolidate all authoring tests into `tooling_canon_matrix`
  2. OR clearly define authoring matrices as "domain-specific authoring integration" separate from canonical tooling surface

#### Overlap 4: Smoke Test Ambiguity
- **Suites:** `smoke` vs `vertical_slice_quality_gates` vs `end_to_end_matrix`
- **Issue:** Three suites with smoke-level tests, unclear boundaries
- **Recommendation:** CLARIFY - Define clear criteria:
  - `smoke`: Fast critical path tests (<5 min)
  - `vertical_slice_quality_gates`: Vertical slice demo quality gates
  - `end_to_end_matrix`: Complete product workflow tests

---

## Part 4: Summary Statistics

### Suite Count by Clarity
- **Clear single roles:** 18 suites
- **Ambiguous/overlapping roles:** 8 suites
- **Total:** 26 suites

### Suite Count by Status
- **Documented in SUITE_ROLE_MAP.md:** 7 suites
- **Needs documentation:** 19 suites
- **Empty/placeholder:** 9 suites (smoke, material_authoring_matrix, terrain_authoring_matrix, environment_authoring_matrix, world_authoring_matrix, engine_perf_harness, editor_shell_matrix, build_release_matrix, audio_authoring_matrix)

### Suite Count by Domain
- **Engine:** 2 suites (engine_canon_matrix, engine_perf_harness)
- **SDK:** 1 suite (sdk_canon_matrix)
- **Tooling:** 2 suites (tooling_canon_matrix, tool_session_matrix)
- **Editor:** 5 suites (editor_canon_matrix, editor_state_matrix, editor_app_matrix, editor_command_matrix, editor_shell_matrix)
- **Integration:** 6 suites (engine_sdk_link_matrix, sdk_tooling_link_matrix, end_to_end_matrix, vertical_slice_quality_gates, proof_region_integration, focus_recovery_matrix)
- **Authoring:** 5 suites (audio_authoring_matrix, material_authoring_matrix, terrain_authoring_matrix, environment_authoring_matrix, world_authoring_matrix)
- **Build/Release:** 1 suite (build_release_matrix)
- **Meta:** 3 suites (repo_hygiene, forbidden_shortcuts, route_schema_golden)
- **Smoke:** 1 suite (smoke)

### Giant Files by Suite
- **sdk_canon_matrix:** 7 giant files (2,210-10,210 lines)
- **tooling_canon_matrix:** 16 giant files (1,012-2,412 lines)
- **editor_canon_matrix:** 16 giant files (812-2,612 lines)
- **engine_sdk_link_matrix:** 5 giant files (802 lines each)
- **sdk_tooling_link_matrix:** 5 giant files (802 lines each)
- **engine_canon_matrix:** 6 giant files (814-1,222 lines)
- **Total:** 56 giant files across 6 suites

---

## Part 5: Recommendations for Suite Law Document

### High Priority Actions

1. **Document roles for 19 undocumented suites** in SUITE_LAW.md
2. **Clarify or consolidate 8 ambiguous suites:**
   - Consolidate authoring matrices OR define clear boundaries
   - Clarify editor_shell_matrix vs editor_canon_matrix boundary
   - Clarify build_release_matrix vs editor_canon_matrix boundary
   - Define smoke suite criteria OR consolidate into other suites
3. **Prohibit new "temporary" or "misc" suites** in SUITE_LAW.md
4. **Define inclusion/exclusion criteria** for all suites
5. **Document default routing** for each suite (verify/smoke/full)

### Suite Consolidation Candidates

#### Option A: Consolidate Authoring Matrices
- **Consolidate:** audio_authoring_matrix, material_authoring_matrix, terrain_authoring_matrix, environment_authoring_matrix, world_authoring_matrix
- **Into:** tooling_canon_matrix (as domain-specific test families)
- **Rationale:** All test authoring runtime, currently mostly empty, reduces suite count from 26 to 21

#### Option B: Consolidate Shell/Build Matrices
- **Consolidate:** editor_shell_matrix, build_release_matrix
- **Into:** editor_canon_matrix (as focused test families)
- **Rationale:** Already have overlapping test content, reduces suite count from 26 to 24 (or 19 if also consolidating authoring)

#### Option C: Remove/Consolidate Smoke Suite
- **Remove:** smoke suite (currently empty)
- **Rationale:** Smoke-level tests already exist in vertical_slice_quality_gates and end_to_end_matrix

### Proposed Final Suite Count
- **Current:** 26 suites
- **After consolidation (Option A + B + C):** 18 suites
- **Reduction:** 8 suites (31% reduction)

---

## Part 6: Next Steps for Package 2

1. **Create SUITE_LAW.md** with:
   - Single role statement for each suite
   - Inclusion criteria (what tests belong)
   - Exclusion criteria (what tests do NOT belong)
   - Concrete examples
   - Default routing category
   - Consolidation recommendations

2. **Review with stakeholders:**
   - Validate suite roles
   - Approve consolidation recommendations
   - Clarify ambiguous boundaries

3. **Proceed to Task 5:** Define suite laws based on this analysis

---

## Appendix: Suite-by-Suite Test File Counts

| Suite | Test Files | Giant Files (800+) | Total Tests (est.) |
|-------|------------|--------------------|--------------------|
| engine_canon_matrix | 6 | 6 | 255 |
| engine_perf_harness | 1 | 0 | 1 |
| sdk_canon_matrix | 7 | 7 | 1,400 |
| tooling_canon_matrix | 16 | 16 | 1,600 |
| editor_canon_matrix | 16 | 16 | 1,600 |
| engine_sdk_link_matrix | 5 | 5 | 1,000 |
| sdk_tooling_link_matrix | 5 | 5 | 1,000 |
| editor_state_matrix | 31 | 0 | 150+ |
| editor_app_matrix | 4 | 0 | 20+ |
| tool_session_matrix | 5 | 0 | 25+ |
| end_to_end_matrix | 3 | 0 | 15+ |
| repo_hygiene | 50+ | 0 | 200+ |
| vertical_slice_quality_gates | 1 | 0 | 3 |
| proof_region_integration | 1 | 0 | 1 |
| focus_recovery_matrix | 1 | 0 | 1 |
| forbidden_shortcuts | 1 | 0 | 1 |
| route_schema_golden | 1 | 0 | 2 |
| editor_command_matrix | 1 | 0 | 2 |
| smoke | 1 | 0 | 1 |
| editor_shell_matrix | 1 | 0 | 2 |
| audio_authoring_matrix | 1 | 0 | 2 |
| material_authoring_matrix | 1 | 0 | 1 |
| terrain_authoring_matrix | 1 | 0 | 1 |
| environment_authoring_matrix | 1 | 0 | 1 |
| world_authoring_matrix | 1 | 0 | 1 |
| build_release_matrix | 1 | 0 | 1 |
| **TOTAL** | **155+** | **56** | **7,285+** |


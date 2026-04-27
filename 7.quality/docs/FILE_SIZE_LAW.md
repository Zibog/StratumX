# File Size Law

## Purpose

This document establishes and enforces file size limits for all files in the 7.quality test contour. The goal is to maintain readability, maintainability, and focused purpose for both test files and support code files.

Giant files (800+ lines) are difficult to navigate, understand, and maintain. They typically indicate multiple responsibilities mixed together, making it hard to locate specific tests or understand the file's purpose. By enforcing size limits, we ensure that files remain focused, understandable, and easy to work with.

## Law Statement

**All files in the 7.quality contour MUST comply with the file size thresholds defined in this document. Files exceeding mandatory split thresholds MUST be split before merge.**

## Test File Thresholds

Test files are located in `7.quality/suites/*/tests/` and contain test functions.

| Line Count | Classification | Action Required | Enforcement |
|------------|----------------|-----------------|-------------|
| 0-300      | Normal         | None - acceptable size | None |
| 301-500    | Suspicious     | Review recommended - consider splitting if file has multiple responsibilities | Manual review |
| 501-800    | Requires Split | Create split plan - file should be split soon | Requires justification |
| 800-1200   | Mandatory Split | MUST be split immediately | Blocks merge |
| 1200+      | Forbidden      | MUST be split before any other work | Hard block |

### Rationale for Test File Thresholds

- **0-300 lines (Normal)**: A focused test file testing a single concept or small family of related tests. Easy to read in one sitting, clear purpose, maintainable.

- **301-500 lines (Suspicious)**: Starting to get large. May indicate multiple test families or responsibilities. Review to determine if splitting would improve clarity.

- **501-800 lines (Requires Split)**: Too large for comfortable navigation. Likely contains multiple test families that should be separated. Split plan required.

- **800-1200 lines (Mandatory Split)**: Definitely too large. Multiple responsibilities mixed together. Must be split immediately to maintain contour health.

- **1200+ lines (Forbidden)**: Unacceptably large. Indicates severe organizational problems. Must be split before any other work proceeds.

### Examples of Test File Sizes

**Good Example (Normal - 250 lines)**:
```
7.quality/suites/engine_canon_matrix/tests/region_allocation_tests.rs
- Tests region allocation behavior
- 15 focused tests
- Clear single responsibility
- Easy to navigate and understand
```

**Suspicious Example (450 lines)**:
```
7.quality/suites/sdk_canon_matrix/tests/query_tests.rs
- Tests both spatial queries AND entity queries
- 30 tests mixing two concepts
- Should consider splitting into:
  - spatial_query_tests.rs
  - entity_query_tests.rs
```

**Requires Split (650 lines)**:
```
7.quality/suites/engine_canon_matrix/tests/storage_tests.rs
- Tests storage allocation, mutation, AND access
- 45 tests across three distinct domains
- MUST split into:
  - storage_allocation_tests.rs
  - storage_mutation_tests.rs
  - storage_access_tests.rs
```

**Mandatory Split (1000 lines)**:
```
7.quality/suites/engine_canon_matrix/tests/lookup_correctness.rs
- 200 numbered tests (lookup_correctness_0 through lookup_correctness_199)
- Impossible to navigate
- MUST split immediately by test number ranges or parameterize
```

## Support File Thresholds

Support files are located in `7.quality/support/*/src/` and contain builders, fixtures, assertions, and case generators used by tests.

| Line Count | Classification | Action Required | Enforcement |
|------------|----------------|-----------------|-------------|
| 0-250      | Normal         | None - acceptable size | None |
| 251-400    | Requires Split | Create split plan - file should be split soon | Requires justification |
| 400+       | Mandatory Split | MUST be split immediately | Blocks merge |

### Rationale for Support File Thresholds

Support files have stricter limits than test files because:

1. **Reusability**: Support code is used across multiple test files, so clarity is critical
2. **Focused Purpose**: Each support module should have a single clear purpose (builders, fixtures, assertions, or case generators)
3. **Discoverability**: Developers need to quickly find the right helper function

- **0-250 lines (Normal)**: A focused support module with a single clear purpose. Contains related helpers that work together.

- **251-400 lines (Requires Split)**: Getting large. May indicate multiple categories of support code mixed together. Should be split by subcategory.

- **400+ lines (Mandatory Split)**: Too large. Definitely contains multiple responsibilities. Must be split immediately.

### Examples of Support File Sizes

**Good Example (Normal - 180 lines)**:
```
7.quality/support/stratumx_test_support/src/builders.rs
- Contains world builders, region builders, entity builders
- All related to constructing test objects
- Clear single purpose
- Easy to find the right builder
```

**Requires Split (320 lines)**:
```
7.quality/support/stratumx_test_support/src/fixtures.rs
- Contains fixtures for worlds, regions, entities, AND queries
- Multiple fixture categories mixed together
- Should split into:
  - world_fixtures.rs
  - region_fixtures.rs
  - entity_fixtures.rs
  - query_fixtures.rs
```

**Mandatory Split (500 lines)**:
```
7.quality/support/stratumx_test_support/src/helpers.rs
- Generic name indicates unclear purpose
- Contains builders, fixtures, AND assertions mixed together
- MUST split by support code category immediately
```

## Enforcement Strategy

### Automated CI Check

The file size law is enforced through an automated check in the `repo_hygiene` test suite.

**Location**: `7.quality/suites/repo_hygiene/tests/file_size_check.rs`

**Behavior**:
- Scans all files in `7.quality/suites/*/tests/` (test files)
- Scans all files in `7.quality/support/*/src/` (support files)
- Counts lines excluding blank lines and comments
- Classifies each file by size category
- Reports violations with file paths and line counts
- **FAILS** if any file exceeds mandatory split threshold (800 for tests, 400 for support)

**CI Integration**:
- Runs in verify mode (every commit)
- Blocks merge if violations exist
- Provides clear error messages with file paths and required actions

### Manual Review Process

For files in the "Suspicious" or "Requires Split" categories:

1. **Identify the file** in the hygiene check output
2. **Analyze the file** to understand its responsibilities
3. **Create a split plan** documenting:
   - Current file structure
   - Proposed split strategy (by family, by domain, by number range, parameterize)
   - Target files with estimated line counts
4. **Execute the split** following the plan
5. **Verify** all tests pass and split files comply with size law

## Line Counting Rules

**Lines that COUNT toward the limit**:
- Code lines (function definitions, statements, expressions)
- Documentation comments (/// and //!)
- Inline comments (// and /* */)
- Blank lines within functions or structs

**Lines that DO NOT COUNT**:
- Blank lines between top-level items
- File-level module documentation (first comment block)
- Import statements (use declarations)

**Rationale**: We count all lines including comments because even well-commented code becomes hard to navigate when files are too large. The goal is to keep files at a size where you can read and understand them in one sitting.

## Split Strategies

When a file exceeds the split threshold, choose the appropriate strategy:

### Strategy 1: Split by Test Family

**When to use**: File contains multiple distinct test families (groups of related tests)

**Example**:
```
Original: storage_tests.rs (800 lines)
- allocation_tests (300 lines)
- mutation_tests (250 lines)
- access_tests (250 lines)

Split into:
- storage_allocation_tests.rs (300 lines)
- storage_mutation_tests.rs (250 lines)
- storage_access_tests.rs (250 lines)
```

### Strategy 2: Split by Number Range

**When to use**: File contains numbered tests (test_0, test_1, ..., test_N)

**Example**:
```
Original: lookup_correctness.rs (10,000 lines, 200 tests)

Split into:
- lookup_correctness_0_49.rs (2,500 lines, tests 0-49)
- lookup_correctness_50_99.rs (2,500 lines, tests 50-99)
- lookup_correctness_100_149.rs (2,500 lines, tests 100-149)
- lookup_correctness_150_199.rs (2,500 lines, tests 150-199)
```

### Strategy 3: Parameterize

**When to use**: File contains many similar tests that differ only in input data

**Example**:
```
Original: boundary_checks.rs (1,200 lines, 60 similar tests)

Convert to:
- boundary_checks.rs (300 lines)
  - Single parameterized test
  - Case generator in support module
  - 60 test cases as data
```

### Strategy 4: Split by Domain

**When to use**: File contains tests for multiple semantic domains

**Example**:
```
Original: world_tests.rs (900 lines)
- Tests for world creation (300 lines)
- Tests for world streaming (300 lines)
- Tests for world queries (300 lines)

Split into:
- world_creation_tests.rs (300 lines)
- world_streaming_tests.rs (300 lines)
- world_query_tests.rs (300 lines)
```

## Exceptions and Waivers

**There are NO exceptions to the mandatory split threshold.**

Files exceeding 800 lines (tests) or 400 lines (support) MUST be split. No waivers will be granted.

**Rationale**: Allowing exceptions undermines the law and leads to gradual decay back to the "giant sprawling city" state. The thresholds are generous enough that any file exceeding them genuinely needs splitting.

## Maintenance

### Adding New Tests

When adding new tests to an existing file:

1. **Check current file size** before adding tests
2. **Estimate new size** after adding tests
3. **If new size exceeds 300 lines**: Consider creating a new file instead
4. **If new size exceeds 500 lines**: MUST create a new file or split existing file first

### Monitoring File Growth

The repo hygiene check runs on every commit, providing continuous monitoring of file sizes. Review the output regularly to catch files approaching split thresholds before they become problems.

### Periodic Audits

Conduct quarterly audits of files in the "Suspicious" category (301-500 lines for tests, 251-400 for support) to determine if splitting would improve maintainability.

## Benefits of File Size Law

1. **Readability**: Files are small enough to read and understand in one sitting
2. **Focused Purpose**: Each file has a clear, single responsibility
3. **Easy Navigation**: Developers can quickly find relevant tests
4. **Better Organization**: Forces thoughtful organization of test code
5. **Prevents Decay**: Stops gradual accumulation of "just one more test" leading to giant files
6. **Easier Reviews**: Code reviews are more effective on focused, small files
7. **Reduced Merge Conflicts**: Smaller files mean less chance of conflicts

## Related Documents

- [SUITE_LAW.md](./SUITE_LAW.md) - Defines the single role for each test suite
- [NAMING_LAW.md](./NAMING_LAW.md) - Defines naming standards for test files
- [INVENTORY.md](./INVENTORY.md) - Complete catalog of current test files with sizes
- [NEW_TEST_LAW.md](./NEW_TEST_LAW.md) - Process for creating new tests
- [SPLIT_PLANS.md](./SPLIT_PLANS.md) - Detailed split plans for all giant files (Task 9 output)

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-04-10 | 1.0 | Initial file size law document |


## Current Violations Report

**Last Updated**: 2024-01-10 (Task 8.2 execution)

This section documents all current violations of the File Size Law as detected by the automated hygiene check in `7.quality/suites/repo_hygiene/tests/quality_file_size_check.rs`.

### Summary

- **Total violations**: 142 files
- **Mandatory splits** (blocks merge): 57 files (56 test files, 1 support file)
- **Requires split plan**: 35 test files
- **Suspicious** (review recommended): 50 test files

### Test Files - Mandatory Split (800+ lines) - BLOCKS MERGE

These 56 test files MUST be split immediately. They exceed the 800-line mandatory split threshold.

**Priority Giant Files** (>4000 lines):
1. `suites/sdk_canon_matrix/tests/lookup_correctness.rs` - **10,407 lines** - CRITICAL
2. `suites/sdk_canon_matrix/tests/field_invariants.rs` - **8,007 lines** - CRITICAL
3. `suites/sdk_canon_matrix/tests/allocation_posture.rs` - **7,807 lines** - CRITICAL
4. `suites/sdk_canon_matrix/tests/pressure_bounds.rs` - **6,807 lines** - CRITICAL
5. `suites/sdk_canon_matrix/tests/boundary_legality.rs` - **4,807 lines** - CRITICAL
6. `suites/sdk_canon_matrix/tests/snapshot_swaps.rs` - **4,407 lines** - CRITICAL

**Large Files** (2000-4000 lines):
7. `suites/editor_canon_matrix/tests/build_release_surface.rs` - 2,709 lines
8. `suites/tooling_canon_matrix/tests/snapshot_immutability.rs` - 2,509 lines
9. `suites/tooling_canon_matrix/tests/validation_legality.rs` - 2,509 lines
10. `suites/tooling_canon_matrix/tests/assistant_lowering.rs` - 2,409 lines
11. `suites/sdk_canon_matrix/tests/opacity_preservation.rs` - 2,407 lines
12. `suites/tooling_canon_matrix/tests/command_schema.rs` - 2,309 lines
13. `suites/tooling_canon_matrix/tests/transaction_determinism.rs` - 2,309 lines
14. `suites/tooling_canon_matrix/tests/authority_isolation.rs` - 2,209 lines
15. `suites/tooling_canon_matrix/tests/index_rebuild.rs` - 2,009 lines

**Medium-Large Files** (1000-2000 lines):
16. `suites/tooling_canon_matrix/tests/apply_revert_chain.rs` - 1,909 lines
17. `suites/editor_canon_matrix/tests/diagnostics_surface.rs` - 1,709 lines
18. `suites/editor_canon_matrix/tests/services.rs` - 1,709 lines
19. `suites/editor_canon_matrix/tests/tool_context_modes.rs` - 1,709 lines
20. `suites/editor_canon_matrix/tests/operations.rs` - 1,609 lines
21. `suites/editor_canon_matrix/tests/assistant_surface.rs` - 1,509 lines
22. `suites/tooling_canon_matrix/tests/artifact_manifest.rs` - 1,409 lines
23. `suites/tooling_canon_matrix/tests/cache_eviction.rs` - 1,409 lines
24. `suites/tooling_canon_matrix/tests/stream_boundedness.rs` - 1,409 lines
25. `suites/editor_canon_matrix/tests/content_browser_sync.rs` - 1,309 lines
26. `suites/editor_canon_matrix/tests/interaction_routing.rs` - 1,309 lines
27. `suites/editor_canon_matrix/tests/workspace_layout.rs` - 1,309 lines
28. `suites/engine_canon_matrix/tests/world_spatial_matrix.rs` - 1,221 lines
29. `suites/editor_canon_matrix/tests/domain_suites.rs` - 1,209 lines
30. `suites/editor_canon_matrix/tests/shell_panels.rs` - 1,209 lines
31. `suites/tooling_canon_matrix/tests/preview_non_authority.rs` - 1,209 lines
32. `suites/tooling_canon_matrix/tests/release_legality.rs` - 1,209 lines
33. `suites/tooling_canon_matrix/tests/workspace_non_truth.rs` - 1,209 lines
34. `suites/editor_canon_matrix/tests/viewport_navigation.rs` - 1,109 lines
35. `suites/editor_canon_matrix/tests/world_scene_suites.rs` - 1,109 lines
36. `suites/tooling_canon_matrix/tests/build_reproducibility.rs` - 1,109 lines
37. `suites/tooling_canon_matrix/tests/derived_non_authority.rs` - 1,109 lines
38. `suites/editor_canon_matrix/tests/inspector_sync.rs` - 1,009 lines
39. `suites/engine_sdk_link_matrix/tests/compatibility_surface.rs` - 1,000 lines
40. `suites/engine_sdk_link_matrix/tests/observation_batches.rs` - 1,000 lines
41. `suites/engine_sdk_link_matrix/tests/projection_alignment.rs` - 1,000 lines
42. `suites/engine_sdk_link_matrix/tests/runtime_export.rs` - 1,000 lines
43. `suites/engine_sdk_link_matrix/tests/startup_bridge.rs` - 1,000 lines
44. `suites/sdk_tooling_link_matrix/tests/artifact_alignment.rs` - 1,000 lines
45. `suites/sdk_tooling_link_matrix/tests/assistant_path.rs` - 1,000 lines
46. `suites/sdk_tooling_link_matrix/tests/command_lowering.rs` - 1,000 lines
47. `suites/sdk_tooling_link_matrix/tests/preview_build_release.rs` - 1,000 lines
48. `suites/sdk_tooling_link_matrix/tests/workspace_bridge.rs` - 1,000 lines

**Just Over Threshold** (800-1000 lines):
49. `suites/engine_canon_matrix/tests/generation_matrix.rs` - 961 lines
50. `suites/engine_canon_matrix/tests/runtime_matrix.rs` - 921 lines
51. `suites/engine_canon_matrix/tests/storage_layout_matrix.rs` - 921 lines
52. `suites/editor_canon_matrix/tests/outliner_sync.rs` - 909 lines
53. `suites/editor_canon_matrix/tests/overlay_gizmo.rs` - 909 lines
54. `suites/engine_canon_matrix/tests/kinetics_matrix.rs` - 841 lines
55. `suites/engine_canon_matrix/tests/storage_access_matrix.rs` - 821 lines
56. `suites/engine_canon_matrix/tests/startup_matrix.rs` - 814 lines

### Support Files - Mandatory Split (400+ lines) - BLOCKS MERGE

This 1 support file MUST be split immediately:

1. `support/stratumx_test_support/src/tooling_runtime.rs` - **424 lines**

### Test Files - Requires Split Plan (501-800 lines)

These 32 test files require split plans. They should be split soon to prevent further growth:

1. `suites/engine_canon_matrix/tests/agents_matrix.rs` - 761 lines
2. `suites/engine_canon_matrix/tests/field_matrix.rs` - 761 lines
3. `suites/engine_canon_matrix/tests/net_transport_matrix.rs` - 751 lines
4. `suites/engine_canon_matrix/tests/imaging_matrix.rs` - 726 lines
5. `suites/engine_canon_matrix/tests/runtime_headless_matrix.rs` - 726 lines
6. `suites/repo_hygiene/tests/test_discovery_scanner.rs` - 721 lines
7. `suites/engine_canon_matrix/tests/ecs_query_matrix.rs` - 711 lines
8. `suites/editor_canon_matrix/tests/destruction_proof_pack.rs` - 706 lines
9. `suites/engine_canon_matrix/tests/acoustics_matrix.rs` - 701 lines
10. `suites/editor_canon_matrix/tests/sky_weather_runtime_proof_pack.rs` - 694 lines
11. `suites/engine_canon_matrix/tests/memory_control_matrix.rs` - 691 lines
12. `suites/repo_hygiene/tests/test_models.rs` - 679 lines
13. `suites/editor_canon_matrix/tests/nav_door_inventory_persistence_proof_pack.rs` - 673 lines
14. `suites/engine_canon_matrix/tests/handle_matrix.rs` - 631 lines
15. `suites/editor_state_matrix/tests/property_05_state_container_authority.rs` - 629 lines
16. `suites/editor_state_matrix/tests/state_query_immutability_tests.rs` - 629 lines
17. `suites/repo_hygiene/tests/test_hygiene_checker.rs` - 603 lines
18. `suites/editor_canon_matrix/tests/brutal_proof_scene_regression_suite.rs` - 585 lines
19. `suites/tool_session_matrix/tests/property_transaction_tests.rs` - 584 lines
20. `suites/editor_canon_matrix/tests/honest_full_proof_pack_integration.rs` - 572 lines
21. `suites/editor_canon_matrix/tests/material_world_proof_pack.rs` - 571 lines
22. `suites/tool_session_matrix/tests/property_precondition_tests.rs` - 570 lines
23. `suites/engine_canon_matrix/tests/inference_matrix.rs` - 561 lines
24. `suites/repo_hygiene/tests/test_code_cleaner.rs` - 554 lines
25. `suites/engine_canon_matrix/tests/content_matrix.rs` - 541 lines
26. `suites/editor_canon_matrix/tests/regression_farm_proof_region.rs` - 523 lines
27. `suites/editor_state_matrix/tests/property_domain_service_encapsulation.rs` - 523 lines
28. `suites/engine_canon_matrix/tests/net_latency_matrix.rs` - 511 lines
29. `suites/engine_canon_matrix/tests/world_matrix.rs` - 511 lines
30. `suites/repo_hygiene/tests/test_discovery_scanner_properties.rs` - 511 lines
31. `suites/editor_state_matrix/tests/property_domain_service_dependency_constraints.rs` - 507 lines
32. `suites/engine_canon_matrix/tests/core_matrix.rs` - 501 lines

### Support Files - Requires Split Plan (251-400 lines)

These 3 support files require split plans:

1. `support/stratumx_test_support/src/bridge_runtime.rs` - 389 lines
2. `support/stratumx_test_support/src/bridge_types.rs` - 346 lines
3. `support/stratumx_repo_hygiene_support/src/lib.rs` - 306 lines

### Test Files - Suspicious (301-500 lines) - Review Recommended

These 50 test files are in the suspicious range. Review recommended to determine if splitting would improve clarity:

1. `suites/repo_hygiene/tests/unit_directory_structure.rs` - 485 lines
2. `suites/engine_canon_matrix/tests/storage_mutation_matrix.rs` - 481 lines
3. `suites/engine_canon_matrix/tests/world_region_matrix.rs` - 481 lines
4. `suites/repo_hygiene/tests/test_waiver_registry.rs` - 466 lines
5. `suites/tool_session_matrix/tests/property_audio_reversibility_tests.rs` - 466 lines
6. `suites/repo_hygiene/tests/test_migrator.rs` - 454 lines
7. `suites/editor_state_matrix/tests/workspace_persistence_tests.rs` - 451 lines
8. `suites/repo_hygiene/tests/property_cache_anti_pattern_detection.rs` - 451 lines
9. `suites/repo_hygiene/tests/test_hygiene_checker_properties.rs` - 451 lines
10. `suites/repo_hygiene/tests/test_migration_logger_properties.rs` - 442 lines
11. `suites/engine_canon_matrix/tests/residency_control_matrix.rs` - 426 lines
12. `suites/editor_state_matrix/tests/query_immutability.rs` - 414 lines
13. `suites/engine_canon_matrix/tests/ecs_matrix.rs` - 411 lines
14. `suites/editor_canon_matrix/tests/brutal_proof_region.rs` - 410 lines
15. `suites/repo_hygiene/tests/property_domain_services_own_truth.rs` - 405 lines
16. `suites/repo_hygiene/tests/property_duplicate_state.rs` - 405 lines
17. `suites/editor_canon_matrix/tests/runtime_backed_authoring_integration.rs` - 402 lines
18. `suites/engine_canon_matrix/tests/stream_control_matrix.rs` - 401 lines
19. `suites/repo_hygiene/tests/property_ownership_validation_detects_violations.rs` - 400 lines
20. `suites/repo_hygiene/tests/test_migrator_properties.rs` - 396 lines
21. `suites/repo_hygiene/tests/property_layer_separation.rs` - 393 lines
22. `suites/editor_canon_matrix/tests/freeze_gating_integration.rs` - 387 lines
23. `suites/end_to_end_matrix/tests/open_world_save_world.rs` - 387 lines
24. `suites/repo_hygiene/tests/integration_state_flow.rs` - 382 lines
25. `suites/repo_hygiene/tests/test_quality_restructurer.rs` - 368 lines
26. `suites/tool_session_matrix/tests/property_material_reversibility_tests.rs` - 365 lines
27. `suites/editor_state_matrix/tests/registry_identity_tests.rs` - 363 lines
28. `suites/engine_canon_matrix/tests/identity_matrix.rs` - 361 lines
29. `suites/engine_canon_matrix/tests/ecs_registry_matrix.rs` - 351 lines
30. `suites/engine_canon_matrix/tests/material_matrix.rs` - 351 lines
31. `suites/engine_canon_matrix/tests/runtime_realtime_matrix.rs` - 351 lines
32. `suites/repo_hygiene/tests/integration/test_rollback_workflow.rs` - 351 lines
33. `suites/repo_hygiene/tests/test_migration_logger.rs` - 351 lines
34. `suites/editor_canon_matrix/tests/integration_save_load.rs` - 350 lines
35. `suites/repo_hygiene/tests/quality_file_size_check.rs` - 344 lines
36. `suites/editor_state_matrix/tests/editor_host_tests.rs` - 343 lines
37. `suites/editor_canon_matrix/tests/honest_material_world_runtime_path.rs` - 341 lines
38. `suites/editor_state_matrix/tests/integration_error_recovery.rs` - 333 lines
39. `suites/repo_hygiene/tests/integration/test_waiver_workflow.rs` - 333 lines
40. `suites/editor_canon_matrix/tests/sky_material_integration_proof_pack.rs` - 332 lines
41. `suites/editor_canon_matrix/tests/honest_scene_runtime_path.rs` - 327 lines
42. `suites/editor_state_matrix/tests/property_state_graph_dependency_accuracy.rs` - 322 lines
43. `suites/engine_canon_matrix/tests/transfer_control_matrix.rs` - 321 lines
44. `suites/editor_state_matrix/tests/ownership_validation_tests.rs` - 318 lines
45. `suites/editor_state_matrix/tests/property_editor_host_minimal_business_logic.rs` - 313 lines
46. `suites/editor_state_matrix/tests/integration_service_coordination.rs` - 312 lines
47. `suites/engine_canon_matrix/tests/net_sync_matrix.rs` - 311 lines
48. `suites/editor_state_matrix/tests/property_query_layer_delegation.rs` - 305 lines
49. `suites/repo_hygiene/tests/integration/test_hygiene_workflow.rs` - 305 lines
50. `suites/editor_canon_matrix/tests/streaming_residency_memory_law_zone_b.rs` - 301 lines

### Prioritization for Splitting

Based on the violation report, the following prioritization is recommended:

**Phase 1 - Critical Giant Files** (Package 5 priority):
1. `lookup_correctness.rs` (10,407 lines) - Highest priority
2. `field_invariants.rs` (8,007 lines)
3. `allocation_posture.rs` (7,807 lines)
4. `pressure_bounds.rs` (6,807 lines)
5. `boundary_legality.rs` (4,807 lines)

**Phase 2 - Large Files** (2000+ lines):
- 10 files ranging from 2,009 to 2,709 lines
- Should be split after Phase 1 completes

**Phase 3 - Medium Files** (1000-2000 lines):
- 33 files ranging from 1,000 to 1,909 lines
- Split as part of ongoing maintenance

**Phase 4 - Just Over Threshold** (800-1000 lines):
- 8 files ranging from 814 to 961 lines
- Split to prevent further growth

**Phase 5 - Support Files**:
- 1 mandatory split (424 lines)
- 3 require split plans (306-389 lines)

**Phase 6 - Requires Split Plan** (501-800 lines):
- 32 test files
- Create split plans and execute systematically

**Phase 7 - Suspicious Files** (301-500 lines):
- 50 test files
- Review and split where multiple responsibilities identified

### Automated Enforcement

The file size law is enforced through the automated hygiene check:

**Test Location**: `7.quality/suites/repo_hygiene/tests/quality_file_size_check.rs`

**Execution**: Runs as part of the repo_hygiene test suite

**Behavior**:
- Scans all test files in `7.quality/suites/*/tests/`
- Scans all support files in `7.quality/support/*/src/`
- Counts lines according to FILE_SIZE_LAW.md rules
- Classifies files by violation category
- Reports all violations with file paths and line counts
- **FAILS** if any file exceeds mandatory split threshold (800 for tests, 400 for support)

**Running the Check**:
```bash
cd 7.quality/suites/repo_hygiene
cargo test quality_file_size_check -- --nocapture
```

### Next Steps

1. **Immediate Action**: Split the 6 critical giant files (4,000+ lines) in Phase 1
2. **Package 5 Execution**: Follow the detailed split plans in [SPLIT_PLANS.md](./SPLIT_PLANS.md) (created in Task 9)
3. **Continuous Monitoring**: Run the hygiene check regularly to prevent new violations
4. **Systematic Reduction**: Work through Phases 2-7 to eliminate all violations
5. **Prevention**: Enforce the law for all new test additions (see NEW_TEST_LAW.md)

**See [SPLIT_PLANS.md](./SPLIT_PLANS.md) for detailed split plans for all 65 violating files, including:**
- Specific split strategies for each file
- Target file names and estimated line counts
- Step-by-step implementation instructions
- Effort estimates and prioritization
- Completion tracking checklist


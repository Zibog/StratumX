# Suite Law Document

**Purpose:** Define the single clear role for each test suite in 7.quality/suites/

**Generated for:** Quality Contour Surgery Phase 3 - Package 2: Suite Law Enforcement  
**Date:** Phase 3 Implementation

---

## Overview

This document establishes the **Suite Law**: each test suite SHALL have exactly one clear, documented role. This law prevents overlapping responsibilities, ambiguous test placement, and the creation of "temporary" or "miscellaneous" suites.

### Core Principles

1. **Single Role**: Each suite tests exactly one domain, layer, or integration boundary
2. **Clear Boundaries**: Inclusion and exclusion criteria are explicit and unambiguous
3. **No Overlap**: No two suites test the same responsibility
4. **No Generic Suites**: Suites named "misc", "temp", "tests", or similar are prohibited
5. **Documented Examples**: Each suite provides concrete examples of appropriate tests

### Suite Categories

Suites are organized into the following categories:

- **Canonical Surface Suites**: Test canonical API surfaces for each layer (engine, SDK, tooling, editor)
- **Link/Integration Suites**: Test integration boundaries between layers
- **State/Architecture Suites**: Test architectural patterns and state management
- **Meta/Hygiene Suites**: Test repository quality and architectural rules
- **Workflow Suites**: Test complete product workflows and quality gates
- **Domain-Specific Suites**: Test specific domain functionality (authoring, commands, etc.)

---

## Canonical Surface Suites

### 1. engine_canon_matrix

**Role:** Tests the canonical engine surface using real `2.engine/*` crates

**Inclusion Criteria:**
- Tests that directly exercise engine layer APIs
- Tests using real engine crates (not through SDK bridge)
- Tests for engine startup, runtime, storage, world spatial, kinetics, generation
- Tests verifying engine canonical contracts and invariants

**Exclusion Criteria:**
- Tests that go through SDK bridge (use `engine_sdk_link_matrix`)
- Tests for editor or tooling layers
- Performance benchmarks (use `engine_perf_harness`)
- Integration tests spanning multiple layers

**Examples:**
- `startup_validate_case`: Tests engine startup validation
- `runtime_apply_tick`: Tests engine runtime tick application
- `storage_access_read_view`: Tests storage access read operations
- `world_spatial_address_case`: Tests world spatial addressing

**Default Routing:** Full (comprehensive engine verification)

---

### 2. sdk_canon_matrix

**Role:** Tests the SDK bridge canonical surface through `stratumx_test_support`

**Inclusion Criteria:**
- Tests that exercise SDK bridge APIs
- Tests for SDK lookup correctness, field invariants, allocation posture
- Tests for SDK pressure bounds, boundary legality, opacity preservation
- Tests for SDK snapshot swaps and canonical contracts

**Exclusion Criteria:**
- Tests for engine layer directly (use `engine_canon_matrix`)
- Tests for tooling or editor layers
- Tests for SDK-to-tooling integration (use `sdk_tooling_link_matrix`)
- Tests for engine-to-SDK integration (use `engine_sdk_link_matrix`)

**Examples:**
- `lookup_correctness`: Tests SDK lookup operations
- `field_invariants`: Tests SDK field invariant preservation
- `allocation_posture`: Tests SDK allocation behavior
- `pressure_bounds`: Tests SDK pressure boundary enforcement

**Default Routing:** Full (comprehensive SDK verification)

---

### 3. tooling_canon_matrix

**Role:** Tests the tooling authoring runtime canonical surface through `stratumx_test_support`

**Inclusion Criteria:**
- Tests for tooling layer canonical APIs
- Tests for command schema, transaction determinism, snapshot immutability
- Tests for authority isolation, cache eviction, build reproducibility
- Tests for validation legality, artifact manifest, index rebuild

**Exclusion Criteria:**
- Tests for SDK layer (use `sdk_canon_matrix`)
- Tests for editor layer (use `editor_canon_matrix`)
- Tests for SDK-to-tooling integration (use `sdk_tooling_link_matrix`)
- Tests for specific authoring domains (use domain-specific suites)

**Examples:**
- `command_schema`: Tests tooling command schema correctness
- `transaction_determinism`: Tests transaction determinism guarantees
- `snapshot_immutability`: Tests snapshot immutability contracts
- `authority_isolation`: Tests authority isolation enforcement

**Default Routing:** Full (comprehensive tooling verification)

---

### 4. editor_canon_matrix

**Role:** Tests the editor product canonical surface through `stratumx_test_support`

**Inclusion Criteria:**
- Tests for editor layer canonical APIs
- Tests for assistant surface, build/release surface, content browser
- Tests for diagnostics, inspector, outliner, viewport, workspace layout
- Tests for editor services, shell panels, tool context modes

**Exclusion Criteria:**
- Tests for engine, SDK, or tooling layers
- Tests for editor state management (use `editor_state_matrix`)
- Tests for editor application integration (use `editor_app_matrix`)
- Tests for editor command routing (use `editor_command_matrix`)

**Examples:**
- `assistant_surface`: Tests editor assistant UI surface
- `build_release_surface`: Tests editor build/release UI
- `content_browser_sync`: Tests content browser synchronization
- `viewport_navigation`: Tests viewport navigation controls

**Default Routing:** Full (comprehensive editor verification)

---

## Link/Integration Suites

### 5. engine_sdk_link_matrix

**Role:** Tests engine-to-SDK linking and compatibility at the integration boundary

**Inclusion Criteria:**
- Tests for engine/SDK integration boundary
- Tests for compatibility surface, observation batches, projection alignment
- Tests for runtime export, startup bridge between engine and SDK
- Tests verifying engine and SDK can communicate correctly

**Exclusion Criteria:**
- Tests for engine layer alone (use `engine_canon_matrix`)
- Tests for SDK layer alone (use `sdk_canon_matrix`)
- Tests for SDK-to-tooling integration (use `sdk_tooling_link_matrix`)

**Examples:**
- `compatibility_surface`: Tests engine/SDK compatibility
- `observation_batches`: Tests observation batch passing
- `projection_alignment`: Tests projection alignment between layers
- `startup_bridge`: Tests startup bridge initialization

**Default Routing:** Full (integration verification)

---

### 6. sdk_tooling_link_matrix

**Role:** Tests SDK-to-tooling linking and integration at the boundary

**Inclusion Criteria:**
- Tests for SDK/tooling integration boundary
- Tests for artifact alignment, assistant path, command lowering
- Tests for preview build/release, workspace bridge
- Tests verifying SDK and tooling can communicate correctly

**Exclusion Criteria:**
- Tests for SDK layer alone (use `sdk_canon_matrix`)
- Tests for tooling layer alone (use `tooling_canon_matrix`)
- Tests for engine-to-SDK integration (use `engine_sdk_link_matrix`)

**Examples:**
- `artifact_alignment`: Tests artifact alignment between SDK and tooling
- `command_lowering`: Tests command lowering from tooling to SDK
- `workspace_bridge`: Tests workspace bridge initialization
- `preview_build_release`: Tests preview build/release integration

**Default Routing:** Full (integration verification)

---

## State/Architecture Suites

### 7. editor_state_matrix

**Role:** Tests editor state management architecture and invariants

**Inclusion Criteria:**
- Tests for editor state container authority and ownership
- Tests for cache invalidation, derived state rebuildability
- Tests for domain service encapsulation, query layer delegation
- Tests for state graph acyclicity and state management patterns
- Property-based tests for state invariants

**Exclusion Criteria:**
- Tests for editor UI surface (use `editor_canon_matrix`)
- Tests for editor application integration (use `editor_app_matrix`)
- Tests for specific editor features (use appropriate canon matrix)

**Examples:**
- `state_container_authority`: Tests state container ownership rules
- `cache_invalidation`: Tests cache invalidation correctness
- `derived_state_rebuildability`: Tests derived state can be rebuilt
- `state_graph_acyclicity`: Tests state graph has no cycles

**Default Routing:** Full (state architecture verification)

---

### 8. tool_session_matrix

**Role:** Tests tooling session transaction semantics and invariants

**Inclusion Criteria:**
- Tests for tooling transaction all-or-nothing semantics
- Tests for audio/material reversibility in transactions
- Tests for transaction preconditions and postconditions
- Property-based tests for transaction invariants

**Exclusion Criteria:**
- Tests for tooling canonical surface (use `tooling_canon_matrix`)
- Tests for specific authoring domains (use domain-specific suites)
- Tests for SDK-to-tooling integration (use `sdk_tooling_link_matrix`)

**Examples:**
- `transaction_all_or_nothing`: Tests transaction atomicity
- `audio_reversibility`: Tests audio transaction reversibility
- `material_reversibility`: Tests material transaction reversibility
- `transaction_preconditions`: Tests transaction precondition enforcement

**Default Routing:** Full (transaction verification)

---

## Meta/Hygiene Suites

### 9. repo_hygiene

**Role:** Tests repository code quality and architectural rules enforcement

**Inclusion Criteria:**
- Tests for code quality rules (no allow attributes, no BOM, no debug modules)
- Tests for file organization rules (no empty dirs, file size limits, no inline tests)
- Tests for architectural rules (ownership validation, state classification, layer separation)
- Tests that scan repository structure and enforce standards

**Exclusion Criteria:**
- Tests for production functionality
- Tests for specific features or domains
- Tests that require runtime execution (use appropriate suite)

**Examples:**
- `no_allow_attributes`: Tests no #[allow] attributes in code
- `file_size_limits`: Tests files comply with size law
- `no_placeholder_stubs`: Tests no placeholder stubs remain
- `layer_separation`: Tests layer boundaries are respected

**Default Routing:** Verify (fast hygiene checks on every commit)

---

### 10. forbidden_shortcuts

**Role:** Tests that forbidden architectural shortcuts are not present in the codebase

**Inclusion Criteria:**
- Tests that scan UI files for forbidden shortcuts
- Tests for direct engine access bypassing services
- Tests for architectural boundary violations
- Tests enforcing architectural patterns

**Exclusion Criteria:**
- Tests for production functionality
- Tests for specific features
- Tests that require runtime execution

**Examples:**
- `no_direct_engine_access`: Tests UI doesn't directly access engine
- `no_service_bypass`: Tests services are not bypassed
- `architectural_boundaries`: Tests architectural boundaries are respected

**Default Routing:** Verify (architectural rule enforcement)

---

### 11. route_schema_golden

**Role:** Tests route manifest and envelope JSON schema stability through golden tests

**Inclusion Criteria:**
- Tests for route manifest JSON shape stability
- Tests for envelope JSON shape stability
- Golden tests that detect schema changes
- Tests verifying schema backward compatibility

**Exclusion Criteria:**
- Tests for route functionality (use appropriate suite)
- Tests for route execution (use appropriate suite)
- Tests for specific route implementations

**Examples:**
- `manifest_json_stable`: Tests manifest JSON shape is stable
- `envelope_json_stable`: Tests envelope JSON shape is stable

**Default Routing:** Verify (schema stability check)

---

## Workflow Suites

### 12. end_to_end_matrix

**Role:** Tests complete product workflows end-to-end across all layers

**Inclusion Criteria:**
- Tests for complete user workflows spanning multiple layers
- Tests for open world/save world cycles
- Tests for startup reference seed chain
- Tests for terrain/sky roundtrip workflows
- Tests that exercise the full stack

**Exclusion Criteria:**
- Tests for single layer functionality (use canonical suites)
- Tests for specific integration boundaries (use link suites)
- Tests for vertical slice demo (use `vertical_slice_quality_gates`)

**Examples:**
- `open_world_save_world_cycle`: Tests complete world persistence workflow
- `startup_reference_seed_chain`: Tests startup seed chain workflow
- `terrain_sky_roundtrip`: Tests terrain/sky authoring roundtrip

**Default Routing:** Smoke (critical path verification)

---

### 13. vertical_slice_quality_gates

**Role:** Tests vertical slice demo quality gates and critical functionality

**Inclusion Criteria:**
- Tests for vertical slice demo boot and initialization
- Tests for fire shot roundtrip in vertical slice
- Tests for DTO extraction in vertical slice
- Smoke tests for vertical slice critical path

**Exclusion Criteria:**
- Tests for complete product workflows (use `end_to_end_matrix`)
- Tests for specific layer functionality (use canonical suites)
- Tests for non-vertical-slice features

**Examples:**
- `vertical_slice_boot`: Tests vertical slice boots successfully
- `fire_shot_roundtrip`: Tests fire shot roundtrip in vertical slice
- `dto_extraction`: Tests DTO extraction in vertical slice

**Default Routing:** Smoke (vertical slice validation)

---

### 14. proof_region_integration

**Role:** Tests proof/evidence focus and history through route chains

**Inclusion Criteria:**
- Tests for proof contour evidence focus preservation
- Tests for evidence history through multiple button routes
- Tests for proof/evidence integration across routes

**Exclusion Criteria:**
- Tests for specific route functionality (use appropriate suite)
- Tests for route schema (use `route_schema_golden`)

**Examples:**
- `proof_contour_keeps_evidence_focus`: Tests evidence focus is preserved through route chains

**Default Routing:** Smoke (proof system integration)

---

### 15. focus_recovery_matrix

**Role:** Tests focus/retry/recovery contracts in canonical routes

**Inclusion Criteria:**
- Tests that canonical routes emit focus_target
- Tests that canonical routes emit retry_target
- Tests that canonical routes emit recovery_anchor
- Tests for focus recovery contract enforcement

**Exclusion Criteria:**
- Tests for specific route functionality (use appropriate suite)
- Tests for route schema (use `route_schema_golden`)

**Examples:**
- `routes_emit_focus_target`: Tests routes emit focus_target
- `routes_emit_retry_target`: Tests routes emit retry_target
- `routes_emit_recovery_anchor`: Tests routes emit recovery_anchor

**Default Routing:** Full (focus contract verification)

---

## Domain-Specific Suites

### 16. editor_command_matrix

**Role:** Tests editor command routing completeness and correctness

**Inclusion Criteria:**
- Tests for editor command manifest completeness
- Tests that manifest has full phase4 superset
- Tests that every route has canonical closure fields
- Tests for command routing correctness

**Exclusion Criteria:**
- Tests for editor UI surface (use `editor_canon_matrix`)
- Tests for command execution (use `editor_canon_matrix`)
- Tests for specific command implementations

**Examples:**
- `manifest_has_phase4_superset`: Tests manifest completeness
- `routes_have_closure_fields`: Tests routes have required fields

**Default Routing:** Verify (command routing verification)

---

### 17. editor_app_matrix

**Role:** Tests editor desktop application integration and initialization

**Inclusion Criteria:**
- Tests for editor desktop app initialization
- Tests for editor host setup
- Tests for GPU viewport renderer initialization
- Tests for viewport camera controller setup
- Application-level integration tests

**Exclusion Criteria:**
- Tests for editor UI surface (use `editor_canon_matrix`)
- Tests for editor state management (use `editor_state_matrix`)
- Tests for specific editor features (use `editor_canon_matrix`)

**Examples:**
- `desktop_app_initialization`: Tests desktop app initializes correctly
- `editor_host`: Tests editor host setup
- `gpu_viewport_renderer`: Tests GPU viewport renderer initialization
- `viewport_camera_controller`: Tests viewport camera controller

**Default Routing:** Smoke (application integration)

---

### 18. audio_authoring_matrix

**Role:** Tests audio authoring routes and ownership contracts

**Inclusion Criteria:**
- Tests for audio authoring routes being first-class and owner-backed
- Tests for audio preview/inspect routes executing with focus
- Tests for audio authoring canonical surface
- Tests specific to audio domain authoring

**Exclusion Criteria:**
- Tests for tooling canonical surface (use `tooling_canon_matrix`)
- Tests for SDK layer (use `sdk_canon_matrix`)
- Tests for other authoring domains (use appropriate domain suite)

**Examples:**
- `audio_routes_first_class`: Tests audio routes are first-class
- `audio_routes_owner_backed`: Tests audio routes are owner-backed
- `preview_inspect_with_focus`: Tests preview/inspect routes execute with focus

**Default Routing:** Full (audio authoring verification)

**Note:** Currently minimal tests (placeholder suite awaiting implementation)

---

### 19. material_authoring_matrix

**Role:** Tests material authoring routes and canonical surface

**Inclusion Criteria:**
- Tests for material authoring routes and contracts
- Tests for material preview/inspect functionality
- Tests specific to material domain authoring

**Exclusion Criteria:**
- Tests for tooling canonical surface (use `tooling_canon_matrix`)
- Tests for SDK layer (use `sdk_canon_matrix`)
- Tests for other authoring domains (use appropriate domain suite)

**Examples:**
- Material authoring route tests (to be implemented)

**Default Routing:** Full (material authoring verification)

**Note:** Currently empty/placeholder suite awaiting implementation

---

### 20. terrain_authoring_matrix

**Role:** Tests terrain authoring routes and canonical surface

**Inclusion Criteria:**
- Tests for terrain authoring routes and contracts
- Tests for terrain preview/inspect functionality
- Tests specific to terrain domain authoring

**Exclusion Criteria:**
- Tests for tooling canonical surface (use `tooling_canon_matrix`)
- Tests for SDK layer (use `sdk_canon_matrix`)
- Tests for other authoring domains (use appropriate domain suite)

**Examples:**
- Terrain authoring route tests (to be implemented)

**Default Routing:** Full (terrain authoring verification)

**Note:** Currently empty/placeholder suite awaiting implementation

---

### 21. environment_authoring_matrix

**Role:** Tests environment authoring routes and canonical surface

**Inclusion Criteria:**
- Tests for environment authoring routes and contracts
- Tests for environment preview/inspect functionality
- Tests specific to environment domain authoring (sky, weather, lighting)

**Exclusion Criteria:**
- Tests for tooling canonical surface (use `tooling_canon_matrix`)
- Tests for SDK layer (use `sdk_canon_matrix`)
- Tests for other authoring domains (use appropriate domain suite)

**Examples:**
- Environment authoring route tests (to be implemented)

**Default Routing:** Full (environment authoring verification)

**Note:** Currently empty/placeholder suite awaiting implementation

---

### 22. world_authoring_matrix

**Role:** Tests world authoring routes and canonical surface

**Inclusion Criteria:**
- Tests for world authoring routes and contracts
- Tests for world preview/inspect functionality
- Tests specific to world domain authoring

**Exclusion Criteria:**
- Tests for tooling canonical surface (use `tooling_canon_matrix`)
- Tests for SDK layer (use `sdk_canon_matrix`)
- Tests for other authoring domains (use appropriate domain suite)

**Examples:**
- World authoring route tests (to be implemented)

**Default Routing:** Full (world authoring verification)

**Note:** Currently empty/placeholder suite awaiting implementation

---

### 23. build_release_matrix

**Role:** Tests build/release route contracts and artifact handling

**Inclusion Criteria:**
- Tests for build result contract carrying release artifacts
- Tests for build result focus and recovery contracts
- Tests for build/release route correctness

**Exclusion Criteria:**
- Tests for editor build UI (use `editor_canon_matrix`)
- Tests for tooling build implementation (use `tooling_canon_matrix`)

**Examples:**
- `build_result_carries_artifacts`: Tests build result includes artifacts
- `build_result_focus_recovery`: Tests build result includes focus and recovery

**Default Routing:** Full (build/release contract verification)

**Note:** Currently minimal tests; may overlap with `editor_canon_matrix` build_release_surface

---

### 24. editor_shell_matrix

**Role:** Tests editor shell/view management infrastructure

**Inclusion Criteria:**
- Tests for view buttons being shell-owned
- Tests for shell routes publishing focus changes
- Tests for shell infrastructure and view management

**Exclusion Criteria:**
- Tests for editor shell panels UI (use `editor_canon_matrix`)
- Tests for editor state management (use `editor_state_matrix`)

**Examples:**
- `view_buttons_shell_owned`: Tests view buttons are shell-owned
- `shell_routes_publish_focus`: Tests shell routes publish focus changes

**Default Routing:** Full (shell infrastructure verification)

**Note:** Currently minimal tests; may overlap with `editor_canon_matrix` shell_panels

---

## Performance/Benchmark Suites

### 25. engine_perf_harness

**Role:** Performance benchmarking harness for engine operations

**Inclusion Criteria:**
- Performance benchmarks for engine operations
- Throughput and latency measurements
- Performance regression tests
- Benchmarks using real `2.engine/*` crates

**Exclusion Criteria:**
- Functional correctness tests (use `engine_canon_matrix`)
- Tests for other layers
- Integration tests

**Examples:**
- Engine operation benchmarks (to be implemented)

**Default Routing:** Full (performance tests)

**Note:** Currently empty/placeholder suite awaiting implementation

---

## Placeholder/Deprecated Suites

### 26. smoke

**Role:** DEPRECATED - Generic smoke test suite (to be consolidated)

**Status:** This suite violates the Suite Law due to its generic name and unclear purpose.

**Recommendation:** CONSOLIDATE into other suites:
- Fast critical path tests → `vertical_slice_quality_gates`
- Complete workflow tests → `end_to_end_matrix`
- Layer-specific smoke tests → appropriate canonical suites

**Current State:** Contains only minimal smoke test (placeholder)

**Action Required:** Define specific smoke test criteria OR consolidate into other suites

---

## Suite Consolidation Recommendations

Based on the analysis in SUITE_STRUCTURE_ANALYSIS.md, the following consolidations are recommended:

### Recommendation 1: Consolidate Authoring Matrices

**Suites to Consolidate:**
- `audio_authoring_matrix`
- `material_authoring_matrix`
- `terrain_authoring_matrix`
- `environment_authoring_matrix`
- `world_authoring_matrix`

**Consolidation Target:** `tooling_canon_matrix` (as domain-specific test families)

**Rationale:**
- All test authoring runtime canonical surface
- Currently mostly empty/placeholder suites
- Reduces suite count from 26 to 21
- Simplifies test organization
- Maintains clear domain separation through test families

**Alternative:** Keep as separate suites but define clear boundaries:
- Authoring matrices test domain-specific authoring integration
- `tooling_canon_matrix` tests tooling layer canonical surface
- Clear separation: domain integration vs layer canonical surface

---

### Recommendation 2: Clarify Editor Shell/Build Boundaries

**Suites with Potential Overlap:**
- `editor_shell_matrix` vs `editor_canon_matrix` (shell_panels.rs)
- `build_release_matrix` vs `editor_canon_matrix` (build_release_surface.rs)

**Recommendation:**
- **Option A:** Consolidate into `editor_canon_matrix` as focused test families
- **Option B:** Define clear boundaries:
  - `editor_shell_matrix`: Shell infrastructure and view management contracts
  - `editor_canon_matrix`: Shell panels UI surface
  - `build_release_matrix`: Build/release route contracts
  - `editor_canon_matrix`: Build/release UI surface

**Rationale:**
- Reduces potential confusion about test placement
- Eliminates overlapping responsibilities
- Simplifies suite structure

---

### Recommendation 3: Remove/Consolidate Smoke Suite

**Suite:** `smoke`

**Recommendation:** REMOVE and consolidate tests into:
- `vertical_slice_quality_gates` for vertical slice smoke tests
- `end_to_end_matrix` for workflow smoke tests
- Appropriate canonical suites for layer-specific smoke tests

**Rationale:**
- Generic "smoke" name violates Suite Law
- Unclear what tests belong here vs other smoke-level tests
- Currently empty/placeholder
- Smoke-level tests already exist in other suites

---

## Prohibited Suite Patterns

The following suite patterns are **PROHIBITED** under the Suite Law:

### 1. Generic Names
- ❌ `smoke` (too generic)
- ❌ `tests` (meaningless)
- ❌ `misc` (catch-all)
- ❌ `temp` (temporary)
- ❌ `utils` (unclear purpose)

### 2. Overlapping Responsibilities
- ❌ Two suites testing the same layer canonical surface
- ❌ Two suites testing the same integration boundary
- ❌ Suites with unclear boundaries between them

### 3. Temporary Suites
- ❌ Suites created "temporarily" for a feature
- ❌ Suites without documented long-term purpose
- ❌ Suites that duplicate existing suite roles

### 4. Catch-All Suites
- ❌ Suites that accept "anything that doesn't fit elsewhere"
- ❌ Suites without clear inclusion/exclusion criteria
- ❌ Suites that grow without bounds

---

## Adding New Suites

When adding a new test suite, you MUST:

1. **Define Single Role**: Document exactly one clear purpose
2. **Define Boundaries**: Specify inclusion and exclusion criteria
3. **Provide Examples**: Give concrete examples of appropriate tests
4. **Check for Overlap**: Verify no existing suite covers this role
5. **Document Routing**: Specify default routing category (verify/smoke/full)
6. **Update This Document**: Add the new suite to SUITE_LAW.md

**Approval Required:** New suites require review and approval to ensure they comply with Suite Law.

---

## Enforcement

The Suite Law is enforced through:

1. **Documentation**: This document defines the law
2. **Code Review**: Reviewers verify test placement matches suite roles
3. **Automated Checks**: `repo_hygiene` suite checks for violations
4. **New Test Law**: NEW_TEST_LAW.md references this document for test placement decisions

---

## Maintenance

This document SHALL be updated when:
- New suites are added
- Suite roles are clarified or changed
- Suites are consolidated or removed
- Inclusion/exclusion criteria are refined

**Last Updated:** Phase 3 Implementation - Package 2 Complete


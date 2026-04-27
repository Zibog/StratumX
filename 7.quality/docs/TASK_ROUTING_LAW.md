# Task Routing Law

**Purpose:** Define how tests are routed into verify, smoke, and full execution modes.

**Scope:** All test suites in `7.quality/suites/`

---

## Law Statement

**Every test MUST be routed into exactly one of three execution modes: verify, smoke, or full. Routing must be documented and justified.**

---

## Routing Categories

### Verify Mode

**Criteria:**
- Fast execution: <1s per test
- Critical path coverage
- Total verify mode: <5 min
- Run on every commit/PR
- Must be reliable (no flaky tests)

**What belongs here:**
- Unit tests for core invariants
- File size and hygiene checks
- Compilation checks
- Smoke-level canonical surface tests

**Examples:**
- `repo_hygiene/quality_file_size_check`
- `sdk_canon_matrix/lookup_correctness` (parameterized, fast)
- `engine_canon_matrix/startup_validate_case`

### Smoke Mode

**Criteria:**
- Representative scenarios
- Key integration points
- Total smoke mode: <15 min
- Run on PR merge to main
- May include slow I/O operations

**What belongs here:**
- Integration tests between layers
- End-to-end critical paths
- SDK-tooling link tests
- Editor state persistence

**Examples:**
- `engine_sdk_link_matrix/compatibility_surface`
- `sdk_tooling_link_matrix/artifact_alignment`
- `editor_state_matrix/property_single_state_ownership`

### Full Mode

**Criteria:**
- Comprehensive coverage
- All edge cases and parameterized variants
- Total full mode: <2 hours
- Run nightly or on demand
- May include resource-heavy operations

**What belongs here:**
- Property-based tests with many cases
- Full canonical matrix tests
- Large integration scenarios
- Performance verification tests

**Examples:**
- All proptest-based parameterized tests (200 cases each)
- `editor_canon_matrix/*_surface` tests
- `tooling_canon_matrix/*_surface` tests

---

## Routing Decision Criteria

### Decision Tree

```
Is this test a critical path check that runs in <1s?
├─ YES → Verify mode
│   └─ Example: file size check, compilation check
│
└─ NO → Is this an integration test or representative scenario?
    ├─ YES → Smoke mode
    │   └─ Example: SDK-tooling link, editor state
    │
    └─ NO → Full mode
        └─ Example: proptest with 200 cases, full matrix tests
```

### Quick Reference

| Test Type | Default Routing | Justification |
|-----------|----------------|---------------|
| Hygiene/meta checks | Verify | Fast, critical for repo health |
| Unit tests (<100ms) | Verify | Fast feedback on core logic |
| Integration tests | Smoke | Representative scenarios |
| Property tests (proptest) | Full | Comprehensive but slow |
| Matrix tests (200+ cases) | Full | Full coverage requires time |
| End-to-end workflows | Smoke/Full | Key paths matter most |

---

## Suite Routing Assignments

| Suite | Default Routing | Justification |
|-------|----------------|---------------|
| repo_hygiene | Verify | Fast checks, run every commit |
| engine_canon_matrix | Smoke/Full | Core engine, mix of fast and slow |
| sdk_canon_matrix | Full | Parameterized tests, comprehensive |
| tooling_canon_matrix | Full | Parameterized, 100 cases each |
| editor_canon_matrix | Full | Parameterized, 100 cases each |
| engine_sdk_link_matrix | Full | Parameterized integration |
| sdk_tooling_link_matrix | Full | Parameterized integration |
| editor_state_matrix | Smoke/Full | Property tests |
| editor_app_matrix | Smoke | UI/app integration |
| end_to_end_matrix | Full | Comprehensive workflows |
| tool_session_matrix | Full | Property tests |
| smoke | Verify | Quick smoke checks |
| audio_authoring_matrix | Full | Domain-specific comprehensive |
| build_release_matrix | Full | Build verification |
| environment_authoring_matrix | Full | Domain-specific |
| material_authoring_matrix | Full | Domain-specific |
| terrain_authoring_matrix | Full | Domain-specific |
| world_authoring_matrix | Full | Domain-specific |
| editor_command_matrix | Full | Command system |
| editor_shell_matrix | Full | Shell integration |
| route_schema_golden | Verify | Golden file checks |
| forbidden_shortcuts | Verify | Static checks |
| focus_recovery_matrix | Full | Recovery scenarios |
| proof_region_integration | Full | Integration proofs |
| vertical_slice_quality_gates | Full | Quality gates |

---

## Enforcement

### Automated Routing

Tests are routed via Cargo test attributes and CI configuration:

```toml
# .github/workflows/ci.yml
# verify: runs on every push/PR
# smoke: runs on PR merge to main
# full: runs nightly
```

### Manual Review

When adding new tests:
1. Choose routing category based on decision tree
2. Document routing in suite README if non-obvious
3. Ensure CI configuration includes the test in correct mode

---

## Related Documents

- [SUITE_LAW.md](./SUITE_LAW.md) - Suite roles
- [FILE_SIZE_LAW.md](./FILE_SIZE_LAW.md) - File size limits
- [NEW_TEST_LAW.md](./NEW_TEST_LAW.md) - Creating new tests

---

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-04-10 | 1.0 | Initial routing law document |

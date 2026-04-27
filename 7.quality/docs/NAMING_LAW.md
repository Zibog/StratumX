# Naming Law

**Purpose:** Establish naming standards for all test files, test functions, and test directories in the 7.quality contour.

**Scope:** All files in `7.quality/suites/*/tests/` and `7.quality/support/*/src/`

---

## Law Statement

**All test files, functions, and directories MUST have semantic, descriptive names that clearly describe what they test. Generic, ambiguous, or temporary names are PROHIBITED.**

---

## Test File Naming Standards

### Required Pattern

Test files MUST follow this naming pattern:

```
<domain>_<what_is_tested>_<aspect>.rs
```

Where:
- `<domain>` - The component or layer being tested (e.g., `lookup`, `field`, `storage`, `world_spatial`)
- `<what_is_tested>` - The specific concept or functionality (e.g., `correctness`, `invariants`, `allocation`)
- `<aspect>` - Optional, the specific aspect or sub-feature (e.g., `basic`, `edge_cases`, `concurrent`)

### Examples of Good Names

| Good Name | What It Tests |
|-----------|---------------|
| `lookup_correctness.rs` | Tests correctness of lookup operations |
| `field_invariants.rs` | Tests invariants on field operations |
| `storage_access_read_view.rs` | Tests read view functionality in storage access |
| `world_spatial_address.rs` | Tests spatial addressing in world |
| `tool_context_modes_basic.rs` | Tests basic context mode behavior |
| `assistant_surface_create.rs` | Tests assistant surface creation |
| `build_release_surface_validation.rs` | Tests validation in build release surface |
| `content_browser_sync_load.rs` | Tests content browser sync loading |

### Prohibited Names

The following patterns are **PROHIBITED**:

| Pattern | Example | Why Prohibited |
|---------|---------|----------------|
| Generic names | `tests.rs`, `test.rs`, `unit_tests.rs` | Don't describe what is being tested |
| Temporary names | `temp.rs`, `tmp.rs`, `scratch.rs` | Implies temporary work that should be finalized |
| Misc names | `misc.rs`, `miscellaneous.rs`, `other.rs` | Catch-all category indicates unclear purpose |
| Number-only names | `test_1.rs`, `case_5.rs` | Numbers provide no semantic information |
| Overly short names | `t.rs`, `x.rs`, `a.rs` | Impossible to understand purpose |
| Duplicates | `foo.rs`, `bar.rs`, `baz.rs` | Placeholder names with no meaning |

### Decision Tree: Naming a New Test File

```
Does the file name describe WHAT is being tested?
├─ NO → Choose a more descriptive name
│   └─ Example: tests.rs → lookup_correctness.rs
│
└─ YES → Does it describe the SPECIFIC aspect?
    ├─ NO → Add the specific aspect
    │   └─ Example: storage.rs → storage_access_read_view.rs
    │
    └─ YES → Is it consistent with existing naming patterns?
        ├─ NO → Align with existing conventions
        │   └─ Example: worldTests.rs → world_spatial_address.rs
        │
        └─ YES → Name is compliant
```

---

## Test Function Naming Standards

### Required Pattern

Test functions MUST follow this pattern:

```
fn <what_is_tested>_<scenario>_<expected_result>()
```

Or for parameterized tests:

```
fn <what_is_tested>_all_cases(case in strategy())
```

### Examples of Good Names

| Good Name | What It Tests |
|-----------|---------------|
| `fn lookup_correctness_all_cases()` | Parameterized test for all lookup correctness cases |
| `fn storage_access_read_view_succeeds_with_valid_descriptor()` | Tests that read view succeeds with valid descriptor |
| `fn world_spatial_address_returns_correct_chunk()` | Tests that spatial address returns correct chunk |
| `fn tool_session_create_succeeds_with_valid_params()` | Tests session creation succeeds with valid params |

### Prohibited Patterns

| Pattern | Example | Why Prohibited |
|---------|---------|----------------|
| Number-only names | `fn test_0()`, `fn case_1()` | No semantic information |
| Generic names | `fn test()`, `fn check()` | Unclear what is being tested |
| Temp names | `fn temp_test()`, `fn placeholder()` | Temporary work should be finalized |
| Empty assertions | `fn some_feature() { assert!(true); }` | No meaningful test content |

---

## Directory Naming Standards

### Test Suite Directories

Suite directories MUST match the suite's documented role in SUITE_LAW.md.

| Suite Directory | Role | Compliant |
|----------------|------|-----------|
| `engine_canon_matrix` | Engine canonical surface | ✅ |
| `sdk_canon_matrix` | SDK canonical surface | ✅ |
| `tooling_canon_matrix` | Tooling authoring surface | ✅ |
| `editor_canon_matrix` | Editor canonical surface | ✅ |
| `engine_sdk_link_matrix` | Engine-SDK integration | ✅ |
| `sdk_tooling_link_matrix` | SDK-Tooling integration | ✅ |
| `editor_state_matrix` | Editor state management | ✅ |
| `repo_hygiene` | Repository quality checks | ✅ |
| `smoke` | Smoke tests | ✅ |
| `end_to_end_matrix` | End-to-end workflow tests | ✅ |

### Support Module Directories

Support modules MUST follow:

```
7.quality/support/stratumx_<domain>_test_support/
```

Where `<domain>` is the component or layer the support is for.

| Support Directory | Purpose | Compliant |
|------------------|---------|-----------|
| `stratumx_test_support` | Core test support | ✅ |
| `stratumx_repo_hygiene_support` | Repo hygiene support | ✅ |
| `stratumx_route_test_support` | Route test support | ✅ |
| `stratumx_shell_test_support` | Shell test support | ✅ |

---

## Enforcement

### Automated Checks

The NAMING_LAW is enforced through:

1. **Repo hygiene check** - Scans for prohibited file names
2. **Manual review** - Pull requests reviewed for naming compliance

### Compliance Process

When a naming violation is found:

1. Identify the violating file
2. Choose a semantic name based the decision tree
3. Rename the file
4. Update `mod` declarations and `Cargo.toml` if needed
5. Run tests to verify no breakage
6. Update documentation references

---

## Migration Guide

### Renaming Existing Files

When renaming a file that violates the naming law:

```bash
# 1. Rename the file
git mv old_name.rs new_descriptive_name.rs

# 2. Update Cargo.toml (test discovery is automatic for [dev-dependencies])

# 3. Run tests to verify
cargo test

# 4. Commit the change
git commit -m "refactor: rename old_name.rs → new_descriptive_name.rs

Rename to comply with NAMING_LAW.md. The new name describes
the specific functionality being tested."
```

---

## Related Documents

- [SUITE_LAW.md](./SUITE_LAW.md) - Defines the single role for each test suite
- [FILE_SIZE_LAW.md](./FILE_SIZE_LAW.md) - Defines file size limits
- [NEW_TEST_LAW.md](./NEW_TEST_LAW.md) - Process for creating new tests
- [TASK_ROUTING_LAW.md](./TASK_ROUTING_LAW.md) - Defines test routing categories

---

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-04-10 | 1.0 | Initial naming law document |

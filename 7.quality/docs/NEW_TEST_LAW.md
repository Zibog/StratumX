# New Test Law

**Purpose:** Define the process for adding new tests to the 7.quality contour.

**Scope:** All new test additions to `7.quality/suites/`

---

## Law Statement

**Every new test MUST follow a documented process: choose suite, choose file, name test, write test, route test, and verify compliance.**

---

## Process Overview

### Step 1: Choose the Right Suite

Use this decision tree to select the appropriate suite:

```
What layer does the test target?
├─ Engine (2.engine/*) → engine_canon_matrix
│   ├─ Direct engine API? → engine_canon_matrix
│   └─ Engine through SDK? → engine_sdk_link_matrix
│
├─ SDK (3.sdk/*) → sdk_canon_matrix
│   ├─ SDK bridge API? → sdk_canon_matrix
│   └─ SDK through Tooling? → sdk_tooling_link_matrix
│
├─ Tooling (4.tooling/*) → tooling_canon_matrix
│   └─ Tooling authoring runtime? → tooling_canon_matrix
│
├─ Editor (5.editor/*) → editor_canon_matrix or editor_state_matrix
│   ├─ Editor surface/UI? → editor_canon_matrix
│   ├─ Editor state management? → editor_state_matrix
│   └─ Editor app/desktop? → editor_app_matrix
│
├─ Apps (6.apps/*) → editor_app_matrix or end_to_end_matrix
│   └─ End-to-end workflow? → end_to_end_matrix
│
└─ Cross-cutting → Specialized suite
    ├─ Repository quality? → repo_hygiene
    ├─ Smoke test? → smoke
    └─ Specific domain? → Domain-specific suite

Still unsure? → Review SUITE_LAW.md for detailed inclusion criteria.
```

### Step 2: Choose or Create the Test File

**Check existing files first:**
1. Is there an existing file testing this aspect? → Add to it
2. Would adding exceed 300 lines? → Create new file
3. Would adding exceed 800 lines? → MUST create new file

**Naming the new file:**
- Follow NAMING_LAW.md patterns
- Use: `<domain>_<what_is_tested>_<aspect>.rs`
- Example: `storage_access_read_view.rs`

### Step 3: Name the Test Function

**Follow NAMING_LAW.md patterns:**
- Descriptive name: `fn <what_is_tested>_<scenario>()`
- For parameterized: `fn <what_is_tested>_all_cases(case in strategy())`
- Prohibited: `test_1`, `check`, `temp_test`, numbered tests

### Step 4: Write the Test

**Test structure:**
```rust
#[test]
fn my_descriptive_test_name() {
    // 1. Setup - create test objects, configure state
    let system = create_test_system();
    
    // 2. Exercise - perform the operation being tested
    let result = system.do_something();
    
    // 3. Verify - assert expected behavior
    assert!(result.is_ok());
    assert_eq!(result.value, expected);
}
```

**For parameterized tests:**
```rust
use proptest::prelude::*;

fn my_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn my_test_all_cases(|case in my_case_strategy()) {
        // Test logic using case
    }
}
```

### Step 5: Route the Test

**Follow TASK_ROUTING_LAW.md:**

| Test Type | Routing |
|-----------|---------|
| Fast unit test (<100ms) | Verify |
| Integration scenario | Smoke |
| Parameterized (100+ cases) | Full |
| End-to-end workflow | Full |

### Step 6: Verify Compliance

**Checklist before committing:**

- [ ] **Test passes**: `cargo test --test <file_name>` succeeds
- [ ] **File size**: File is under 300 lines (or split if >800)
- [ ] **Naming**: File name follows NAMING_LAW.md
- [ ] **Function names**: Test functions are descriptive
- [ ] **Suite fit**: Test belongs in chosen suite (per SUITE_LAW.md)
- [ ] **Support code**: Used existing support modules where possible
- [ ] **Routing**: Test is in appropriate routing category
- [ ] **No embedded support**: Support code extracted to support modules

---

## Templates

### Basic Unit Test Template

```rust
//! <Brief description of what is tested>

#![allow(unused_imports, unused_mut, unused_variables)]

mod common;
use common::*;
use stratumx_test_support::*;

#[test]
fn <domain>_<concept>_<scenario>() {
    // Setup
    let system = create_test_system();
    
    // Exercise
    let result = system.do_operation();
    
    // Verify
    assert!(result.is_ok());
    assert_eq!(result.value, expected_value);
}
```

### Integration Test Template

```rust
//! <Brief description of integration>

#![allow(unused_imports, unused_mut, unused_variables)]

mod common;
use common::*;
use stratumx_test_support::*;

#[test]
fn <layer_a>_to_<layer_b>_<scenario>() {
    // Setup layer A
    let component_a = setup_component_a();
    
    // Setup layer B
    let component_b = setup_component_b(&component_a);
    
    // Exercise integration
    let result = component_b.perform_integrated_operation();
    
    // Verify
    assert!(result.is_ok());
    assert_integration_invariants(&result);
}
```

### Property-Based Test Template

```rust
//! <Brief description of property being tested>

#![allow(unused_imports, unused_mut, unused_variables)]

mod common;
use common::*;
use stratumx_test_support::*;
use proptest::prelude::*;

fn <property>_case_strategy() -> impl Strategy<Value = usize> {
    0usize..100
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn <property>_all_cases(|case in <property>_case_strategy()) {
        // Setup using case
        let system = setup_with_case(case);
        
        // Exercise
        let result = system.perform_operation();
        
        // Verify property holds
        prop_assert!(result.satisfies_property());
        prop_assert_eq!(result.invariant(), expected_invariant);
    }
}
```

---

## When to Create Support Code

**Create inline test code when:**
- Test-specific setup (less than 20 lines)
- Unique assertions for this test
- One-off test data

**Create support module code when:**
- Builder functions used by 2+ test files
- Fixtures shared across tests
- Common assertion helpers
- Case generators for parameterized tests

**Support code location:**
- Suite-specific: `7.quality/suites/<suite>/tests/support.rs`
- Cross-suite: `7.quality/support/stratumx_<domain>_test_support/`

---

## Examples

### Example 1: Adding a Unit Test

```rust
// File: 7.quality/suites/engine_canon_matrix/tests/storage_access_read_view.rs

#[test]
fn storage_access_read_view_returns_correct_data() {
    let handle = StableEntityHandle::new(EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    });
    let d = StorageAccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: true,
    };
    assert!(make_read_view(d, handle).is_ok());
}
```

### Example 2: Adding a Parameterized Test

```rust
// File: 7.quality/suites/sdk_canon_matrix/tests/new_property.rs

use proptest::prelude::*;

fn new_property_case_strategy() -> impl Strategy<Value = usize> {
    0usize..50
}

proptest! {
    #[test]
    fn new_property_all_cases(|case in new_property_case_strategy()) {
        let system = setup_with_case(case);
        prop_assert!(system.property_holds());
    }
}
```

---

## Verification Checklist

Before committing a new test, verify:

- [ ] Test passes locally (`cargo test --test <name>`)
- [ ] File size complies with FILE_SIZE_LAW.md (<300 lines normal)
- [ ] File name complies with NAMING_LAW.md
- [ ] Test function names are descriptive
- [ ] Test is in the right suite (per SUITE_LAW.md)
- [ ] Support code extracted if reused
- [ ] Routing category documented if non-obvious
- [ ] No `#[ignore]` without justification

---

## Related Documents

- [SUITE_LAW.md](./SUITE_LAW.md) - Suite roles and responsibilities
- [NAMING_LAW.md](./NAMING_LAW.md) - Naming standards
- [FILE_SIZE_LAW.md](./FILE_SIZE_LAW.md) - File size limits
- [TASK_ROUTING_LAW.md](./TASK_ROUTING_LAW.md) - Test routing categories
- [SPLIT_PLANS.md](./SPLIT_PLANS.md) - Plans for splitting large files

---

## Revision History

| Date | Version | Changes |
|------|---------|---------|
| 2026-04-10 | 1.0 | Initial new test law document |

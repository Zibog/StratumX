# Integration Tests for Repository Sanitization Phase 1

This directory contains end-to-end integration tests for the repository sanitization system.

## Test Organization

### Common Utilities (`common.rs`)
- `MockRepo`: Helper for creating temporary test repositories
- `create_repo_with_test_files()`: Creates a mock repo with test files in src/
- `create_repo_with_violations()`: Creates a mock repo with known hygiene violations
- `create_repo_with_waivers()`: Creates a mock repo with waiver registry
- `create_repo_for_rollback_test()`: Creates a mock repo for testing rollback scenarios

### Test Suites

#### Migration Workflow Tests (`test_migration_workflow.rs`)
Tests the complete test file migration process from production src/ to 7.quality/suites/.

**Key Tests:**
- `test_full_migration_workflow`: End-to-end migration of test files
- `test_migration_destination_mapping`: Verifies correct suite mapping
- `test_migration_import_updates`: Verifies import path updates
- `test_migration_dry_run`: Tests dry-run mode without actual file moves
- `test_migration_with_regression_files`: Verifies proptest regression file migration

**Requirements Validated:** 1.1-1.13, 12.1-12.6

#### Hygiene Check Workflow Tests (`test_hygiene_workflow.rs`)
Tests the automated hygiene checking system.

**Key Tests:**
- `test_full_hygiene_check_workflow`: End-to-end hygiene checking
- `test_line_limit_check`: Verifies 200-line limit enforcement
- `test_todo_comment_check`: Verifies TODO comment detection
- `test_allow_attribute_check`: Verifies #[allow(...)] detection
- `test_prohibition_check`: Verifies prohibited pattern detection
- `test_hygiene_report_format`: Verifies report structure and content
- `test_violation_context_extraction`: Verifies line number and context extraction

**Requirements Validated:** 6.1-6.12, 7.1-7.7, 8.1-8.8

#### Waiver Workflow Tests (`test_waiver_workflow.rs`)
Tests the waiver registry system for explicit rule exceptions.

**Key Tests:**
- `test_waiver_workflow_with_exemptions`: End-to-end waiver workflow
- `test_waiver_registry_loading`: Verifies TOML parsing and loading
- `test_waiver_registry_validation`: Verifies waiver entry validation
- `test_waiver_registry_detects_stale_entries`: Detects non-existent files
- `test_waiver_mechanism_correctness`: Verifies waiver checking logic
- `test_non_waived_violations_still_reported`: Ensures non-waived violations are detected
- `test_waiver_entry_format_validation`: Validates justification requirements
- `test_multiple_waivers_same_file`: Tests multiple waivers for one file

**Requirements Validated:** 9.1-9.7

#### Rollback Workflow Tests (`test_rollback_workflow.rs`)
Tests migration rollback and error recovery.

**Key Tests:**
- `test_migration_rollback_on_compilation_failure`: Verifies rollback on compilation errors
- `test_original_state_preserved_on_failure`: Ensures original files are preserved
- `test_partial_migration_failure_handling`: Tests handling of partial failures
- `test_migration_atomic_operation`: Verifies atomic migration operations
- `test_rollback_preserves_file_permissions`: Ensures file permissions are preserved
- `test_migration_log_on_failure`: Verifies failure logging

**Requirements Validated:** 12.1-12.6

## Running Integration Tests

Run all integration tests:
```bash
cargo test --test integration_tests
```

Run specific test suite:
```bash
cargo test --test integration_tests test_migration_workflow
cargo test --test integration_tests test_hygiene_workflow
cargo test --test integration_tests test_waiver_workflow
cargo test --test integration_tests test_rollback_workflow
```

Run specific test:
```bash
cargo test --test integration_tests test_full_migration_workflow
```

## Test Coverage

These integration tests provide end-to-end validation of:
- ✅ Test file discovery and migration
- ✅ Hygiene rule enforcement
- ✅ Waiver registry functionality
- ✅ Migration rollback and error recovery
- ✅ Import path updates
- ✅ Regression file preservation
- ✅ Violation reporting and remediation guidance
- ✅ Atomic operations and state preservation

## Notes

- All tests use temporary directories created with `tempfile::TempDir`
- Mock repositories are automatically cleaned up after tests complete
- Tests are isolated and can run in parallel
- Each test validates specific requirements from the design document

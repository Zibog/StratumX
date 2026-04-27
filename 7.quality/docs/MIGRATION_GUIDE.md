# Test Migration Guide

This guide documents the process for migrating test files from production `src/` directories to the `7.quality/suites/` structure as part of Phase 1 repository sanitization.

## Overview

The test migration system automatically moves test files from production source directories to the appropriate test suite locations, updating import paths and preserving all test logic and regression files.

## Migration Process

The migration follows a three-phase approach:

### Phase 1: Discovery

The system scans the repository to identify test files in production `src/` directories.

**What gets discovered**:
- `*.test.rs` files
- `*_tests.rs` files
- Files containing property test markers (`proptest!`, `prop_compose!`)
- Files containing invariant test markers
- Files with smoke test patterns

**Excluded from discovery**:
- Files already in `7.quality/suites/`
- Files in crate-level `tests/` directories
- Files in `.git/`, `target/`, or other build artifacts

### Phase 2: Migration

For each discovered test file, the system:

1. **Classifies the test type**:
   - Unit test
   - Integration test
   - Property test (proptest)
   - Invariant test
   - Smoke test

2. **Determines the destination suite** based on source location:
   - `5.editor/l7.0-editor-command-spine/src/` → `7.quality/suites/editor_canon_matrix/command_spine/`
   - `5.editor/l8.0-editor-shell/src/` → `7.quality/suites/editor_canon_matrix/editor_shell/`
   - `6.apps/editor/stratumx_editor_app/src/` → `7.quality/suites/editor_app_matrix/desktop_app/`
   - `4.tooling/l6.0-tool-session/src/` → `7.quality/suites/tooling_canon_matrix/tool_session/`
   - `5.editor/editor-state-containers/src/` → `7.quality/suites/editor_canon_matrix/state_containers/`

3. **Creates the destination directory structure** if it doesn't exist

4. **Moves the test file** using git-aware operations to preserve history

5. **Updates import paths**:
   - Analyzes all `use` statements in the test file
   - Calculates the correct relative path from the new location to production code
   - Updates import statements to reference production code correctly

6. **Moves proptest regression files**:
   - Detects `proptest-regressions/` directory adjacent to the test file
   - Moves all regression files to the destination suite
   - Preserves regression file structure

7. **Verifies compilation**:
   - Runs `cargo check` on the migrated test file
   - Reports compilation errors if any
   - Rolls back migration if compilation fails

8. **Logs the migration**:
   - Records source and destination paths
   - Records timestamp
   - Records number of imports updated
   - Records regression files moved
   - Records compilation status

### Phase 3: Verification

After migration, the system verifies:

1. **All migrated tests compile successfully**
2. **All migrated tests pass**
3. **Property test regression files are preserved and functional**
4. **No tests were lost or duplicated**

## Running Migration

### Dry Run (Preview Changes)

Before performing the actual migration, run a dry run to preview what will be migrated:

```bash
cargo run -p stratumx_quality_tasks -- migrate --dry-run
```

**Output**:
```
Running test migration...
DRY RUN MODE - No changes will be made
Scanning for test files in production src/ directories...
Found 5 test files to migrate:
  - 5.editor/l7.0-editor-command-spine/src/command_tests.rs (Unit)
  - 5.editor/l8.0-editor-shell/src/shell_property_tests.rs (Property)
  - 6.apps/editor/stratumx_editor_app/src/desktop_app/panel_tests.rs (Unit)
  - 4.tooling/l6.0-tool-session/src/session_tests.rs (Unit)
  - 5.editor/editor-state-containers/src/state_tests.rs (Unit)

Migrating 5.editor/l7.0-editor-command-spine/src/command_tests.rs...
  ✓ Would migrate to 7.quality/suites/editor_canon_matrix/command_spine/command_tests.rs
    - Would update 3 imports
    - Compilation: Not checked in dry-run mode

...

Migration complete (dry run):
  Would migrate: 5
  Would fail: 0
```

### Actual Migration

Once you've reviewed the dry run output and are ready to proceed:

```bash
cargo run -p stratumx_quality_tasks -- migrate
```

**Output**:
```
Running test migration...
Scanning for test files in production src/ directories...
Found 5 test files to migrate:
  - 5.editor/l7.0-editor-command-spine/src/command_tests.rs (Unit)
  - 5.editor/l8.0-editor-shell/src/shell_property_tests.rs (Property)
  - 6.apps/editor/stratumx_editor_app/src/desktop_app/panel_tests.rs (Unit)
  - 4.tooling/l6.0-tool-session/src/session_tests.rs (Unit)
  - 5.editor/editor-state-containers/src/state_tests.rs (Unit)

Migrating 5.editor/l7.0-editor-command-spine/src/command_tests.rs...
  ✓ Migrated to 7.quality/suites/editor_canon_matrix/command_spine/command_tests.rs
    - Updated 3 imports
    - Compilation: Success

Migrating 5.editor/l8.0-editor-shell/src/shell_property_tests.rs...
  ✓ Migrated to 7.quality/suites/editor_canon_matrix/editor_shell/shell_property_tests.rs
    - Updated 5 imports
    - Moved 1 regression files
    - Compilation: Success

Migrating 6.apps/editor/stratumx_editor_app/src/desktop_app/panel_tests.rs...
  ✓ Migrated to 7.quality/suites/editor_app_matrix/desktop_app/panel_tests.rs
    - Updated 2 imports
    - Compilation: Success

Migrating 4.tooling/l6.0-tool-session/src/session_tests.rs...
  ✓ Migrated to 7.quality/suites/tooling_canon_matrix/tool_session/session_tests.rs
    - Updated 4 imports
    - Compilation: Success

Migrating 5.editor/editor-state-containers/src/state_tests.rs...
  ✓ Migrated to 7.quality/suites/editor_canon_matrix/state_containers/state_tests.rs
    - Updated 3 imports
    - Compilation: Success

Migration complete:
  Successful: 5
  Failed: 0
```

### No Tests to Migrate

If no test files are found in production `src/` directories:

```bash
cargo run -p stratumx_quality_tasks -- migrate
```

**Output**:
```
Running test migration...
Scanning for test files in production src/ directories...
No test files found in production src/ directories.
```

## Verifying Migration Results

After migration, verify that everything works correctly:

### 1. Check Compilation

Verify that all migrated tests compile:

```bash
cargo check --workspace
```

**Expected**: No compilation errors

### 2. Run Migrated Tests

Run the tests in their new locations:

```bash
# Run all tests in editor_canon_matrix suite
cargo test -p editor_canon_matrix

# Run all tests in editor_app_matrix suite
cargo test -p editor_app_matrix

# Run all tests in tooling_canon_matrix suite
cargo test -p tooling_canon_matrix
```

**Expected**: All tests pass

### 3. Run Property Tests with Regression Files

If property tests were migrated, verify that regression files work:

```bash
# Run property tests
cargo test -p editor_canon_matrix property
```

**Expected**: Property tests pass and use regression files

### 4. Review Migration Log

Check the migration log for details:

```bash
cat 7.quality/docs/phase1_migration_log.md
```

**Expected**: Log contains entries for all migrated tests with timestamps, paths, and status

### 5. Run Hygiene Checks

Verify that no test files remain in production `src/` directories:

```bash
cargo run -p stratumx_quality_tasks -- verify
```

**Expected**: No violations for "Test Files in Production Src"

## Migration Log

All migration operations are logged to `7.quality/docs/phase1_migration_log.md`.

### Log Format

```markdown
# Phase 1 Migration Log

## Test Migrations

### 2024-01-15 14:32:00 UTC

**Source**: `5.editor/l7.0-editor-command-spine/src/command_tests.rs`
**Destination**: `7.quality/suites/editor_canon_matrix/command_spine/command_tests.rs`
**Type**: Unit Test
**Imports Updated**: 3
**Regression Files**: None
**Status**: Success

### 2024-01-15 14:33:15 UTC

**Source**: `5.editor/l8.0-editor-shell/src/shell_property_tests.rs`
**Destination**: `7.quality/suites/editor_canon_matrix/editor_shell/shell_property_tests.rs`
**Type**: Property Test
**Imports Updated**: 5
**Regression Files**: `proptest-regressions/shell_property_tests.txt`
**Status**: Success
```

### Log Contents

Each migration entry includes:
- **Source**: Original path of the test file
- **Destination**: New path in `7.quality/suites/`
- **Type**: Test type (Unit, Integration, Property, Invariant, Smoke)
- **Imports Updated**: Number of import statements modified
- **Regression Files**: List of proptest regression files moved
- **Status**: Success or failure with error details

## Troubleshooting

### Migration Fails with Compilation Error

**Symptom**:
```
Migrating 5.editor/l7.0-editor-command-spine/src/command_tests.rs...
  ✗ Failed: Compilation error after migration
```

**Cause**: Import paths could not be automatically updated correctly

**Solution**:
1. Check the migration log for details
2. Manually review the migrated test file
3. Correct import paths manually
4. Run `cargo check` to verify
5. If needed, report the issue for migration system improvement

### Regression Files Not Found

**Symptom**:
```
Migrating 5.editor/l8.0-editor-shell/src/shell_property_tests.rs...
  ✓ Migrated to 7.quality/suites/editor_canon_matrix/editor_shell/shell_property_tests.rs
    - Updated 5 imports
    - Moved 0 regression files
    - Compilation: Success
```

**Cause**: Regression files may not exist or may be in a non-standard location

**Solution**:
1. Check if regression files exist in the original location
2. If they exist but weren't moved, manually move them to the destination
3. Regression files should be in `proptest-regressions/` directory adjacent to the test file

### Test Fails After Migration

**Symptom**: Test passes in original location but fails after migration

**Cause**: Test may depend on file paths, working directory, or other location-specific factors

**Solution**:
1. Review the test code for hardcoded paths or assumptions about location
2. Update the test to use relative paths or configuration
3. Ensure test assets are also migrated if needed

### Import Path Issues

**Symptom**: Compilation errors about unresolved imports after migration

**Cause**: Complex import patterns that the migration system couldn't handle

**Solution**:
1. Manually review and fix import statements
2. Use absolute paths from crate root when possible
3. Ensure the test file can access production code from its new location

## Best Practices

### Before Migration

1. **Commit all changes**: Ensure your working directory is clean
2. **Run tests**: Verify all tests pass before migration
3. **Review dry run**: Always run `--dry-run` first to preview changes
4. **Backup**: Consider creating a branch for the migration

### During Migration

1. **Monitor output**: Watch for compilation errors or warnings
2. **Check logs**: Review the migration log for any issues
3. **Verify incrementally**: If migrating many files, verify in batches

### After Migration

1. **Run full test suite**: Ensure all tests still pass
2. **Run hygiene checks**: Verify no test files remain in `src/`
3. **Review changes**: Use `git diff` to review all changes
4. **Update documentation**: Update any documentation that references old test locations
5. **Commit**: Commit the migration with a clear message

## Rollback

If migration fails or causes issues, you can rollback:

### Using Git

```bash
# If you haven't committed yet
git restore .

# If you've committed
git revert <commit-hash>
```

### Manual Rollback

1. Move test files back to their original locations
2. Restore original import paths
3. Move regression files back
4. Run tests to verify

## Suite Organization

After migration, tests are organized in the following structure:

```
7.quality/suites/
├── editor_canon_matrix/
│   ├── command_spine/
│   │   ├── command_tests.rs
│   │   └── ...
│   ├── editor_shell/
│   │   ├── shell_property_tests.rs
│   │   ├── proptest-regressions/
│   │   │   └── shell_property_tests.txt
│   │   └── ...
│   └── state_containers/
│       ├── state_tests.rs
│       └── ...
├── editor_app_matrix/
│   └── desktop_app/
│       ├── panel_tests.rs
│       └── ...
└── tooling_canon_matrix/
    └── tool_session/
        ├── session_tests.rs
        └── ...
```

## Import Path Patterns

The migration system handles various import path patterns:

### Absolute Imports (from crate root)

**Before**:
```rust
use crate::command_registry::CommandRegistry;
```

**After** (from `7.quality/suites/editor_canon_matrix/command_spine/`):
```rust
use editor_command_spine::command_registry::CommandRegistry;
```

### Relative Imports

**Before**:
```rust
use super::command_registry::CommandRegistry;
```

**After**:
```rust
use editor_command_spine::command_registry::CommandRegistry;
```

### External Crate Imports

**Before and After** (unchanged):
```rust
use proptest::prelude::*;
```

## Regression File Handling

Property tests using `proptest` may have regression files that capture previously-found failing cases.

### Regression File Structure

**Before migration**:
```
5.editor/l8.0-editor-shell/src/
├── shell_property_tests.rs
└── proptest-regressions/
    └── shell_property_tests.txt
```

**After migration**:
```
7.quality/suites/editor_canon_matrix/editor_shell/
├── shell_property_tests.rs
└── proptest-regressions/
    └── shell_property_tests.txt
```

### Regression File Preservation

The migration system:
1. Detects `proptest-regressions/` directory adjacent to the test file
2. Moves all `.txt` files in the directory
3. Preserves the directory structure
4. Updates proptest configuration if needed

## Continuous Integration

After migration, update CI configuration to run tests from their new locations:

```yaml
# Example CI configuration
- name: Run Editor Canon Matrix Tests
  run: cargo test -p editor_canon_matrix

- name: Run Editor App Matrix Tests
  run: cargo test -p editor_app_matrix

- name: Run Tooling Canon Matrix Tests
  run: cargo test -p tooling_canon_matrix
```

## References

- **Hygiene Suite README**: `7.quality/suites/repo_hygiene/README.md`
- **Suite Index**: `7.quality/suites/README.md`
- **Migration Log**: `7.quality/docs/phase1_migration_log.md`
- **Requirements**: `.kiro/specs/repo-sanitization-phase-1/requirements.md`
- **Design**: `.kiro/specs/repo-sanitization-phase-1/design.md`

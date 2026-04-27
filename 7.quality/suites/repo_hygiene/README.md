# Repository Hygiene Suite

The `repo_hygiene` suite enforces automated quality rules and architectural discipline for the StratumX game engine repository. This suite implements Phase 1 of repository sanitization, ensuring production code remains clean, tests are properly organized, and architectural boundaries are maintained.

## Overview

The hygiene suite provides:
- **Automated hygiene checks** that fail the build when violated
- **Test migration** from production `src/` to `7.quality/suites/`
- **Code cleanup detection** for temporary adapters and bypasses
- **Waiver registry** for explicit, documented exceptions
- **Unified CLI** through `stratumx_quality_tasks` runner

## Hygiene Checks

The suite enforces the following automated checks:

### 1. Line Limit Check

**Rule**: No production code file may exceed 200 lines (excluding waived files)

**Rationale**: Enforces modularity and prevents monolithic files

**Violation Example**:
```
[LineLimit] 5.editor/l7.0-editor-command-spine/src/large_module.rs
  File has 347 lines (limit: 200)
  Remediation: Decompose into focused modules, each handling a single concern
```

**How to Fix**:
- Break large files into smaller, focused modules
- Extract related functionality into separate files
- If decomposition is not beneficial, add a waiver with justification

### 2. Test Files in Production Src

**Rule**: No test files (`*.test.rs`, `*_tests.rs`) may exist in production `src/` directories

**Rationale**: Tests should live in `7.quality/suites/` or crate-level `tests/` directories

**Violation Example**:
```
[TestFileInSrc] 5.editor/l7.0-editor-command-spine/src/command_tests.rs
  Test file found in production src/ directory
  Remediation: Run 'migrate' command to move tests to 7.quality/suites/
```

**How to Fix**:
```bash
cargo run -p stratumx_quality_tasks -- migrate
```

### 3. TODO Comments

**Rule**: No TODO comments may exist in accepted production code

**Rationale**: TODOs indicate incomplete work and should be tracked in issues

**Violation Example**:
```
[TodoComment] 6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs
  Line 145: // TODO: implement proper error handling
  Remediation: Create tracking issue and remove TODO, or implement the work
```

**How to Fix**:
- Complete the TODO work
- Create a tracking issue and remove the TODO comment
- If the TODO is intentional (e.g., in experimental code), add a waiver

### 4. Allow Attributes

**Rule**: No `#[allow(...)]` attributes may exist outside the waiver registry

**Rationale**: Suppressing warnings should be explicit and justified

**Violation Example**:
```
[AllowAttribute] 2.engine/l1-foundation/src/module.rs
  Line 23: #[allow(dead_code)]
  Remediation: Remove attribute and fix warning, or add to waiver registry with justification
```

**How to Fix**:
- Fix the underlying warning
- Add to waiver registry if the warning is unavoidable (e.g., FFI code)

### 5. Misplaced Assets

**Rule**: No documentation or test assets may exist in runtime crate directories

**Rationale**: Assets should live in appropriate locations (`1.docs/`, `7.quality/`)

**Violation Example**:
```
[DocAssetInRuntime] 2.engine/l1-foundation/architecture.md
  Documentation asset found in runtime crate
  Remediation: Move to 1.docs/canonical/ or appropriate documentation location
```

**How to Fix**:
- Move documentation to `1.docs/`
- Move test assets to `7.quality/data/`

### 6. Duplicate Module Names

**Rule**: No duplicate module names may exist across crates

**Rationale**: Prevents confusion and import ambiguity

**Violation Example**:
```
[DuplicateModule] 2.engine/l1-foundation/src/types.rs
  Duplicate module name found in 3.sdk/l2-sdk-core/src/types.rs
  Remediation: Rename one module to be more specific (e.g., foundation_types, sdk_types)
```

**How to Fix**:
- Rename modules to be more specific and descriptive

### 7. App Logic Violations

**Rule**: No business logic or domain logic may exist in app crates

**Rationale**: App crates should only contain UI coordination logic

**Violation Example**:
```
[AppLogic] 6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs
  Business logic detected in app crate
  Remediation: Extract to appropriate service layer
```

**How to Fix**:
- Extract business logic to editor services
- Route operations through Command_Spine

## Prohibition Checks

The suite also enforces prohibitions on known anti-patterns:

### 1. Static Mut Global Variables

**Rule**: No `static mut` global variables in production code

**Rationale**: Unsafe global mutable state violates Rust safety guarantees

**Violation Example**:
```
[StaticMut] 2.engine/l1-foundation/src/globals.rs
  Line 45: static mut GLOBAL_STATE: Option<State> = None;
  Remediation: Use thread-local storage, lazy_static, or proper state management
```

### 2. Execute Bridge Functions

**Rule**: No temporary `execute_*` bridge functions in production runtime

**Rationale**: Bridge functions are temporary scaffolding and should be removed

**Violation Example**:
```
[ExecuteBridge] 5.editor/l8.0-editor-shell/src/execute_temp.rs
  Line 67: fn execute_temporary_bridge(...) { ... }
  Remediation: Implement proper canonical action path
```

### 3. Host Bypass Patterns

**Rule**: No direct `self.host.*` calls in UI code without canonical routing

**Rationale**: All operations should route through Command_Spine

**Violation Example**:
```
[HostBypass] 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
  Line 145: self.host.save_file(path, content)
  Remediation: Route through Command_Spine SaveFile action
```

### 4. Hardcoded Paths

**Rule**: No hardcoded save/open paths in production code

**Rationale**: Paths should be configurable and user-controlled

**Violation Example**:
```
[HardcodedPath] 4.tooling/l6.0-tool-session/src/session.rs
  Line 89: let path = "/tmp/default.save";
  Remediation: Use configuration system or user-provided paths
```

### 5. Fake Truth Patterns

**Rule**: No "default fake truth" patterns in persistence code

**Rationale**: Persistence should handle real data, not fake defaults

**Violation Example**:
```
[FakeTruth] 2.engine/l0-world-truth/src/persistence.rs
  Line 123: let default_truth = Truth::fake_default();
  Remediation: Implement proper truth initialization from real data
```

## Waiver Registry

The waiver registry provides an explicit mechanism for documenting legitimate exceptions to hygiene rules.

### Format

The waiver registry is stored in `waivers.toml` with the following structure:

```toml
# Line Limit Waivers
[[line_limit_waivers]]
path = "relative/path/to/file.rs"
justification = "Explanation of why this file exceeds 200 lines"

# Allow Attribute Waivers
[[allow_attr_waivers]]
path = "relative/path/to/file.rs"
justification = "Explanation of why #[allow(...)] is needed"
```

### Requirements

Each waiver entry **must** include:
- `path`: Relative path from repository root to the file
- `justification`: Clear explanation of why the exception is necessary

### Guidelines

- **Waivers should be temporary** whenever possible
- Include a reference to a tracking issue or refactoring plan if applicable
- Waivers are version-controlled and require code review
- Stale waivers (referencing non-existent files) will be flagged during validation

### Example Waivers

```toml
[[line_limit_waivers]]
path = "5.editor/l7.0-editor-command-spine/src/legacy_registration.rs"
justification = "Legacy registration module scheduled for decomposition in Phase 2 (Issue #123)"

[[allow_attr_waivers]]
path = "2.engine/l0-world-truth/src/ffi_bridge.rs"
justification = "FFI bridge requires #[allow(improper_ctypes)] for C interop with external library"
```

### Adding a Waiver

1. Edit `7.quality/suites/repo_hygiene/waivers.toml`
2. Add a new entry with path and justification
3. Run verification to ensure the waiver is valid:
   ```bash
   cargo run -p stratumx_quality_tasks -- verify
   ```
4. Submit for code review (waivers require explicit approval)

### Waiver Validation

The hygiene checker validates all waivers on each run:
- Ensures all referenced files exist
- Reports stale waivers (files that no longer exist)
- Fails if waiver format is invalid

## Running Hygiene Checks

### Run All Checks

```bash
cargo run -p stratumx_quality_tasks -- verify
```

**Output**:
```
Running hygiene checks...
Repository root: /path/to/stratumx

=== Hygiene Check Results ===
Passed checks: 8
Failed checks: 0
Execution time: 2.3s

All hygiene checks passed!
```

### Run with Verbose Output

```bash
cargo run -p stratumx_quality_tasks -- verify --verbose
```

**Output includes**:
- Repository root path
- Waiver registry status
- Detailed check execution
- All violations with context

### Run Specific Suite

```bash
cargo run -p stratumx_quality_tasks -- verify --suite repo_hygiene
```

### Expected Output on Violations

```
Found 3 violations:

  [LineLimit] 5.editor/l7.0-editor-command-spine/src/large_module.rs
    File has 347 lines (limit: 200)
    Remediation: Decompose into focused modules, each handling a single concern

  [TodoComment] 6.apps/editor/stratumx_editor_app/src/desktop_app/panel.rs
    Line 145: // TODO: implement proper error handling
    Remediation: Create tracking issue and remove TODO, or implement the work

  [HostBypass] 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
    Line 145: self.host.save_file(path, content)
    Remediation: Route through Command_Spine SaveFile action

Hygiene checks failed with 3 violations
```

### Exit Codes

- **0**: All checks passed
- **1**: One or more checks failed

## Test Migration

The hygiene suite includes a test migration system that moves test files from production `src/` directories to `7.quality/suites/`.

### Running Migration

```bash
# Dry run (preview changes without making them)
cargo run -p stratumx_quality_tasks -- migrate --dry-run

# Actual migration
cargo run -p stratumx_quality_tasks -- migrate
```

### What Gets Migrated

The migration system identifies and moves:
- `*.test.rs` files
- `*_tests.rs` files
- Property test files (containing `proptest!` or `prop_compose!`)
- Invariant test files
- Smoke test files

### Migration Process

For each test file, the migration system:

1. **Identifies the test type** (unit, integration, property, invariant, smoke)
2. **Determines the target suite** based on source location:
   - `5.editor/l7.0-editor-command-spine/` → `7.quality/suites/editor_canon_matrix/command_spine/`
   - `5.editor/l8.0-editor-shell/` → `7.quality/suites/editor_canon_matrix/editor_shell/`
   - `6.apps/editor/stratumx_editor_app/` → `7.quality/suites/editor_app_matrix/desktop_app/`
   - `4.tooling/l6.0-tool-session/` → `7.quality/suites/tooling_canon_matrix/tool_session/`
   - `5.editor/editor-state-containers/` → `7.quality/suites/editor_canon_matrix/state_containers/`
3. **Creates the destination directory** if it doesn't exist
4. **Moves the test file** using git-aware operations
5. **Updates import paths** to reference production code from the new location
6. **Moves proptest regression files** if they exist
7. **Verifies compilation** of the migrated test
8. **Logs the migration** to `7.quality/docs/phase1_migration_log.md`

### Migration Output

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

...

Migration complete:
  Successful: 5
  Failed: 0
```

### Migration Log

All migrations are logged to `7.quality/docs/phase1_migration_log.md` with:
- Source and destination paths
- Timestamp
- Number of imports updated
- Regression files moved
- Compilation status

## Code Cleanup

The hygiene suite includes a code cleaner that identifies code requiring manual refactoring.

### Running Cleanup Detection

```bash
cargo run -p stratumx_quality_tasks -- clean --report-only
```

### What Gets Detected

The cleanup system identifies:

1. **Host Bypasses**: Direct `self.host.*` calls in UI code
2. **Registration Blobs**: Large registration modules (>200 lines) or modules mixing concerns
3. **Domain Logic in UI**: Parser implementations, validation logic, or business rules in app crates

### Cleanup Report Output

```
Generating cleanup report...
REPORT ONLY MODE - No changes will be made

=== Cleanup Report ===

Host Bypasses Found: 2
  File: 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
  Line: 145
  Pattern: self.host.save_file(path, content)
  Action: Route through Command_Spine SaveFile action

  File: 6.apps/editor/stratumx_editor_app/src/desktop_app/asset_panel.rs
  Line: 203
  Pattern: self.host.open_file_dialog()
  Action: Route through Command_Spine OpenFileDialog action

Registration Blobs Found: 1
  File: 5.editor/l7.0-editor-command-spine/src/registration.rs
  Lines: 347
  Mixed Concerns: command_registration, menu_registration, keybinding_registration
  Suggested Decomposition:
    - Extract command_registration to separate module
    - Extract menu_registration to separate module
    - Extract keybinding_registration to separate module

Domain Logic Violations Found: 1
  File: 6.apps/editor/stratumx_editor_app/src/desktop_app/asset_panel.rs
  Type: Parser
  Lines: 89-145
  Target Layer: 4.tooling/asset-parser or appropriate service layer

Total issues found: 4

These issues require manual refactoring.
```

### Remediation Strategies

**For Host Bypasses**:
1. Define a command in Command_Spine
2. Route the operation through the command
3. Remove the direct host call

**For Registration Blobs**:
1. Identify distinct concerns in the blob
2. Extract each concern to a focused module
3. Keep registration modules under 200 lines

**For Domain Logic in UI**:
1. Identify the appropriate service layer
2. Extract the logic to that layer
3. Call the service from the UI

## Auto-Fix

The hygiene suite includes an auto-fix command for correctable violations.

### Running Auto-Fix

```bash
# Fix all auto-correctable violations
cargo run -p stratumx_quality_tasks -- fix

# Fix a specific rule
cargo run -p stratumx_quality_tasks -- fix --rule line_limit
```

### Current Limitations

Most violations require manual intervention. The auto-fix command currently:
- Identifies violations
- Reports which violations can be auto-fixed (currently none)
- Reports which violations require manual intervention (most)

Future enhancements may add auto-fix capabilities for:
- Removing trailing whitespace
- Formatting issues
- Simple import path corrections

## Integration with CI

The hygiene checks should be integrated into CI pipelines:

```yaml
# Example CI configuration
- name: Run Hygiene Checks
  run: cargo run -p stratumx_quality_tasks -- verify
```

This ensures:
- All PRs are checked for hygiene violations
- Violations block merging
- Code quality is maintained continuously

## Performance

The hygiene suite is designed for fast execution:

- **Hygiene checks**: Complete within 10 seconds
- **Full verification**: Complete within 30 seconds
- **Migration**: Processes >10 files/second

## Architecture

The hygiene suite consists of five primary components:

1. **Discovery Scanner**: Traverses the repository to identify violations and migration candidates
2. **Test Migrator**: Moves test files from production `src/` to `7.quality/suites/`
3. **Code Cleaner**: Identifies temporary code, bypasses, and misplaced domain logic
4. **Hygiene Checker**: Enforces automated quality rules with waiver registry support
5. **Quality Task Runner**: Provides unified CLI interface for all verification operations

## Testing

The hygiene suite itself is thoroughly tested:

- **Unit tests**: Test specific examples and edge cases
- **Property tests**: Verify universal correctness properties
- **Integration tests**: Test end-to-end workflows

Run the hygiene suite tests:

```bash
cargo test -p repo_hygiene
```

## References

- **Requirements**: `.kiro/specs/repo-sanitization-phase-1/requirements.md`
- **Design**: `.kiro/specs/repo-sanitization-phase-1/design.md`
- **Tasks**: `.kiro/specs/repo-sanitization-phase-1/tasks.md`
- **Migration Log**: `7.quality/docs/phase1_migration_log.md`
- **Suite Index**: `7.quality/suites/README.md`

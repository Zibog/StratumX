# StratumX Quality Task Runner

The `stratumx_quality_tasks` crate provides a unified command-line interface for all quality operations in the StratumX game engine repository, including hygiene checks, test migration, code cleanup, and quality verification.

## Overview

The quality task runner provides the following commands:

- **verify**: Run automated hygiene checks
- **migrate**: Migrate test files from `src/` to `7.quality/suites/`
- **clean**: Generate cleanup report for manual refactoring
- **fix**: Auto-fix correctable violations
- **smoke**: Run smoke tests
- **full**: Run full test suite with formatting and linting
- **bench**: Run performance benchmarks
- **metrics**: Generate quality metrics
- **evidence**: Generate evidence pack
- **inventory**: Generate inventory of test files for Quality Contour Surgery Phase 3
- **gold**: Run all quality commands (full + bench + metrics + evidence)

## Installation

The quality task runner is part of the StratumX workspace and doesn't require separate installation.

## Usage

All commands follow the pattern:

```bash
cargo run -p stratumx_quality_tasks -- <command> [options]
```

## Commands

### verify

Run automated hygiene checks to ensure code quality and architectural discipline.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- verify [OPTIONS]
```

**Options**:
- `--suite <SUITE>`: Run checks for a specific suite
- `--verbose`: Enable verbose output

**Examples**:

```bash
# Run all hygiene checks
cargo run -p stratumx_quality_tasks -- verify

# Run with verbose output
cargo run -p stratumx_quality_tasks -- verify --verbose

# Run checks for a specific suite
cargo run -p stratumx_quality_tasks -- verify --suite repo_hygiene
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

**Exit Codes**:
- `0`: All checks passed
- `1`: One or more checks failed

**What It Checks**:
- Line limit violations (200 line limit)
- Test files in production `src/` directories
- TODO comments in production code
- `#[allow(...)]` attributes outside waiver registry
- Misplaced documentation or test assets
- Duplicate module names
- Business logic in app crates
- Prohibited patterns (static mut, execute_* bridges, host bypasses, hardcoded paths, fake truth)

**See Also**:
- [Hygiene Suite README](../../suites/repo_hygiene/README.md)

---

### migrate

Migrate test files from production `src/` directories to `7.quality/suites/` with proper organization and import path updates.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- migrate [OPTIONS]
```

**Options**:
- `--dry-run`: Perform a dry run without making changes

**Examples**:

```bash
# Preview migration without making changes
cargo run -p stratumx_quality_tasks -- migrate --dry-run

# Perform actual migration
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

...

Migration complete:
  Successful: 5
  Failed: 0
```

**What It Does**:
- Identifies test files in production `src/` directories
- Determines the appropriate destination suite
- Moves test files to `7.quality/suites/`
- Updates import paths to reference production code
- Moves proptest regression files
- Verifies compilation of migrated tests
- Logs all migrations to `7.quality/docs/phase1_migration_log.md`

**See Also**:
- [Migration Guide](../../docs/MIGRATION_GUIDE.md)

---

### clean

Generate a cleanup report identifying code that requires manual refactoring (host bypasses, registration blobs, domain logic in UI).

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- clean [OPTIONS]
```

**Options**:
- `--report-only`: Only generate report without making changes (default behavior)

**Examples**:

```bash
# Generate cleanup report
cargo run -p stratumx_quality_tasks -- clean --report-only
```

**Output**:

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

**What It Detects**:
- **Host Bypasses**: Direct `self.host.*` calls in UI code
- **Registration Blobs**: Large registration modules (>200 lines) or modules mixing concerns
- **Domain Logic in UI**: Parsers, validators, or business rules in app crates

**See Also**:
- [Cleanup Guide](../../docs/CLEANUP_GUIDE.md)

---

### fix

Auto-fix correctable violations (currently most violations require manual intervention).

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- fix [OPTIONS]
```

**Options**:
- `--rule <RULE>`: Fix a specific rule

**Examples**:

```bash
# Fix all auto-correctable violations
cargo run -p stratumx_quality_tasks -- fix

# Fix a specific rule
cargo run -p stratumx_quality_tasks -- fix --rule line_limit
```

**Output**:

```
Running auto-fix...
Fixing all auto-correctable violations

Auto-fix complete:
  Fixed: 0
  Requires manual intervention: 12

Run 'verify' command to see details of violations requiring manual fixes.
```

**Current Limitations**:

Most violations require manual intervention. Future enhancements may add auto-fix capabilities for:
- Removing trailing whitespace
- Formatting issues
- Simple import path corrections

**See Also**:
- [Hygiene Suite README](../../suites/repo_hygiene/README.md)

---

### smoke

Run smoke tests for core functionality across all major subsystems.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- smoke
```

**What It Runs**:
- `engine_sdk_link_matrix` tests
- `sdk_canon_matrix` tests
- `sdk_tooling_link_matrix` tests
- `tooling_canon_matrix` tests
- `editor_canon_matrix` tests

**Output**:

```
running 15 tests from engine_sdk_link_matrix
test result: ok. 15 passed; 0 failed; 0 ignored

running 23 tests from sdk_canon_matrix
test result: ok. 23 passed; 0 failed; 0 ignored

...
```

**Exit Codes**:
- `0`: All smoke tests passed
- `1`: One or more smoke tests failed

---

### full

Run the full test suite including formatting checks, linting, and all tests.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- full
```

**What It Runs**:
1. `cargo fmt --all --check` - Verify code formatting
2. `cargo clippy --workspace --all-targets -- -D warnings` - Run linter
3. `cargo test --workspace --all-targets -- --nocapture` - Run all tests

**Output**:

```
Checking formatting...
Checking clippy...
Running tests...

test result: ok. 1247 passed; 0 failed; 0 ignored
```

**Exit Codes**:
- `0`: All checks and tests passed
- `1`: One or more checks or tests failed

---

### bench

Run performance benchmarks for the engine.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- bench
```

**What It Runs**:
- `cargo bench -p engine_perf_harness`

**Output**:

```
running 8 benchmarks
test ecs_query_benchmark ... bench:   1,234 ns/iter (+/- 45)
test physics_step_benchmark ... bench:  12,345 ns/iter (+/- 234)
...
```

**Exit Codes**:
- `0`: Benchmarks completed successfully
- `1`: Benchmarks failed

---

### metrics

Generate quality metrics for the repository.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- metrics
```

**What It Does**:
- Generates a metrics summary
- Writes to `7.quality/data/generated/metrics/metrics-summary.md`

**Output**:

```
Generating metrics...
Metrics written to 7.quality/data/generated/metrics/metrics-summary.md
```

**Exit Codes**:
- `0`: Metrics generated successfully
- `1`: Metrics generation failed

---

### evidence

Generate an evidence pack documenting quality status.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- evidence
```

**What It Does**:
- Generates an evidence pack
- Writes to `7.quality/data/generated/evidence/evidence-pack.md`

**Output**:

```
Generating evidence pack...
Evidence pack written to 7.quality/data/generated/evidence/evidence-pack.md
```

**Exit Codes**:
- `0`: Evidence pack generated successfully
- `1`: Evidence pack generation failed

---

### inventory

Generate a comprehensive inventory of all test files in the quality contour for Phase 3: Quality Contour Surgery.

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- inventory
```

**What It Does**:
- Scans all test files in `7.quality/suites/`
- Counts lines (excluding blank lines and comments)
- Classifies files by size category (Normal 0-300, Suspicious 301-500, Requires Split 501-800, Forbidden 800+)
- Identifies test families (groups of related tests)
- Detects embedded support code (builders, fixtures, assertions, case generators)
- Classifies tests by domain (engine, sdk, editor, tooling, authoring)
- Generates comprehensive INVENTORY.md document

**Output**:

```
Starting inventory generation...
✓ Ensured 7.quality/docs/ directory exists
✓ Scanned 239 test files
✓ Generated INVENTORY.md at "7.quality/docs/INVENTORY.md"
```

**Generated Document Structure**:

The `INVENTORY.md` document includes:

1. **Giant Files Requiring Splitting (800+ lines)**: Lists all files exceeding 800 lines with their test families
2. **Files by Size Category**: Summary counts for each size category
3. **Test Families Within Giant Files**: Detailed breakdown of test families in large files
4. **Embedded Support Code Locations**: Identifies builders, fixtures, assertions, and case generators embedded in test files
5. **Suite-by-Suite Breakdown**: Statistics for each test suite
6. **Domain Classification Summary**: Distribution of tests across domains
7. **Summary Statistics**: Overall metrics (total files, tests, lines, averages)

**Example Output Section**:

```markdown
## 1. Giant Files Requiring Splitting (800+ lines)

Found 56 giant files:

- **7.quality/suites/editor_canon_matrix/tests/build_release_surface.rs** (2612 lines, 100 tests)
  - Suite: editor_canon_matrix
  - Domain: editor
  - Test families:
    - build_release_surface (100 tests, lines 15-2787)
```

**Use Cases**:

- **Package 1 of Quality Contour Surgery**: Initial inventory and classification
- **Identifying giant files**: Find files that need splitting (800+ lines)
- **Planning support code extraction**: Locate embedded builders, fixtures, and assertions
- **Understanding test distribution**: See how tests are distributed across suites and domains
- **Baseline metrics**: Establish before-state metrics for restructuring

**Exit Codes**:
- `0`: Inventory generated successfully
- `1`: Inventory generation failed

**See Also**:
- [Quality Contour Surgery Phase 3 Spec](.kiro/specs/quality-contour-surgery-phase-3/)
- [INVENTORY.md](../../docs/INVENTORY.md)

---

### gold

Run all quality commands to achieve "gold" status (full + bench + metrics + evidence).

**Usage**:
```bash
cargo run -p stratumx_quality_tasks -- gold
```

**What It Runs**:
1. `full` - Full test suite with formatting and linting
2. `bench` - Performance benchmarks
3. `metrics` - Quality metrics
4. `evidence` - Evidence pack

**Output**:

```
Running full test suite...
Running benchmarks...
Generating metrics...
Generating evidence pack...

Gold status achieved!
Phase status written to 7.quality/data/generated/gold/phase-status.md
```

**Exit Codes**:
- `0`: All quality commands passed
- `1`: One or more quality commands failed

---

## Common Workflows

### Daily Development

```bash
# Before committing
cargo run -p stratumx_quality_tasks -- verify
```

### Before Pull Request

```bash
# Run full test suite
cargo run -p stratumx_quality_tasks -- full
```

### Phase 1 Sanitization

```bash
# 1. Check current state
cargo run -p stratumx_quality_tasks -- verify

# 2. Preview test migration
cargo run -p stratumx_quality_tasks -- migrate --dry-run

# 3. Perform migration
cargo run -p stratumx_quality_tasks -- migrate

# 4. Generate cleanup report
cargo run -p stratumx_quality_tasks -- clean --report-only

# 5. Verify final state
cargo run -p stratumx_quality_tasks -- verify
```

### Release Preparation

```bash
# Achieve gold status
cargo run -p stratumx_quality_tasks -- gold
```

## Configuration

### Waiver Registry

The waiver registry is located at `7.quality/suites/repo_hygiene/waivers.toml`.

To add a waiver:

1. Edit `waivers.toml`
2. Add an entry with path and justification
3. Run `verify` to validate the waiver

**Example**:

```toml
[[line_limit_waivers]]
path = "5.editor/l7.0-editor-command-spine/src/legacy_registration.rs"
justification = "Legacy registration module scheduled for decomposition in Phase 2"
```

### Repository Root

The quality task runner automatically detects the repository root by traversing up from the crate location.

## Exit Codes

All commands use consistent exit codes:

- `0`: Success
- `1`: Failure

This allows for easy integration with CI/CD pipelines and shell scripts.

## Output Directories

The quality task runner creates the following output directories:

```
7.quality/data/generated/
├── smoke/
│   ├── engine/
│   ├── sdk/
│   ├── shared/
│   ├── tooling/
│   ├── editor/
│   └── apps/
├── test-results/
├── bench/
│   └── engine/
├── metrics/
│   └── metrics-summary.md
├── evidence/
│   └── evidence-pack.md
└── gold/
    └── phase-status.md
```

These directories are created automatically on first run.

## Integration with CI

### GitHub Actions Example

```yaml
name: Quality Checks

on: [push, pull_request]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Hygiene Checks
        run: cargo run -p stratumx_quality_tasks -- verify
      - name: Run Full Test Suite
        run: cargo run -p stratumx_quality_tasks -- full
```

### GitLab CI Example

```yaml
quality:
  stage: test
  script:
    - cargo run -p stratumx_quality_tasks -- verify
    - cargo run -p stratumx_quality_tasks -- full
```

## Troubleshooting

### Command Not Found

**Problem**: `cargo run -p stratumx_quality_tasks` fails with "package not found"

**Solution**: Ensure you're running the command from the repository root

### Waiver Registry Errors

**Problem**: Waiver registry validation fails

**Solution**: Check that all paths in `waivers.toml` reference existing files

### Migration Failures

**Problem**: Test migration fails with compilation errors

**Solution**: Check the migration log at `7.quality/docs/phase1_migration_log.md` for details

### Slow Execution

**Problem**: Quality commands take too long to execute

**Solution**: Use `--suite` flag to run specific suites, or run commands in parallel

## Performance

Expected execution times on a standard development machine:

| Command | Expected Time |
|---------|---------------|
| verify | < 10 seconds |
| migrate | < 30 seconds (depends on number of files) |
| clean | < 5 seconds |
| fix | < 10 seconds |
| smoke | < 30 seconds |
| full | < 5 minutes |
| bench | < 2 minutes |
| metrics | < 5 seconds |
| evidence | < 5 seconds |
| gold | < 10 minutes |

## Development

### Building

```bash
cargo build -p stratumx_quality_tasks
```

### Testing

```bash
cargo test -p stratumx_quality_tasks
```

### Adding New Commands

To add a new command:

1. Add a variant to the `Commands` enum in `src/main.rs`
2. Implement the command handler function
3. Add the command to the match statement in `main()`
4. Add tests in `tests/`
5. Update this README

## References

- **Hygiene Suite README**: `../../suites/repo_hygiene/README.md`
- **Migration Guide**: `../../docs/MIGRATION_GUIDE.md`
- **Cleanup Guide**: `../../docs/CLEANUP_GUIDE.md`
- **Suite Index**: `../../suites/README.md`
- **Requirements**: `.kiro/specs/repo-sanitization-phase-1/requirements.md`
- **Design**: `.kiro/specs/repo-sanitization-phase-1/design.md`

## Support

For issues or questions:

1. Check the relevant guide (Hygiene, Migration, or Cleanup)
2. Review the spec documents in `.kiro/specs/repo-sanitization-phase-1/`
3. Check the migration log at `7.quality/docs/phase1_migration_log.md`
4. Create an issue in the project tracker

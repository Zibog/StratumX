# Quality Suites Index

This directory contains all test suites for the StratumX game engine, organized by functional area and test type. The suite structure follows a matrix organization that maps to the repository's layered architecture.

## Suite Organization

Tests are organized into three primary categories:

### 1. Editor Canon Matrix (`editor_canon_matrix/`)

Tests for core editor subsystems that implement canonical editor functionality.

**Subsystems:**

- **command_spine/** - Tests for the L7.0 editor command routing and dispatch system
  - `unit/` - Unit tests for command registration, routing, and execution
  - `integration/` - Integration tests for command flow through the spine
  - `property/` - Property-based tests for command invariants

- **editor_shell/** - Tests for the L8.0 editor runtime shell
  - `unit/` - Unit tests for shell lifecycle and coordination
  - `integration/` - Integration tests for shell runtime behavior
  - `property/` - Property-based tests for shell invariants

- **state_containers/** - Tests for editor state management
  - `unit/` - Unit tests for state container operations
  - `integration/` - Integration tests for state synchronization
  - `property/` - Property-based tests for state consistency

### 2. Editor App Matrix (`editor_app_matrix/`)

Tests for the editor application UI layer.

**Subsystems:**

- **desktop_app/** - Tests for the desktop application UI
  - `unit/` - Unit tests for panel components and UI logic
  - `integration/` - Integration tests for UI workflows
  - `property/` - Property-based tests for UI invariants

### 3. Tooling Canon Matrix (`tooling_canon_matrix/`)

Tests for tooling subsystems.

**Subsystems:**

- **tool_session/** - Tests for the L6.0 tooling session management system
  - `unit/` - Unit tests for session lifecycle and management
  - `integration/` - Integration tests for tool session workflows
  - `property/` - Property-based tests for session invariants

## Test Type Guidelines

### Unit Tests (`unit/`)

Unit tests verify individual functions, methods, and classes in isolation. They should:
- Test specific examples and edge cases
- Execute quickly (< 100ms per test)
- Have no external dependencies
- Focus on a single unit of functionality

### Integration Tests (`integration/`)

Integration tests verify interactions between multiple components. They should:
- Test realistic workflows and scenarios
- Verify component integration points
- May have external dependencies (file system, etc.)
- Execute within reasonable time (< 5s per test)

### Property Tests (`property/`)

Property-based tests verify universal properties that hold across all inputs. They should:
- Use proptest or similar framework
- Generate diverse test inputs automatically
- Verify invariants and correctness properties
- Reference design document properties in comments

## Running Tests

### Run all tests in a suite category:
```bash
cargo test -p editor_canon_matrix
cargo test -p editor_app_matrix
cargo test -p tooling_canon_matrix
```

### Run tests for a specific subsystem:
```bash
cargo test -p editor_canon_matrix --test command_spine
```

### Run only unit tests:
```bash
cargo test -p editor_canon_matrix unit
```

### Run only property tests:
```bash
cargo test -p editor_canon_matrix property
```

## Adding New Tests

When adding new tests to the quality suites:

1. **Determine the correct suite category** based on the subsystem being tested
2. **Choose the appropriate test type** (unit, integration, or property)
3. **Place the test file** in the corresponding subdirectory
4. **Follow naming conventions**:
   - Unit tests: `test_<feature>.rs`
   - Integration tests: `integration_<workflow>.rs`
   - Property tests: `property_<invariant>.rs`
5. **Reference requirements** in test comments when applicable

## Migration from Production Code

Tests migrated from production `src/` directories are automatically placed in the appropriate suite category based on:
- Source location (which subsystem the test came from)
- Test type (detected from test patterns and markers)

The migration process preserves:
- All test logic and assertions
- Property test regression files
- Test metadata and documentation

## Suite Maintenance

### Existing Suite Preservation

Tests already in `7.quality/suites/` are never modified or moved by the migration process. This ensures:
- Stable test organization
- No disruption to existing test workflows
- Explicit control over test placement

### Suite Structure

The suite directory structure is created automatically by the Quality Restructurer. All necessary subdirectories are created on demand when tests are migrated.

## Additional Suites

This directory also contains other specialized test suites:

- **engine_canon_matrix/** - Core engine functionality tests
- **sdk_canon_matrix/** - SDK functionality tests
- **engine_sdk_link_matrix/** - Engine-SDK integration tests
- **sdk_tooling_link_matrix/** - SDK-Tooling integration tests
- **end_to_end_matrix/** - End-to-end workflow tests
- **engine_perf_harness/** - Performance benchmarks
- **vertical_slice_quality_gates/** - Vertical slice validation tests
- **repo_hygiene/** - Repository quality and hygiene checks

See individual suite README files for details on these specialized suites.

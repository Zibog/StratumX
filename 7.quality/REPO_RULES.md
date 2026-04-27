# StratumX Repository Rules

## Architecture Hygiene Rules

### 1. File Size Limits
```
no production file > 200 LOC
```
- Maximum 200 lines per production Rust file
- Excludes tests, comments, and blank lines
- Enforced by: `no_file_over_200_lines.rs`

### 2. Test Location
```
no *.test.rs inside src/
```
- Inline tests allowed in src/ via `#[cfg(test)]`
- External test files must be in `tests/` directory
- Enforced by: `no_inline_tests_in_src.rs`, `no_tests_outside_quality.rs`

### 3. TODO Policy
```
no TODO in accepted production code
```
- `TODO`, `FIXME`, `XXX` not allowed in production code
- Must be tracked in issue tracker, not in code
- Enforced by: `no_placeholder_stubs_in_active_path.rs`

### 4. Allow Attributes
```
no #[allow(...)] without explicit waiver registry
```
- All `#[allow(...)]` attributes must be documented in waiver registry
- Enforced by: `no_allow_attrs_in_active_path.rs`

### 5. Business Logic Location
```
no business logic in 6.apps
```
- Application layer (6.apps) must only contain thin UI/entrypoint
- Business logic must be in layers 5-7
- Enforced by architecture review

### 6. State Ownership
```
no duplicated state owners
```
- Each domain state has exactly one owner
- No duplicate authority containers
- Enforced by: `no_missing_module_references.rs`

### 7. Cross-Layer Imports
```
no direct-cross-layer-import
```
- Cannot import from non-adjacent layers
- Must follow canonical path: L11 → L10 → ... → L0
- Enforced by: `direct_cross_layer_import_checker.rs` (TODO)

### 8. Debug Artifacts
```
no debug modules in release path
no demo/debug/proof names in release path
```
- Debug code must not ship in release
- Release builds must not contain test names
- Enforced by: `no_debug_modules_in_release_path.rs`, `no_demo_debug_proof_names_in_release_path.rs`

## Canonical Layers Reference

```
L11: UI Components
L10: UI State Management  
L9: Domain Authoring Suites (Terrain, Material, Audio, Sky)
L8: Editor Shell
L7: Command Spine
L6: Tooling (Executor, Planes, Services)
L5: SDK Bridge
L4: Engine Core
L3: ECS/World
L2: Storage
L1: Platform
L0: Runtime
```

## Waiver Registry

To add an `#[allow(...)]` waiver, add entry to `7.quality/suites/repo_hygiene/waiver_registry.md`:

```markdown
| Attribute | File | Reason | Valid Until |
|-----------|------|--------|-------------|
| dead_code | src/foo.rs | Temporary during development | 2025-01-01 |
```

## Running Hygiene Checks

```bash
# Run all hygiene checks
cargo run -p stratumx_quality_tasks -- verify

# Or via xtask
cargo xtask quality verify
```

## Exceptions

Some rules may have documented exceptions:
- Performance-critical code may exceed 200 LOC with justification
- Generated code is exempt from line count
- External bindings may have allow attributes

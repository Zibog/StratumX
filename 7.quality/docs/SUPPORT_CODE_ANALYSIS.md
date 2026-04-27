# Support Code Analysis

This document catalogs embedded support code found in test files that should be extracted to support modules.

## Summary

The support infrastructure is already well-established with:
- `stratumx_test_support` - Core test support crate
- `stratumx_repo_hygiene_support` - Repo hygiene support
- `stratumx_route_test_support` - Route test support  
- `stratumx_shell_test_support` - Shell test support

However, embedded support code still exists in test files that needs extraction.

## Embedded Support Code Found

### 1. Builders (Test Object Construction)

#### repo_hygiene suite
- **Location**: `suites/repo_hygiene/tests/test_migrator.rs`
  - `create_test_repo()` (line 14)
  - `create_file()` (line 19)
  
- **Location**: `suites/repo_hygiene/tests/test_migrator_properties.rs`
  - `create_test_repo()` (line 15)
  - `create_file()` (line 20)
  
- **Location**: `suites/repo_hygiene/tests/test_hygiene_checker.rs`
  - `create_test_repo()` (line 21)
  - `create_test_file()` (line 33)
  
- **Location**: `suites/repo_hygiene/tests/test_hygiene_checker_properties.rs`
  - `create_test_repo()` (line 16)
  - `create_test_file()` (line 24)
  
- **Location**: `suites/repo_hygiene/tests/test_discovery_scanner.rs`
  - `create_test_repo()` (line 18)
  - `create_file()` (line 23)
  
- **Location**: `suites/repo_hygiene/tests/test_discovery_scanner_properties.rs`
  - `create_test_repo()` (line 15)
  - `create_file()` (line 20)
  
- **Location**: `suites/repo_hygiene/tests/test_quality_restructurer.rs`
  - `create_test_repo()` (line 9)

#### editor_state_matrix suite
- **Location**: `suites/editor_state_matrix/tests/property_substate_lifecycle_coupling.rs`
  - `create_test_system()` (line 20)
  
- **Location**: `suites/editor_state_matrix/tests/property_cache_invalidation_propagation.rs`
  - `create_test_system()` (line 20)
  
- **Location**: `suites/editor_state_matrix/tests/property_state_graph_dependency_accuracy.rs`
  - `create_test_system()` (line 20)
  
- **Location**: `suites/editor_state_matrix/tests/property_single_state_ownership.rs`
  - `create_test_system()` (line 59)
  - `create_test_system_with_world()` (line 79)
  
- **Location**: `suites/editor_state_matrix/tests/ownership_validation_tests.rs`
  - `create_test_system()` (line 18)
  
- **Location**: `suites/editor_state_matrix/tests/integration_service_coordination.rs`
  - `create_test_editor_host()` (line 20)
  
- **Location**: `suites/editor_state_matrix/tests/property_ownership_validation_completeness.rs`
  - `create_test_system()` (line 20)
  
- **Location**: `suites/editor_state_matrix/tests/property_ownership_transfer_atomicity.rs`
  - `create_test_system()` (line 21)
  
- **Location**: `suites/editor_state_matrix/tests/editor_host_tests.rs`
  - `create_test_identities()` (line 15)
  - `create_test_editor_host()` (line 29)
  
- **Location**: `suites/editor_state_matrix/tests/property_editor_host_delegation.rs`
  - `create_editor_host_from_identities()` (line 22)
  
- **Location**: `suites/editor_state_matrix/tests/property_derived_state_dependency_invariant.rs`
  - `create_test_system()` (line 20)

#### editor_canon_matrix suite
- **Location**: `suites/editor_canon_matrix/tests/honest_material_world_runtime_path.rs`
  - `create_session_with_runtime()` (line 18)
  
- **Location**: `suites/editor_canon_matrix/tests/honest_full_proof_pack_integration.rs`
  - `create_session_with_runtime()` (line 20)

### 2. Fixtures (Predefined Test Data)

Most fixture data appears to be already in `stratumx_test_support` in files like:
- `chunk_fixture.rs`
- `file_fixture.rs`
- `id_and_clock_fixture.rs`
- `package_fixture.rs`
- `sky_fixture.rs`
- `temp_fs_fixture.rs`
- `terrain_fixture.rs`
- `world_fixture.rs`

**Status**: Fixtures appear to be well-extracted already.

### 3. Assertions (Custom Assertion Helpers)

**Status**: No custom assertion helpers found in test files. The inventory may have been conservative in its assessment.

### 4. Case Generators (Test Case Generation)

#### editor_state_matrix suite (Proptest strategies)
- **Location**: `suites/editor_state_matrix/tests/property_substate_lifecycle_coupling.rs`
  - `project_substate()` (line 42)
  - `workspace_substate()` (line 51)
  - `diagnostics_substate()` (line 58)
  - `world_substate()` (line 65)

**Status**: Proptest strategies are embedded in test files and could be extracted.

### 5. Common Modules

Found common modules that act as local support code:
- `suites/tooling_canon_matrix/tests/common/mod.rs` - Contains builder functions
- `suites/engine_sdk_link_matrix/tests/common.rs`
- `suites/sdk_tooling_link_matrix/tests/common.rs`

**Recommendation**: These common modules should be reviewed and their contents moved to appropriate global support modules in `stratumx_test_support`.

## Extraction Priority

### High Priority (Frequently duplicated)
1. `create_test_repo()` and `create_file()` - Found in 6+ files in repo_hygiene
2. `create_test_system()` - Found in 8+ files in editor_state_matrix
3. `create_test_editor_host()` - Found in 2+ files in editor_state_matrix
4. `common/mod.rs` in tooling_canon_matrix - Contains runtime builders

### Medium Priority
1. Proptest strategies in editor_state_matrix
2. `create_session_with_runtime()` in editor_canon_matrix
3. Other one-off builders

### Low Priority
1. Simple one-line helper functions that don't justify extraction overhead

## Next Steps

1. Extract high-priority builders to `stratumx_test_support`
2. Move common module contents to appropriate support modules
3. Update test files to import from support modules instead of local common
4. Verify all tests pass after extraction
5. Ensure support modules comply with file size law (<400 lines each)

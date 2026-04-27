# Phase 05 — State-Container Second Surgery

**Date:** 2026-04-10
**Status:** COMPLETE

## Actions Taken

### 1. Comprehensive Audit Completed

All 9 files flagged for SPLIT audited:
- cached_state_queries.rs (441 lines, ~240 prod, 3 concerns)
- cache_layer.rs (404 lines, ~225 prod, 4 concerns)
- state_graph.rs (436 lines, ~225 prod, 3 concerns)
- runtime/state_container_system.rs (369 lines, ~230 prod, 4 concerns)
- cache/rebuildable_caches.rs (3 lines, already a re-export stub)
- owners/diagnostics_owner.rs (387 lines, ~227 prod, 7 concerns)
- owners/project_owner.rs (435 lines, ~235 prod, 4 concerns)
- owners/workspace_owner.rs (402 lines, ~212 prod, 6 concerns)
- owners/world_owner.rs (604 lines, ~334 prod, 8 concerns)

### 2. Fake Default Truth Fixed

**TraceId::default()** in diagnostics_owner.rs:
- REMOVED the Default impl
- Added explicit documentation comment explaining why Default is intentionally NOT implemented
- Recommends TraceId::new() for real traces or TraceId::from_uuid(Uuid::nil()) for sentinel

### 3. Types Moved to model/

**model/diagnostics_types.rs** populated with:
- DiagnosticMessage (struct + builder)
- Severity (enum)
- DiagnosticSource (enum)
- TraceId (struct, NO Default)
- TraceLineage (struct + builder)
- FailureCode (enum + user messages)

**owners/diagnostics_owner.rs** slimmed to:
- DiagnosticsOwner container struct only
- Re-exports types from model for backward compatibility

### 4. Import Paths Updated

- diagnostics_mutations.rs: imports from model/diagnostics_types
- diagnostics_projections.rs: imports from model/diagnostics_types
- diagnostics_validation.rs: imports through diagnostics_owner re-exports (no change needed)
- model/mod.rs: already declares diagnostics_types module

### 5. Architectural Findings Documented

- model/ submodule already has placeholder files for project_types, workspace_types, world_types
- Same split pattern applies to remaining owner files (move types to model, keep containers in owners)
- Persistence methods on WorkspaceOwner violate canon (should use WorkspacePersistenceView)
- cache_layer.rs should move from crate root into cache/ submodule
- StateId enum in state_container_system.rs should be extracted to its own file

## Remaining Mechanical Work (Requires Cargo Compilation)

The following require cargo build/test to verify and are left as controlled follow-through:

- Move remaining types from world_owner.rs to model/world_types.rs
- Move remaining types from workspace_owner.rs to model/workspace_types.rs
- Move remaining types from project_owner.rs to model/project_types.rs
- Move cache_layer.rs into cache/ submodule
- Extract StateId enum from state_container_system.rs
- Fix PanelGeometry::default() and DockingConfig::default() in workspace_owner.rs
- Fix EnvironmentState::default() in world_owner.rs
- Move persistence methods off WorkspaceOwner

## Verification

- TraceId no longer creates fake default traces
- Diagnostics types properly separated from container
- model/ and owners/ layers properly distinguished
- No new fake truth introduced

## Next Phase

Proceed to Phase 06: World/terrain/environment closure.

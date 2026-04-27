# Phase 02 Foundation Closure

## Scope

Owner-level closure wave for foundational engine crates:

- `engine_core`
- `engine_identity`
- `engine_handle`
- `engine_storage_layout`
- `engine_storage_access`
- `engine_storage_mutation`
- `engine_ecs_registry`
- `engine_ecs_query`
- `engine_ecs`
- `engine_world_spatial`
- `engine_world_region`

## What changed

- Added crate-local `tests/` surfaces across the foundation cluster so invariants now live in owner crates instead of only in matrix suites.
- Promoted explicit owner helpers in `engine_world_spatial`:
  `CoordinateSpace`, `SpatialAddress`, deterministic chunk/region origin helpers, region-local rebasing, and halo enumeration.
- Promoted explicit owner helpers in `engine_world_region`:
  `DirtyChunkEntry`, `DirtyRegionSnapshot`, direct dirty-flag lookup, and versioned dirty snapshots.
- Added `Aabb3f` invariant enforcement in `engine_core` through `ensure_valid()` and `Invariant`.
- Added `entity_archetype()` in `engine_ecs` so assembled ECS can expose explicit archetype truth instead of only query results.
- Tightened `engine_storage_mutation::MutationBuffer` so coalescing happens only for idempotent-to-idempotent writes; non-idempotent writes no longer collapse into the same slot accidentally.

## Evidence

- `cargo test -p engine_world_region -p engine_world_spatial -p engine_ecs -p engine_ecs_query -p engine_ecs_registry -p engine_storage_mutation -p engine_storage_access -p engine_storage_layout -p engine_handle -p engine_identity -p engine_core -j 1`
- `cargo test -p engine_canon_matrix -j 1`
- `cargo run -p stratumx_quality_tasks -- inventory`
- `cargo run -p stratumx_quality_tasks -- verify --verbose`

## What now works

- Foundation crates now reject illegal owner states locally with deterministic tests for negative paths, round-trips, and boundary contracts.
- Spatial and region ownership is more explicit for downstream runtime/editor consumers because address, origin, rebase, halo, and dirty-snapshot helpers exist as typed surfaces.
- Mutation staging respects idempotence boundaries instead of silently coalescing non-idempotent writes.

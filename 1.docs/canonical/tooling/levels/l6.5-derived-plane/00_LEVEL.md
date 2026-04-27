# Derived Plane

## Role
`derived_plane` computes deterministic derived projections from snapshots and indices without owning source truth.

## Owns
- `derived_projection_id`
- `source_snapshot_set`
- `source_index_set`
- `projection_class`
- `freshness_epoch`

## Consumes
- `l6.3-snapshot-plane`
- `l6.4-index-plane`
- `l6.9-budget-runtime`

## Emits
- derived projections
- heatmaps, summaries, and prepared views for editor/runtime surfaces

## Never owns
- authoritative mutable state
- hidden caches beyond cache_plane
- editor shell ownership

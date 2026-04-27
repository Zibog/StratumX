# Invalidation

This contract belongs specifically to the derived plane level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `derived_plane`
- dependency shift in `l6.3-snapshot-plane` that changes `derived_projection_id` semantics
- dependency shift in `l6.4-index-plane` that changes `source_snapshot_set` semantics
- dependency shift in `l6.9-budget-runtime` that changes `source_index_set` semantics
- supersede, deny, or cancel affecting `derived_projection_id`
- budget pressure that invalidates disposable outputs of `derived_plane` while preserving authoritative rows

## Invalidation law
Invalidation in `derived_plane` must explicitly name stale records such as `derived_projection_id`, `source_snapshot_set`, `source_index_set`, `projection_class` instead of rebuilding an unnamed mirror.

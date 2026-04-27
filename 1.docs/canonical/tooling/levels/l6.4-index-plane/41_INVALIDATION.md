# Invalidation

This contract belongs specifically to the index plane level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `index_plane`
- dependency shift in `l6.3-snapshot-plane` that changes `index_id` semantics
- dependency shift in `l6.6-artifact-plane` that changes `source_snapshot_set` semantics
- dependency shift in `l6.8-cache-plane` that changes `indexed_keyspace` semantics
- supersede, deny, or cancel affecting `index_id`
- budget pressure that invalidates disposable outputs of `index_plane` while preserving authoritative rows

## Invalidation law
Invalidation in `index_plane` must explicitly name stale records such as `index_id`, `source_snapshot_set`, `indexed_keyspace`, `freshness_epoch` instead of rebuilding an unnamed mirror.

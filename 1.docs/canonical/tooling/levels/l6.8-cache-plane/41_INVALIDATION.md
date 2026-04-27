# Invalidation

This contract belongs specifically to the cache plane level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `cache_plane`
- dependency shift in `l6.3-snapshot-plane` that changes `cache_entry_id` semantics
- dependency shift in `l6.4-index-plane` that changes `cache_class` semantics
- dependency shift in `l6.5-derived-plane` that changes `source_digest` semantics
- dependency shift in `l6.9-budget-runtime` that changes `eviction_priority` semantics
- supersede, deny, or cancel affecting `cache_entry_id`
- budget pressure that invalidates disposable outputs of `cache_plane` while preserving authoritative rows

## Invalidation law
Invalidation in `cache_plane` must explicitly name stale records such as `cache_entry_id`, `cache_class`, `source_digest`, `eviction_priority` instead of rebuilding an unnamed mirror.

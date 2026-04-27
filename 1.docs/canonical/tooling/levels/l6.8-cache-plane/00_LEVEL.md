# Cache Plane

## Role
`cache_plane` holds disposable acceleration caches that may be dropped and rebuilt from authoritative sources.

## Owns
- `cache_entry_id`
- `cache_class`
- `source_digest`
- `eviction_priority`
- `cache_state`

## Consumes
- `l6.3-snapshot-plane`
- `l6.4-index-plane`
- `l6.5-derived-plane`
- `l6.9-budget-runtime`

## Emits
- rebuildable cache entries for search, preview, and diagnostics

## Never owns
- authority or transaction ownership
- artifact permanence
- editor-visible canonical selections or views

# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| cache_entry_id | CacheEntryId | stable identity for one disposable cache row | unique within one cache class |
| cache_class | CacheClass | closed cache class enum | must use declared enum |
| source_digest | SourceDigest | digest proving which source snapshot/index produced the row | must resolve to current source set |
| eviction_priority | EvictionPriority | drop order under pressure | finite enum only |
| cache_state | CacheState | warm/cold/evicted status | must never become source truth |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `cache_plane` without consulting a hidden mirror.

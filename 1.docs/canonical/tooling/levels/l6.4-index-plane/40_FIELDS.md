# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| index_id | IndexId | stable index instance identity | unique per scope and build |
| source_snapshot_set | SnapshotSetRef | snapshots indexed by this build | must resolve through snapshot_plane |
| indexed_keyspace | IndexedKeyspace | keys or paths materialized by the index | must be explicit and bounded |
| freshness_epoch | IndexFreshnessEpoch | epoch at which the index is current | must trail or match source snapshots |
| rebuild_reason | IndexRebuildReason | why the index changed | finite enum only |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `index_plane` without consulting a hidden mirror.

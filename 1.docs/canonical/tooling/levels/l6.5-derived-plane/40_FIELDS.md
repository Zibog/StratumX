# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| derived_projection_id | DerivedProjectionId | stable derived-view identity | unique per projection class |
| source_snapshot_set | SnapshotSetRef | snapshots used to compute the projection | must resolve through snapshot_plane |
| source_index_set | IndexSetRef | indices used during derivation | must resolve through index_plane |
| projection_class | ProjectionClass | closed derived projection class enum | must use declared enum |
| freshness_epoch | DerivedFreshnessEpoch | epoch of the derived result | must be explicit |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `derived_plane` without consulting a hidden mirror.

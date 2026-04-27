# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| snapshot_id | SnapshotId | stable snapshot identity | unique per plane publication |
| source_epoch | AuthorityEpoch | authority epoch captured by the snapshot | must resolve to committed epoch |
| snapshot_scope | SnapshotScope | surface or domain represented by the snapshot | closed enum only |
| source_transaction_id | TransactionId | transaction that caused the snapshot | must resolve through transaction_ledger |
| content_hash | SnapshotContentHash | hash of immutable snapshot body | stable after publish |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `snapshot_plane` without consulting a hidden mirror.

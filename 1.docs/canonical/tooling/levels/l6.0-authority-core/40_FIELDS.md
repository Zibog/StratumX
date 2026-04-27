# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| authority_epoch | AuthorityEpoch | ordered epoch for writer decisions | monotonic single-writer counter |
| writer_scope_id | WriterScopeId | writer that owns the current mutation window | one active writer per authority scope |
| mutation_ticket | MutationTicket | ticket attached to an accepted command | must be issued before transaction commit |
| deny_reason | DenyReasonCode | machine-readable reject code for refused mutation | finite enum only |
| authority_snapshot_ref | AuthoritySnapshotRef | immutable authority snapshot for readers | published only after accepted changes |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `authority_core` without consulting a hidden mirror.

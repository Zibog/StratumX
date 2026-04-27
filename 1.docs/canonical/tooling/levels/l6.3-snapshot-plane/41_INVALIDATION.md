# Invalidation

This contract belongs specifically to the snapshot plane level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `snapshot_plane`
- dependency shift in `l6.0-authority-core` that changes `snapshot_id` semantics
- dependency shift in `l6.2-transaction-ledger` that changes `source_epoch` semantics
- supersede, deny, or cancel affecting `snapshot_id`
- budget pressure that invalidates disposable outputs of `snapshot_plane` while preserving authoritative rows

## Invalidation law
Invalidation in `snapshot_plane` must explicitly name stale records such as `snapshot_id`, `source_epoch`, `snapshot_scope`, `source_transaction_id` instead of rebuilding an unnamed mirror.

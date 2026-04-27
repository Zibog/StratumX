# Authority Core

## Role
`authority_core` owns the minimal single-writer authority cursor, mutation legality, and deny reasons for tooling-side state changes.

## Owns
- `authority_epoch`
- `writer_scope_id`
- `mutation_ticket`
- `deny_reason`
- `authority_snapshot_ref`

## Consumes
- `l6.2-transaction-ledger`
- `l6.9-budget-runtime`

## Emits
- command accept/deny decisions
- authority snapshots for snapshot_plane
- writer-pressure deny records

## Never owns
- editor widget/layout state
- build or release queue ownership
- assistant planning truth

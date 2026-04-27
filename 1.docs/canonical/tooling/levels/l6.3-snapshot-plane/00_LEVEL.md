# Snapshot Plane

## Role
`snapshot_plane` publishes immutable tooling snapshots that readers use instead of peeking into authority-owned mutable state.

## Owns
- `snapshot_id`
- `source_epoch`
- `snapshot_scope`
- `source_transaction_id`
- `content_hash`

## Consumes
- `l6.0-authority-core`
- `l6.2-transaction-ledger`

## Emits
- immutable snapshots for index/derived/preview/runtime consumers

## Never owns
- mutable command queues
- hidden cache mirrors
- editor-owned live widgets

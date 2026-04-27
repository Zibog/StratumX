# Transaction Ledger

## Role
`transaction_ledger` records ordered accepted mutations, their before/after cursors, and rollback-grade audit entries.

## Owns
- `transaction_id`
- `command_envelope_id`
- `before_authority_epoch`
- `after_authority_epoch`
- `transaction_outcome`

## Consumes
- `l6.0-authority-core`
- `l6.1-command-envelopes`

## Emits
- transaction records
- rollback/revert inputs
- snapshot invalidation triggers

## Never owns
- snapshot payload ownership
- editor-visible widget history
- preview-only speculative results

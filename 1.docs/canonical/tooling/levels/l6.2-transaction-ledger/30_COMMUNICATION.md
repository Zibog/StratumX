# Communication

This communication contract belongs specifically to the transaction ledger tooling level.

## Sends or publishes
- transaction records
- rollback/revert inputs
- snapshot invalidation triggers

## Required posture
- sender and receiver scopes are explicit for transaction ledger work
- published records such as transaction_id, command_envelope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for transaction ledger may not be copied to another level without changing operational meaning.

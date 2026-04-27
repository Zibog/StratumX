# Test Surfaces

This contract belongs specifically to the transaction ledger level and may not be reused verbatim by another tooling level.


## Required verification for `transaction_ledger`
- invariant tests for `transaction_id`, `command_envelope_id`, `before_authority_epoch`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.1-command-envelopes`
- communication tests covering transaction records; rollback/revert inputs; snapshot invalidation triggers
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.2-transaction-ledger` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

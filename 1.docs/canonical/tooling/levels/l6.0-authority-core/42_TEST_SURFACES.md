# Test Surfaces

This contract belongs specifically to the authority core level and may not be reused verbatim by another tooling level.


## Required verification for `authority_core`
- invariant tests for `authority_epoch`, `writer_scope_id`, `mutation_ticket`
- dependency legality tests proving `{key}` resolves only `l6.2-transaction-ledger`, `l6.9-budget-runtime`
- communication tests covering command accept/deny decisions; authority snapshots for snapshot_plane; writer-pressure deny records
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.0-authority-core` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

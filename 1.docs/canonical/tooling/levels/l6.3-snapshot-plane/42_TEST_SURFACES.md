# Test Surfaces

This contract belongs specifically to the snapshot plane level and may not be reused verbatim by another tooling level.


## Required verification for `snapshot_plane`
- invariant tests for `snapshot_id`, `source_epoch`, `snapshot_scope`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.2-transaction-ledger`
- communication tests covering immutable snapshots for index/derived/preview/runtime consumers
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.3-snapshot-plane` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

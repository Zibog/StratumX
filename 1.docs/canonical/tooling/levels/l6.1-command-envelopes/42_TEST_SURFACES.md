# Test Surfaces

This contract belongs specifically to the command envelopes level and may not be reused verbatim by another tooling level.


## Required verification for `command_envelopes`
- invariant tests for `command_envelope_id`, `command_kind`, `target_ref_set`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.0-tool-session`, `l6.14-tool-task-requests`
- communication tests covering validated command envelopes; transaction-ledger materialization inputs; command reject records
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.1-command-envelopes` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

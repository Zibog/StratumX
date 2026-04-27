# Test Surfaces

This contract belongs specifically to the validation runtime level and may not be reused verbatim by another tooling level.


## Required verification for `validation_runtime`
- invariant tests for `validation_run_id`, `validation_scope`, `rule_set_id`
- dependency legality tests proving `{key}` resolves only `l6.3-snapshot-plane`, `l6.4-index-plane`, `l6.7-stream-plane`, `l6.9-budget-runtime`
- communication tests covering validation issue sets; validation status streams; gate decisions for build/release/editor surfaces
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.11-validation-runtime` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

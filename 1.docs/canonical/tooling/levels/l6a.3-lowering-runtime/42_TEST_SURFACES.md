# Test Surfaces

This contract belongs specifically to the lowering runtime level and may not be reused verbatim by another tooling level.


## Required verification for `lowering_runtime`
- invariant tests for `lowering_runtime_id`, `source_session_id`, `state`
- dependency legality tests proving `{key}` resolves only `l6a.0-assistant-sessions`, `l6.9-budget-runtime`, `l6.7-stream-plane`
- communication tests covering assistant runtime records; bounded status streams; lowering/apply/revert handoffs where legal
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6a.3-lowering-runtime` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

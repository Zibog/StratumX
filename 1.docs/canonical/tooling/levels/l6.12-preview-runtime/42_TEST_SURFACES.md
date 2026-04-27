# Test Surfaces

This contract belongs specifically to the preview runtime level and may not be reused verbatim by another tooling level.


## Required verification for `preview_runtime`
- invariant tests for `preview_run_id`, `preview_request_id`, `preview_kind`
- dependency legality tests proving `{key}` resolves only `l6.3-snapshot-plane`, `l6.4-index-plane`, `l6.5-derived-plane`, `l6.7-stream-plane`
- communication tests covering preview result refs; preview progress streams; cancel/supersede decisions under pressure
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.12-preview-runtime` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

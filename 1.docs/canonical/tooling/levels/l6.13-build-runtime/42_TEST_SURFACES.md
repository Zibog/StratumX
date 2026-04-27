# Test Surfaces

This contract belongs specifically to the build runtime level and may not be reused verbatim by another tooling level.


## Required verification for `build_runtime`
- invariant tests for `build_job_id`, `build_target_set`, `input_digest`
- dependency legality tests proving `{key}` resolves only `l6.6-artifact-plane`, `l6.7-stream-plane`, `l6.9-budget-runtime`, `l6.11-validation-runtime`
- communication tests covering built artifacts; build progress streams; deterministic manifests for release handoff
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.13-build-runtime` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

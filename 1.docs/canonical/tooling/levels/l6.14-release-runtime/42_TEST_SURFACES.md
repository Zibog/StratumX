# Test Surfaces

This contract belongs specifically to the release runtime level and may not be reused verbatim by another tooling level.


## Required verification for `release_runtime`
- invariant tests for `release_run_id`, `release_manifest_id`, `input_artifact_set`
- dependency legality tests proving `{key}` resolves only `l6.6-artifact-plane`, `l6.7-stream-plane`, `l6.9-budget-runtime`, `l6.13-build-runtime`
- communication tests covering release manifests; publication status streams; package/signing handoff records
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.14-release-runtime` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

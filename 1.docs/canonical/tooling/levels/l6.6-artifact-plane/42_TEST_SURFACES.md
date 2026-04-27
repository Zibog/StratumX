# Test Surfaces

This contract belongs specifically to the artifact plane level and may not be reused verbatim by another tooling level.


## Required verification for `artifact_plane`
- invariant tests for `artifact_id`, `artifact_kind`, `source_digest`
- dependency legality tests proving `{key}` resolves only `l6.3-snapshot-plane`, `l6.13-build-runtime`, `l6.14-release-runtime`
- communication tests covering artifact manifests; reverse references to generated outputs; deterministic output lookup surfaces
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.6-artifact-plane` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

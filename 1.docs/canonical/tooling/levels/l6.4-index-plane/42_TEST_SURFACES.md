# Test Surfaces

This contract belongs specifically to the index plane level and may not be reused verbatim by another tooling level.


## Required verification for `index_plane`
- invariant tests for `index_id`, `source_snapshot_set`, `indexed_keyspace`
- dependency legality tests proving `{key}` resolves only `l6.3-snapshot-plane`, `l6.6-artifact-plane`, `l6.8-cache-plane`
- communication tests covering lookup indices; reverse-reference indices; search and dependency query surfaces
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.4-index-plane` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

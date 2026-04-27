# Test Surfaces

This contract belongs specifically to the canon reasoner level and may not be reused verbatim by another tooling level.


## Required verification for `canon_reasoner`
- invariant tests for `canon_reasoner_record_id`, `scope_id`, `state`
- dependency legality tests proving `{key}` resolves only `l6.3-snapshot-plane`, `l6.4-index-plane`, `l6.5-derived-plane`, `l6.6-artifact-plane`
- communication tests covering cold planning/meta records; reports or routing decisions; bounded orchestration summaries
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l7a.2-canon-reasoner` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

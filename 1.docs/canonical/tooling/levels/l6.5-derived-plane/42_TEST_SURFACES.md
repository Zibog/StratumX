# Test Surfaces

This contract belongs specifically to the derived plane level and may not be reused verbatim by another tooling level.


## Required verification for `derived_plane`
- invariant tests for `derived_projection_id`, `source_snapshot_set`, `source_index_set`
- dependency legality tests proving `{key}` resolves only `l6.3-snapshot-plane`, `l6.4-index-plane`, `l6.9-budget-runtime`
- communication tests covering derived projections; heatmaps, summaries, and prepared views for editor/runtime surfaces
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.5-derived-plane` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

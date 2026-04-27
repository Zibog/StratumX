# Test Surfaces

This contract belongs specifically to the workspace runtime level and may not be reused verbatim by another tooling level.


## Required verification for `workspace_runtime`
- invariant tests for `workspace_session_id`, `published_selection_ref_set`, `published_focus_ref_set`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.3-snapshot-plane`, `l6.7-stream-plane`, `l6.9-budget-runtime`
- communication tests covering public coordination refs; runtime-attach publications; selection/focus/panel/view ref snapshots
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.10-workspace-runtime` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

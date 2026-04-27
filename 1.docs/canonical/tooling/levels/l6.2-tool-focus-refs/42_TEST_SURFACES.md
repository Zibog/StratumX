# Test Surfaces

This contract belongs specifically to the tool focus refs level and may not be reused verbatim by another tooling level.


## Required verification for `tool_focus_refs`
- invariant tests for `focus_ref_id`, `focused_target_ref`, `focus_owner_surface`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.0-tool-session`
- communication tests covering published `tool_focus_refs` rows for downstream services; bounded status or routing updates where applicable
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.2-tool-focus-refs` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

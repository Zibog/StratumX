# Test Surfaces

This contract belongs specifically to the tool panel refs level and may not be reused verbatim by another tooling level.


## Required verification for `tool_panel_refs`
- invariant tests for `panel_ref_id`, `panel_kind`, `attachment_scope`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.0-tool-session`
- communication tests covering published `tool_panel_refs` rows for downstream services; bounded status or routing updates where applicable
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.16-tool-panel-refs` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

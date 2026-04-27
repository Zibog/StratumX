# Test Surfaces

This contract belongs specifically to the tool diagnostics views level and may not be reused verbatim by another tooling level.


## Required verification for `tool_diagnostics_views`
- invariant tests for `diagnostic_view_id`, `issue_set_ref`, `grouping_mode`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.0-tool-session`
- communication tests covering published `tool_diagnostics_views` rows for downstream services; bounded status or routing updates where applicable
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.7-tool-diagnostics-views` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

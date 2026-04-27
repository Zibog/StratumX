# Test Surfaces

This contract belongs specifically to the tool task results level and may not be reused verbatim by another tooling level.


## Required verification for `tool_task_results`
- invariant tests for `task_result_id`, `task_request_id`, `result_state`
- dependency legality tests proving `{key}` resolves only `l6.0-authority-core`, `l6.0-tool-session`
- communication tests covering published `tool_task_results` rows for downstream services; bounded status or routing updates where applicable
- concurrency tests for the writer/reader posture of `{key}`

## Operational note
This file remains active and package-specific for `l6.15-tool-task-results` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

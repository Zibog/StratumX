# Budget Envelope

This contract belongs specifically to the tool task results level and may not be reused verbatim by another tooling level.


## Budget posture for `tool_task_results`
- correctness of `task_result_id`, `task_request_id` is non-degradable
- lower-priority outputs such as published `tool_task_results` rows for downstream services; bounded status or routing updates where applicable degrade before correctness
- any drop or defer decision must be visible through `{key}` publications, never hidden in an unnamed cache

## Operational note
This file remains active and package-specific for `l6.15-tool-task-results` / `43_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 43 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

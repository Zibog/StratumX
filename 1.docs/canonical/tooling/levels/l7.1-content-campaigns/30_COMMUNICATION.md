# Communication

This communication contract belongs specifically to the content campaigns tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for content campaigns work
- published records such as content_campaigns_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for content campaigns may not be copied to another level without changing operational meaning.

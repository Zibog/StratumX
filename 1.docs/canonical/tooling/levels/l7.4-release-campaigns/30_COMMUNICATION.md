# Communication

This communication contract belongs specifically to the release campaigns tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for release campaigns work
- published records such as release_campaigns_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for release campaigns may not be copied to another level without changing operational meaning.

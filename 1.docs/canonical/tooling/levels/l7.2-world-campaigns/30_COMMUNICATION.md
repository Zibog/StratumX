# Communication

This communication contract belongs specifically to the world campaigns tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for world campaigns work
- published records such as world_campaigns_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for world campaigns may not be copied to another level without changing operational meaning.

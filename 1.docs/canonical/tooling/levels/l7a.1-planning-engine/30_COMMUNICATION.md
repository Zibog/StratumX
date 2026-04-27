# Communication

This communication contract belongs specifically to the planning engine tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for planning engine work
- published records such as planning_engine_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for planning engine may not be copied to another level without changing operational meaning.

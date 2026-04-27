# Communication

This communication contract belongs specifically to the generation planner tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for generation planner work
- published records such as generation_planner_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for generation planner may not be copied to another level without changing operational meaning.

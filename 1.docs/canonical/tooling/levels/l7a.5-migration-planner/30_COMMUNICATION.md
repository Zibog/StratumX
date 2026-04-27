# Communication

This communication contract belongs specifically to the migration planner tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for migration planner work
- published records such as migration_planner_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for migration planner may not be copied to another level without changing operational meaning.

# Communication

This communication contract belongs specifically to the simulation campaigns tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for simulation campaigns work
- published records such as simulation_campaigns_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for simulation campaigns may not be copied to another level without changing operational meaning.

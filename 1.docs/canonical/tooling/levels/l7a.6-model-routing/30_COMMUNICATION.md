# Communication

This communication contract belongs specifically to the model routing tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for model routing work
- published records such as model_routing_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for model routing may not be copied to another level without changing operational meaning.

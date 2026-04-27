# Communication

This communication contract belongs specifically to the prompt understanding tooling level.

## Sends or publishes
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Required posture
- sender and receiver scopes are explicit for prompt understanding work
- published records such as prompt_understanding_record_id, scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for prompt understanding may not be copied to another level without changing operational meaning.

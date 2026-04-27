# Communication

This communication contract belongs specifically to the preview runtime tooling level.

## Sends or publishes
- preview result refs
- preview progress streams
- cancel/supersede decisions under pressure

## Required posture
- sender and receiver scopes are explicit for preview runtime work
- published records such as preview_run_id, preview_request_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for preview runtime may not be copied to another level without changing operational meaning.

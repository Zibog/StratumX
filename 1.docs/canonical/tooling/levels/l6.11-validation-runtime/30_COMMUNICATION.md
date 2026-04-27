# Communication

This communication contract belongs specifically to the validation runtime tooling level.

## Sends or publishes
- validation issue sets
- validation status streams
- gate decisions for build/release/editor surfaces

## Required posture
- sender and receiver scopes are explicit for validation runtime work
- published records such as validation_run_id, validation_scope remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for validation runtime may not be copied to another level without changing operational meaning.

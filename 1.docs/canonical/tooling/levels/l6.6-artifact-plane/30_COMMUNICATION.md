# Communication

This communication contract belongs specifically to the artifact plane tooling level.

## Sends or publishes
- artifact manifests
- reverse references to generated outputs
- deterministic output lookup surfaces

## Required posture
- sender and receiver scopes are explicit for artifact plane work
- published records such as artifact_id, artifact_kind remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for artifact plane may not be copied to another level without changing operational meaning.

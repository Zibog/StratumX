# Communication

This communication contract belongs specifically to the build runtime tooling level.

## Sends or publishes
- built artifacts
- build progress streams
- deterministic manifests for release handoff

## Required posture
- sender and receiver scopes are explicit for build runtime work
- published records such as build_job_id, build_target_set remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for build runtime may not be copied to another level without changing operational meaning.

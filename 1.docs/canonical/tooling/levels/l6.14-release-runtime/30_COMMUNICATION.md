# Communication

This communication contract belongs specifically to the release runtime tooling level.

## Sends or publishes
- release manifests
- publication status streams
- package/signing handoff records

## Required posture
- sender and receiver scopes are explicit for release runtime work
- published records such as release_run_id, release_manifest_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for release runtime may not be copied to another level without changing operational meaning.

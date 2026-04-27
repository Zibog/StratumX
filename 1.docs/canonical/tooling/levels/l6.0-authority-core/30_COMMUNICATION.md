# Communication

This communication contract belongs specifically to the authority core tooling level.

## Sends or publishes
- command accept/deny decisions
- authority snapshots for snapshot_plane
- writer-pressure deny records

## Required posture
- sender and receiver scopes are explicit for authority core work
- published records such as authority_epoch, writer_scope_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for authority core may not be copied to another level without changing operational meaning.

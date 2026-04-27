# Communication

This communication contract belongs specifically to the snapshot plane tooling level.

## Sends or publishes
- immutable snapshots for index/derived/preview/runtime consumers
- sender and receiver scopes are explicit
- mutating flows remain authority or transaction visible

## Required posture
- sender and receiver scopes are explicit for snapshot plane work
- published records such as snapshot_id, source_epoch remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for snapshot plane may not be copied to another level without changing operational meaning.

# Communication

This communication contract belongs specifically to the index plane tooling level.

## Sends or publishes
- lookup indices
- reverse-reference indices
- search and dependency query surfaces

## Required posture
- sender and receiver scopes are explicit for index plane work
- published records such as index_id, source_snapshot_set remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for index plane may not be copied to another level without changing operational meaning.

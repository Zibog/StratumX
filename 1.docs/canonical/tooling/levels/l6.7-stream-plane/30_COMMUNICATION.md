# Communication

This communication contract belongs specifically to the stream plane tooling level.

## Sends or publishes
- diagnostic streams
- preview status streams
- build and release progress streams

## Required posture
- sender and receiver scopes are explicit for stream plane work
- published records such as stream_event_id, stream_kind remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for stream plane may not be copied to another level without changing operational meaning.

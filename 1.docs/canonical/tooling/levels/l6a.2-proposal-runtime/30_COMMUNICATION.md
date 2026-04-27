# Communication

This communication contract belongs specifically to the proposal runtime tooling level.

## Sends or publishes
- assistant runtime records
- bounded status streams
- lowering/apply/revert handoffs where legal

## Required posture
- sender and receiver scopes are explicit for proposal runtime work
- published records such as proposal_runtime_id, source_session_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for proposal runtime may not be copied to another level without changing operational meaning.

# Communication

This communication contract belongs specifically to the safety gates tooling level.

## Sends or publishes
- assistant runtime records
- bounded status streams
- lowering/apply/revert handoffs where legal

## Required posture
- sender and receiver scopes are explicit for safety gates work
- published records such as safety_gate_id, source_session_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for safety gates may not be copied to another level without changing operational meaning.

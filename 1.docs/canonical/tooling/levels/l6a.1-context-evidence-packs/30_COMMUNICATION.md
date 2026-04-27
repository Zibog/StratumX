# Communication

This communication contract belongs specifically to the context evidence packs tooling level.

## Sends or publishes
- assistant runtime records
- bounded status streams
- lowering/apply/revert handoffs where legal

## Required posture
- sender and receiver scopes are explicit for context evidence packs work
- published records such as context_evidence_pack_id, source_session_id remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for context evidence packs may not be copied to another level without changing operational meaning.

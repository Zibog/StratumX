# Communication

This communication contract belongs specifically to the workspace runtime tooling level.

## Sends or publishes
- public coordination refs
- runtime-attach publications
- selection/focus/panel/view ref snapshots

## Required posture
- sender and receiver scopes are explicit for workspace runtime work
- published records such as workspace_session_id, published_selection_ref_set remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for workspace runtime may not be copied to another level without changing operational meaning.

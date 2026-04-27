# Communication

This communication contract belongs specifically to the derived plane tooling level.

## Sends or publishes
- derived projections
- heatmaps, summaries, and prepared views for editor/runtime surfaces
- sender and receiver scopes are explicit

## Required posture
- sender and receiver scopes are explicit for derived plane work
- published records such as derived_projection_id, source_snapshot_set remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for derived plane may not be copied to another level without changing operational meaning.

# Composition

## Member composition
- workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing
- authority-facing minimal truth: workspace session ownership only
- snapshot classes: workspace snapshots and session-scoped view snapshots
- index classes: workspace/session lookup indices
- derived classes: derived workspace panels and inspector views

## Composition rule
The family exists to make domain adjacency for `workspace_shell_family` explicit, not to merge members into one truth owner.

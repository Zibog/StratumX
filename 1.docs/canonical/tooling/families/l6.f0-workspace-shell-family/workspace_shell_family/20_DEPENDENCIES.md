# Family Local Dependencies

This local family contract belongs specifically to the l6.f0 workspace shell family family and may not be reused by a different family key.


## Allowed for `workspace_shell_family`
- member-local coordination for workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing
- member-local coordination for authority-facing minimal truth: workspace session ownership only
- member-local coordination for snapshot classes: workspace snapshots and session-scoped view snapshots
- member-local coordination for index classes: workspace/session lookup indices
- member-local coordination for derived classes: derived workspace panels and inspector views
- package-root family registry and shared ids
- lower packages only through member-legal surfaces

## Forbidden
- undeclared member truth
- unrelated domain truth

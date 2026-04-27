# Dependencies

This contract belongs specifically to the l6.f0 workspace shell family and describes family-only coordination.


## Family dependency posture for `workspace_shell_family`
- family composition may touch workspace shell composition, layout sessions, focus/selection presentation, and workspace host routing
- family composition may touch authority-facing minimal truth: workspace session ownership only
- family composition may touch snapshot classes: workspace snapshots and session-scoped view snapshots
- family composition may touch index classes: workspace/session lookup indices
- family composition may touch derived classes: derived workspace panels and inspector views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

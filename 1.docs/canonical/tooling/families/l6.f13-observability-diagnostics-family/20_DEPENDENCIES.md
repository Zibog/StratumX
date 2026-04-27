# Dependencies

This contract belongs specifically to the l6.f13 observability diagnostics family family and describes family-only coordination.


## Family dependency posture for `observability_diagnostics_family`
- family composition may touch observability views, logs, probes, metrics dashboards, validation visibility, and diagnostics routing
- family composition may touch authority-facing minimal truth: observability-session refs only
- family composition may touch snapshot classes: observability snapshots
- family composition may touch index classes: diagnostics lookup indices
- family composition may touch derived classes: dashboards, summaries, and remediation views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

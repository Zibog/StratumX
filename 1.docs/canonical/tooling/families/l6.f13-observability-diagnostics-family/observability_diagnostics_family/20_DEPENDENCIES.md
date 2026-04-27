# Family Local Dependencies

This local family contract belongs specifically to the l6.f13 observability diagnostics family family and may not be reused by a different family key.


## Allowed for `observability_diagnostics_family`
- member-local coordination for observability views, logs, probes, metrics dashboards, validation visibility, and diagnostics routing
- member-local coordination for authority-facing minimal truth: observability-session refs only
- member-local coordination for snapshot classes: observability snapshots
- member-local coordination for index classes: diagnostics lookup indices
- member-local coordination for derived classes: dashboards, summaries, and remediation views
- package-root family registry and shared ids
- lower packages only through member-legal surfaces

## Forbidden
- undeclared member truth
- unrelated domain truth

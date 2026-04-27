# Family Local Dependencies

This local family contract belongs specifically to the l6.f6 simulation debug family and may not be reused by a different family key.


## Allowed for `simulation_debug_family`
- member-local coordination for simulation debug views, probes, timeline views, and diagnostic overlays
- member-local coordination for authority-facing minimal truth: no domain authority beyond debug-session refs
- member-local coordination for snapshot classes: simulation debug snapshots
- member-local coordination for index classes: simulation debug indices
- member-local coordination for derived classes: derived simulation debug views
- package-root family registry and shared ids
- lower packages only through member-legal surfaces

## Forbidden
- undeclared member truth
- unrelated domain truth

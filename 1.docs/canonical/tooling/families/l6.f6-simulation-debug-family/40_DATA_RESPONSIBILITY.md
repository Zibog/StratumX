# Data Responsibility

This contract belongs specifically to the l6.f6 simulation debug family and describes family-only coordination.


## Family-scoped coordination for `simulation_debug_family`
- locality, activation, and diagnostics around simulation debug views, probes, timeline views, and diagnostic overlays
- locality, activation, and diagnostics around authority-facing minimal truth: no domain authority beyond debug-session refs
- locality, activation, and diagnostics around snapshot classes: simulation debug snapshots
- locality, activation, and diagnostics around index classes: simulation debug indices
- locality, activation, and diagnostics around derived classes: derived simulation debug views

## Responsibility note
These coordination classes describe family behavior and do not replace member truth ownership.

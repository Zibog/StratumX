# Communication

This contract belongs specifically to the l6.f6 simulation debug family and describes family-only coordination.


## Family-local communication for `simulation_debug_family`
- coordination and refresh propagation related to simulation debug views, probes, timeline views, and diagnostic overlays
- coordination and refresh propagation related to authority-facing minimal truth: no domain authority beyond debug-session refs
- coordination and refresh propagation related to snapshot classes: simulation debug snapshots
- coordination and refresh propagation related to index classes: simulation debug indices
- coordination and refresh propagation related to derived classes: derived simulation debug views

## Note
The family coordinates these member concerns without introducing a new authority path.

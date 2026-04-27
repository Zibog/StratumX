# Dependencies

This contract belongs specifically to the l6.f6 simulation debug family and describes family-only coordination.


## Family dependency posture for `simulation_debug_family`
- family composition may touch simulation debug views, probes, timeline views, and diagnostic overlays
- family composition may touch authority-facing minimal truth: no domain authority beyond debug-session refs
- family composition may touch snapshot classes: simulation debug snapshots
- family composition may touch index classes: simulation debug indices
- family composition may touch derived classes: derived simulation debug views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

# Dependencies

This contract belongs specifically to the l7.f3 simulation meta family and describes family-only coordination.


## Family dependency posture for `simulation_meta_family`
- family composition may touch simulation campaign composition and simulation workflow meta
- family composition may touch authority-facing minimal truth: simulation campaign refs only
- family composition may touch snapshot classes: simulation-campaign snapshots
- family composition may touch index classes: simulation-campaign indices
- family composition may touch derived classes: derived simulation campaign views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

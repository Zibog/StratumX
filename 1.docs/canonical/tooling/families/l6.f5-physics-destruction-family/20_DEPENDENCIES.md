# Dependencies

This contract belongs specifically to the l6.f5 physics destruction family and describes family-only coordination.


## Family dependency posture for `physics_destruction_family`
- family composition may touch physics/destruction authoring, structural constraints, and destruction diagnostics
- family composition may touch authority-facing minimal truth: physics/destruction edit intents
- family composition may touch snapshot classes: physics snapshots
- family composition may touch index classes: physics lookup indices
- family composition may touch derived classes: derived physics views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

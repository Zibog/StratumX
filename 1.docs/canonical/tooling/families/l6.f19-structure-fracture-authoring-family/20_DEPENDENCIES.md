# Dependencies

This contract belongs specifically to the l6.f4 structure fracture authoring family and describes family-only coordination.


## Family dependency posture for `structure_fracture_authoring_family`
- family composition may touch fracture authoring, fracture rule sets, and fracture preview routing
- family composition may touch authority-facing minimal truth: fracture edit intents
- family composition may touch snapshot classes: fracture snapshots
- family composition may touch index classes: fracture lookup indices
- family composition may touch derived classes: derived fracture views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

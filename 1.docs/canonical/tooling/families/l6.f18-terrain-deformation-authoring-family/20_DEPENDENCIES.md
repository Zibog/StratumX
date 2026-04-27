# Dependencies

This contract belongs specifically to the l6.f3 terrain deformation authoring family and describes family-only coordination.


## Family dependency posture for `terrain_deformation_authoring_family`
- family composition may touch terrain layers, deformation intents, brush results, and terrain manifests
- family composition may touch authority-facing minimal truth: terrain edit intents only
- family composition may touch snapshot classes: terrain snapshots
- family composition may touch index classes: terrain indices
- family composition may touch derived classes: terrain-derived overlays
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

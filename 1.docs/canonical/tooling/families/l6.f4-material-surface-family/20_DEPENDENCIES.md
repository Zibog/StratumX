# Dependencies

This contract belongs specifically to the l6.f4 material surface family and describes family-only coordination.


## Family dependency posture for `material_surface_family`
- family composition may touch surface authoring, material layering, and surface instance views
- family composition may touch authority-facing minimal truth: surface edit intents
- family composition may touch snapshot classes: surface snapshots
- family composition may touch index classes: surface lookup indices
- family composition may touch derived classes: derived surface views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

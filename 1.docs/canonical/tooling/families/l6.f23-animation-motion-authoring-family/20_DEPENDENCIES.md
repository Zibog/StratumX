# Dependencies

This contract belongs specifically to the l6.f8 animation motion authoring family and describes family-only coordination.


## Family dependency posture for `animation_motion_authoring_family`
- family composition may touch motion authoring, clip graphs, motion rules, and motion previews
- family composition may touch authority-facing minimal truth: motion edit intents
- family composition may touch snapshot classes: motion snapshots
- family composition may touch index classes: motion lookup indices
- family composition may touch derived classes: derived motion views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

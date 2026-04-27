# Dependencies

This contract belongs specifically to the l6.f10 render lookdev authoring family and describes family-only coordination.


## Family dependency posture for `render_lookdev_authoring_family`
- family composition may touch render lookdev authoring, lighting/look rules, and render previews
- family composition may touch authority-facing minimal truth: lookdev edit intents
- family composition may touch snapshot classes: lookdev snapshots
- family composition may touch index classes: lookdev lookup indices
- family composition may touch derived classes: derived lookdev views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

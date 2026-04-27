# Dependencies

This contract belongs specifically to the l6.f3 world family and describes family-only coordination.


## Family dependency posture for `world_family`
- family composition may touch world composition, world-part selection, data layers, and world-state views
- family composition may touch authority-facing minimal truth: world edit intents and active world refs
- family composition may touch snapshot classes: world snapshots
- family composition may touch index classes: world lookup indices
- family composition may touch derived classes: derived world views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

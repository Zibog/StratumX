# Dependencies

This contract belongs specifically to the l7.f2 world meta family and describes family-only coordination.


## Family dependency posture for `world_meta_family`
- family composition may touch world campaign composition and world workflow meta
- family composition may touch authority-facing minimal truth: world campaign refs only
- family composition may touch snapshot classes: world-campaign snapshots
- family composition may touch index classes: world-campaign indices
- family composition may touch derived classes: derived world campaign views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

# Dependencies

This contract belongs specifically to the l6.f7 animation rig family and describes family-only coordination.


## Family dependency posture for `animation_rig_family`
- family composition may touch rig authoring, rig refs, rig validation, and rig manifests
- family composition may touch authority-facing minimal truth: rig edit intents
- family composition may touch snapshot classes: rig snapshots
- family composition may touch index classes: rig lookup indices
- family composition may touch derived classes: derived rig views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

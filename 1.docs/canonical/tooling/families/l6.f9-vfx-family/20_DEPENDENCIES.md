# Dependencies

This contract belongs specifically to the l6.f9 vfx family and describes family-only coordination.


## Family dependency posture for `vfx_family`
- family composition may touch VFX authoring, effect graphs, effect views, and VFX manifests
- family composition may touch authority-facing minimal truth: VFX edit intents
- family composition may touch snapshot classes: VFX snapshots
- family composition may touch index classes: VFX lookup indices
- family composition may touch derived classes: derived VFX views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

# Dependencies

This contract belongs specifically to the l6.f8 cinematic family and describes family-only coordination.


## Family dependency posture for `cinematic_family`
- family composition may touch timeline authoring, shot views, track bindings, event markers, camera rigs, and cinematic manifests
- family composition may touch authority-facing minimal truth: cinematic edit intents and binding refs
- family composition may touch snapshot classes: cinematic snapshots
- family composition may touch index classes: shot, track, and binding lookup indices
- family composition may touch derived classes: cinematic timelines, shot lists, and blend graphs
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

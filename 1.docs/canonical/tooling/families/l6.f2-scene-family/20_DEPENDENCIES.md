# Dependencies

This contract belongs specifically to the l6.f2 scene family and describes family-only coordination.


## Family dependency posture for `scene_family`
- family composition may touch scene graph authoring, entity grouping, scene views, and scene manifests
- family composition may touch authority-facing minimal truth: scene edit intents and scene root refs
- family composition may touch snapshot classes: scene snapshots
- family composition may touch index classes: scene/spatial lookup indices
- family composition may touch derived classes: derived scene views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

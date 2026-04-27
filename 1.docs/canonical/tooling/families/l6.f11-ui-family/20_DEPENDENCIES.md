# Dependencies

This contract belongs specifically to the l6.f11 ui family and describes family-only coordination.


## Family dependency posture for `ui_family`
- family composition may touch UI authoring, widget surfaces, UI schemas, and UI manifests
- family composition may touch authority-facing minimal truth: UI edit intents
- family composition may touch snapshot classes: UI snapshots
- family composition may touch index classes: UI lookup indices
- family composition may touch derived classes: derived UI views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

# Dependencies

This contract belongs specifically to the l6.f10 audio family and describes family-only coordination.


## Family dependency posture for `audio_family`
- family composition may touch audio authoring, buses, routing views, and audio manifests
- family composition may touch authority-facing minimal truth: audio edit intents
- family composition may touch snapshot classes: audio snapshots
- family composition may touch index classes: audio lookup indices
- family composition may touch derived classes: derived audio views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

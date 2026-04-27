# Dependencies

This contract belongs specifically to the l6.f9 acoustics voice authoring family family and describes family-only coordination.


## Family dependency posture for `acoustics_voice_authoring_family`
- family composition may touch acoustics and voice authoring, emitters, and propagation authoring views
- family composition may touch authority-facing minimal truth: acoustics/voice edit intents
- family composition may touch snapshot classes: acoustics snapshots
- family composition may touch index classes: acoustics lookup indices
- family composition may touch derived classes: derived acoustics views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

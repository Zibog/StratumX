# Dependencies

This contract belongs specifically to the l6.f2 material response authoring family family and describes family-only coordination.


## Family dependency posture for `material_response_authoring_family`
- family composition may touch material response authoring, reaction tables, and response rule authoring
- family composition may touch authority-facing minimal truth: material-response edit intents
- family composition may touch snapshot classes: response snapshots
- family composition may touch index classes: response lookup indices
- family composition may touch derived classes: derived response views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

# Dependencies

This contract belongs specifically to the l6.f12 automation family and describes family-only coordination.


## Family dependency posture for `automation_family`
- family composition may touch automation editor views, automation specs, and automation diagnostics
- family composition may touch authority-facing minimal truth: automation edit intents
- family composition may touch snapshot classes: automation snapshots
- family composition may touch index classes: automation lookup indices
- family composition may touch derived classes: derived automation views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

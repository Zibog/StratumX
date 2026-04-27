# Dependencies

This contract belongs specifically to the l6a.f0 assistant experience family and describes family-only coordination.


## Family dependency posture for `assistant_experience_family`
- family composition may touch assistant sessions, evidence pack composition, proposal experience, and assistant UI composition
- family composition may touch authority-facing minimal truth: assistant session refs only
- family composition may touch snapshot classes: assistant proposal and evidence snapshots
- family composition may touch index classes: assistant lookup indices and proposal indices
- family composition may touch derived classes: derived assistant proposal views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

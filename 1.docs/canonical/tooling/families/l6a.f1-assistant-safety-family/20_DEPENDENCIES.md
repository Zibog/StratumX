# Dependencies

This contract belongs specifically to the l6a.f1 assistant safety family and describes family-only coordination.


## Family dependency posture for `assistant_safety_family`
- family composition may touch assistant safety, approval, legality, and apply/revert policy composition
- family composition may touch authority-facing minimal truth: safety/approval refs only
- family composition may touch snapshot classes: safety snapshots
- family composition may touch index classes: safety lookup indices
- family composition may touch derived classes: derived safety views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

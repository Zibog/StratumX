# Dependencies

This contract belongs specifically to the l6.f11 performance governance authoring family family and describes family-only coordination.


## Family dependency posture for `performance_governance_authoring_family`
- family composition may touch performance governance authoring, budget policies, and degradation policy authoring
- family composition may touch authority-facing minimal truth: budget/governance edit intents
- family composition may touch snapshot classes: budget/governance snapshots
- family composition may touch index classes: policy lookup indices
- family composition may touch derived classes: derived governance views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

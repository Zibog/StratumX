# Dependencies

This contract belongs specifically to the l7a.f0 assistant intelligence family and describes family-only coordination.


## Family dependency posture for `assistant_intelligence_family`
- family composition may touch goal understanding, planning, canon reasoning, optimization, migration, and routing composition
- family composition may touch authority-facing minimal truth: goal/plan refs only
- family composition may touch snapshot classes: planning snapshots
- family composition may touch index classes: planning lookup indices
- family composition may touch derived classes: derived plan and reasoning views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

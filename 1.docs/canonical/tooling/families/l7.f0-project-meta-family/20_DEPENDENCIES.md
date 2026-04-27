# Dependencies

This contract belongs specifically to the l7.f0 project meta family and describes family-only coordination.


## Family dependency posture for `project_meta_family`
- family composition may touch project orchestration, governance, and reporting composition
- family composition may touch authority-facing minimal truth: project meta refs only
- family composition may touch snapshot classes: project-orchestration snapshots
- family composition may touch index classes: project-orchestration lookup indices
- family composition may touch derived classes: derived project-meta views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

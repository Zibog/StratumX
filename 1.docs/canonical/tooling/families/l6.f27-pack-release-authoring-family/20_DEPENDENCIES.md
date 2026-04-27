# Dependencies

This contract belongs specifically to the l6.f12 pack release authoring family and describes family-only coordination.


## Family dependency posture for `pack_release_authoring_family`
- family composition may touch build targets, bake graphs, cook graphs, release manifests, package closure, and release diagnostics
- family composition may touch authority-facing minimal truth: target refs and explicit task intents only
- family composition may touch snapshot classes: build/release status snapshots
- family composition may touch index classes: target and manifest lookup indices
- family composition may touch derived classes: readiness summaries and closure dashboards
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

# Dependencies

This contract belongs specifically to the l7.f4 release meta family and describes family-only coordination.


## Family dependency posture for `release_meta_family`
- family composition may touch release campaign composition, release governance, and release reporting
- family composition may touch authority-facing minimal truth: release campaign refs only
- family composition may touch snapshot classes: release-campaign snapshots
- family composition may touch index classes: release-campaign indices
- family composition may touch derived classes: derived release campaign views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

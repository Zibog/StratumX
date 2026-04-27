# Dependencies

This contract belongs specifically to the l7.f1 content meta family and describes family-only coordination.


## Family dependency posture for `content_meta_family`
- family composition may touch content campaign composition and content workflow meta
- family composition may touch authority-facing minimal truth: content campaign refs only
- family composition may touch snapshot classes: content-campaign snapshots
- family composition may touch index classes: content-campaign indices
- family composition may touch derived classes: derived content campaign views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

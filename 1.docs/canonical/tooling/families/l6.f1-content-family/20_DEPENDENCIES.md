# Dependencies

This contract belongs specifically to the l6.f1 content family and describes family-only coordination.


## Family dependency posture for `content_family`
- family composition may touch content browser, asset identity, import/reimport flows, dependency and reverse-reference views, content manifests
- family composition may touch authority-facing minimal truth: minimal asset refs and edit intents
- family composition may touch snapshot classes: content snapshots
- family composition may touch index classes: content lookup, dependency, reverse-dependency, and search indices
- family composition may touch derived classes: filtered views, thumbnails, diff summaries
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

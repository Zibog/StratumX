# Family Local Dependencies

This local family contract belongs specifically to the l6.f12 pack release authoring family family and may not be reused by a different family key.


## Allowed for `pack_release_authoring_family`
- member-local coordination for build targets, bake graphs, cook graphs, release manifests, package closure, and release diagnostics
- member-local coordination for authority-facing minimal truth: target refs and explicit task intents only
- member-local coordination for snapshot classes: build/release status snapshots
- member-local coordination for index classes: target and manifest lookup indices
- member-local coordination for derived classes: readiness summaries and closure dashboards
- package-root family registry and shared ids
- lower packages only through member-legal surfaces

## Forbidden
- undeclared member truth
- unrelated domain truth

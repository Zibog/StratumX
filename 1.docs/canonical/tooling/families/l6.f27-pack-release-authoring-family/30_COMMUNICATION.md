# Communication

This contract belongs specifically to the l6.f12 pack release authoring family and describes family-only coordination.


## Family-local communication for `pack_release_authoring_family`
- coordination and refresh propagation related to build targets, bake graphs, cook graphs, release manifests, package closure, and release diagnostics
- coordination and refresh propagation related to authority-facing minimal truth: target refs and explicit task intents only
- coordination and refresh propagation related to snapshot classes: build/release status snapshots
- coordination and refresh propagation related to index classes: target and manifest lookup indices
- coordination and refresh propagation related to derived classes: readiness summaries and closure dashboards

## Note
The family coordinates these member concerns without introducing a new authority path.

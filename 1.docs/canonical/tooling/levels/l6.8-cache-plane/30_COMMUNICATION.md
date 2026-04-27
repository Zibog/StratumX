# Communication

This communication contract belongs specifically to the cache plane tooling level.

## Sends or publishes
- rebuildable cache entries for search, preview, and diagnostics
- sender and receiver scopes are explicit
- mutating flows remain authority or transaction visible

## Required posture
- sender and receiver scopes are explicit for cache plane work
- published records such as cache_entry_id, cache_class remain typed and bounded
- mutating flows remain authority or transaction visible where applicable

## Audit rule
The communication surface for cache plane may not be copied to another level without changing operational meaning.

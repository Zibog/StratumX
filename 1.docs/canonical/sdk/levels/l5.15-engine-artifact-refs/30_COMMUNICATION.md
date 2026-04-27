# Communication

## Publication classes
- publishes immutable artifact refs for captures, replays, and exported records

## Synchronization surfaces
- artifact ref publish surface
- artifact ref lookup surface
- artifact export handoff

## Communication law
Communication in `engine_artifact_refs` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

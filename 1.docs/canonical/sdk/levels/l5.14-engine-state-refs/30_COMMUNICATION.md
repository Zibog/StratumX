# Communication

## Publication classes
- publishes immutable state refs for observations and diagnostics

## Synchronization surfaces
- state ref publish surface
- state ref lookup surface
- observation export handoff

## Communication law
Communication in `engine_state_refs` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

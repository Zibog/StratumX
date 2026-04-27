# Communication

## Publication classes
- publishes immutable version rows consumed by packets, controls, and metrics
- never emits mutable runtime state

## Synchronization surfaces
- version registry publish surface
- version lookup surface

## Communication law
Communication in `compat_versions` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

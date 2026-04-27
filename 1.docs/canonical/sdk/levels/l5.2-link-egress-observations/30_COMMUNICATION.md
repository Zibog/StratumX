# Communication

## Publication classes
- consumes immutable factual snapshots
- publishes read-only observation batches
- emits no mutating control intent

## Synchronization surfaces
- engine factual snapshot handoff
- L4 observation subscription surface
- bounded observation stream surface

## Communication law
Communication in `link_egress_observations` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

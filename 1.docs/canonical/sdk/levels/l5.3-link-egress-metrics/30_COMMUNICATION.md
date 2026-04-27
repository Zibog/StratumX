# Communication

## Publication classes
- receives counters/gauges already reduced by engine exports
- publishes metric batches for dashboards and monitoring APIs
- emits explicit quality flags when sampling degrades

## Synchronization surfaces
- engine metric export handoff
- L4 metric subscription surface
- bounded metric stream surface

## Communication law
Communication in `link_egress_metrics` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

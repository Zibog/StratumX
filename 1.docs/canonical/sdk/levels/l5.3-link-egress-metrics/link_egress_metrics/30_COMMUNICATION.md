# Link Egress Metrics Local Communication

## Sends or publishes
- receives counters/gauges already reduced by engine exports
- publishes metric batches for dashboards and monitoring APIs
- emits explicit quality flags when sampling degrades

## Receives or resolves
- engine metric export handoff
- L4 metric subscription surface
- bounded metric stream surface

## Local law
The communication contour above is exhaustive for `link_egress_metrics`.

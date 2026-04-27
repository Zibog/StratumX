# Link Egress Observations Local Communication

## Sends or publishes
- consumes immutable factual snapshots
- publishes read-only observation batches
- emits no mutating control intent

## Receives or resolves
- engine factual snapshot handoff
- L4 observation subscription surface
- bounded observation stream surface

## Local law
The communication contour above is exhaustive for `link_egress_observations`.

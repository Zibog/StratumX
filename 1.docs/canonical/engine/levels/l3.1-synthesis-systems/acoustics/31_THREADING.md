# Threading

## Ownership

No global threading ownership.

## Posture

Parallel-ready by source groups, spatial partitions, and acoustic solve batches under runtime control.

## Parallel Units

Source groups, regions, path batches, occlusion batches.

## Write Model

Produces acoustic outputs; does not own world mutation.

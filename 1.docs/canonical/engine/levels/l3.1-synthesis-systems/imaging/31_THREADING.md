# Threading

## Ownership

No global threading ownership.

## Posture

Parallel-ready by extraction partitions, visibility tiles, synthesis batches, and transfer-completion batches under runtime control.

## Parallel Units

Regions, scene partitions, tiles, light batches, and frame-local synthesis batches.

## Write Model

Produces synthesis outputs; does not own world mutation.

# Threading

## Ownership

No global threading ownership.

## Posture

Partition-first substrate for runtime-driven parallel work.

## Parallel Units

Regions and chunks.

## Write Model

Dirty tracking and version transitions are world-owned above this layer.

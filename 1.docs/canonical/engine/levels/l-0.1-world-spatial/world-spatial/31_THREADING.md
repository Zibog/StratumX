# Threading

## Ownership

No global threading ownership.

## Posture

Parallel-safe spatial reads and partition-aware spatial descriptors.

## Parallel Units

Spatial partitions and region-aligned reads.

## Write Model

Spatial updates are world-owned above this layer.

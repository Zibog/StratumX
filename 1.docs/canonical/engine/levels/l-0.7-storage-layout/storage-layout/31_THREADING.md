# Threading

## Ownership

No global threading ownership.

## Posture

Partition-friendly, cache-aware layout designed for runtime-driven parallelism.

## Parallel Units

Chunks, columns, and locality partitions.

## Write Model

Layout definitions are configured structurally rather than mutated in hot execution.

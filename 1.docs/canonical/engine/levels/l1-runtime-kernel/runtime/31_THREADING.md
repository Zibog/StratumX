# Threading

## Ownership

Owns global parallel execution policy.

## Posture

Partition-aware scheduler with hard read/compute/resource/authority-sync/apply separation.

## Parallel Units

Regions, chunks, islands, batches, connection batches, and family-specific task groups.

## Write Model

Controls when staged outputs can be applied into world truth and when ordered publication is required for stream or connection correctness.

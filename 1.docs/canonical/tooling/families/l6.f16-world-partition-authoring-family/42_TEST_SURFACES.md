# Test Surfaces

This contract belongs specifically to the l6.f1 world partition authoring family and describes family-only coordination.


## Required checks for `world_partition_authoring_family`
- composition-only legality across world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring, authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs, snapshot classes: partition snapshots and region/cell snapshots
- dependency legality against member contracts
- activation and invalidation propagation for the declared family members

## Operational note
This file remains active and package-specific for `l6.f16-world-partition-authoring-family` / `42_TEST_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 42 TEST SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

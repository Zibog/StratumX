# Boundary Preservation

This contract belongs specifically to the l6.f1 world partition authoring family and describes family-only coordination.


## Must remain outside `world_partition_authoring_family` ownership
- member-internal mutable authority rows for world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring, authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs, snapshot classes: partition snapshots and region/cell snapshots, index classes: region/cell lookup indices and layer-membership indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f16-world-partition-authoring-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

# Family Local Boundary Preservation

This local family contract belongs specifically to the l6.f1 world partition authoring family and may not be reused by a different family key.


## Must stay out of `world_partition_authoring_family`
- authority ownership for world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring, authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs, snapshot classes: partition snapshots and region/cell snapshots, index classes: region/cell lookup indices and layer-membership indices
- unrelated domain truth
- editor-local UI state for other families

## Operational note
This file remains active and package-specific for `world_partition_authoring_family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.

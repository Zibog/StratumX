# Composition

## Member composition
- world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring
- authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs
- snapshot classes: partition snapshots and region/cell snapshots
- index classes: region/cell lookup indices and layer-membership indices
- derived classes: partition views, world heatmaps, authoring overlays

## Composition rule
The family exists to make domain adjacency for `world_partition_authoring_family` explicit, not to merge members into one truth owner.

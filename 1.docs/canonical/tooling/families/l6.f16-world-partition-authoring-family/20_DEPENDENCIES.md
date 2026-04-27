# Dependencies

This contract belongs specifically to the l6.f1 world partition authoring family and describes family-only coordination.


## Family dependency posture for `world_partition_authoring_family`
- family composition may touch world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring
- family composition may touch authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs
- family composition may touch snapshot classes: partition snapshots and region/cell snapshots
- family composition may touch index classes: region/cell lookup indices and layer-membership indices
- family composition may touch derived classes: partition views, world heatmaps, authoring overlays
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination

# Communication

This contract belongs specifically to the l6.f1 world partition authoring family and describes family-only coordination.


## Family-local communication for `world_partition_authoring_family`
- coordination and refresh propagation related to world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring
- coordination and refresh propagation related to authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs
- coordination and refresh propagation related to snapshot classes: partition snapshots and region/cell snapshots
- coordination and refresh propagation related to index classes: region/cell lookup indices and layer-membership indices
- coordination and refresh propagation related to derived classes: partition views, world heatmaps, authoring overlays

## Note
The family coordinates these member concerns without introducing a new authority path.

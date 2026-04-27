# Data Responsibility

This contract belongs specifically to the l6.f1 world partition authoring family family and describes family-only coordination.


## Family-scoped coordination for `world_partition_authoring_family`
- locality, activation, and diagnostics around world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring
- locality, activation, and diagnostics around authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs
- locality, activation, and diagnostics around snapshot classes: partition snapshots and region/cell snapshots
- locality, activation, and diagnostics around index classes: region/cell lookup indices and layer-membership indices
- locality, activation, and diagnostics around derived classes: partition views, world heatmaps, authoring overlays

## Responsibility note
These coordination classes describe family behavior and do not replace member truth ownership.

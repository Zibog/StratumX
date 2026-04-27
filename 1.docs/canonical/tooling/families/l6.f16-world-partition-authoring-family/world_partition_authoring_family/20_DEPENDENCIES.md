# Family Local Dependencies

This local family contract belongs specifically to the l6.f1 world partition authoring family and may not be reused by a different family key.


## Allowed for `world_partition_authoring_family`
- member-local coordination for world partitions, cells, regions, world chunks, data-layer and streaming-unit authoring
- member-local coordination for authority-facing minimal truth: partition refs, cell membership refs, data-layer refs, streaming-policy refs
- member-local coordination for snapshot classes: partition snapshots and region/cell snapshots
- member-local coordination for index classes: region/cell lookup indices and layer-membership indices
- member-local coordination for derived classes: partition views, world heatmaps, authoring overlays
- package-root family registry and shared ids
- lower packages only through member-legal surfaces

## Forbidden
- undeclared member truth
- unrelated domain truth

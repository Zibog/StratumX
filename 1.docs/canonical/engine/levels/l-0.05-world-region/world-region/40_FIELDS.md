# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| region_descriptor | RegionDescriptor | required | immutable region identity | owned by `engine_world_region` |
| chunk_descriptor | ChunkDescriptor | required | immutable chunk identity within region | owned by `engine_world_region` |
| chunk_edge_length | Distance | required | frozen at 32 m equivalent world-space edge | numeric authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §4 |
| vertical_slab_height | Distance | required | frozen at 16 m | numeric authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §4 |
| dirty_map | DirtyMap | required | width-bounded and versioned | owned by `engine_world_region` |
| region_version | VersionMetadata | required | attached to region boundaries for safe progression | owned by `engine_world_region` |
| same_tick_halo_width | ChunkCount | required | default 1 chunk | numeric authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §4 |
| max_field_solve_halo_width | ChunkCount | required | maximum 2 chunks | numeric authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §4 |

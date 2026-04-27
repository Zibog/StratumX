# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| coordinate | WorldCoordinate | required | world-space position semantic | owned by `engine_world_spatial` |
| transform | Transform | required | transform relationship for world-bound objects | owned by `engine_world_spatial` |
| spatial_address | SpatialAddress | required | deterministic spatial addressing binding | owned by `engine_world_spatial` |
| spatial_relation | SpatialRelation | required | adjacency, containment, distance semantics | owned by `engine_world_spatial` |
| world_local_space | CoordinateSpace | required | world-local coordinate space | owned by `engine_world_spatial` |
| region_local_space | CoordinateSpace | optional | region-local coordinate space | owned by `engine_world_spatial` |
| presentation_space | CoordinateSpace | optional | rebased presentation coordinates | owned by `engine_world_spatial` |

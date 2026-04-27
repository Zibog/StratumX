# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| layout_class | Layout classification | Required | Chosen per workload, not universal dogma | engine_storage_layout |
| chunk_descriptor | Chunk layout structure | Optional | Dense chunk layout for hot signatures | engine_storage_layout |
| chunk_signature | Component signature | Required (for chunks) | Frozen at chunk creation | engine_storage_layout |
| chunk_access_mode | Access mode | Required (for chunks) | Frozen at chunk creation | engine_storage_layout |
| chunk_invalidation_law | Invalidation rule | Required (for chunks) | Frozen at chunk creation | engine_storage_layout |
| column_family | Column grouping | Optional | Component-aligned for cache efficiency | engine_storage_layout |
| column_descriptor | Column layout structure | Optional | Column-oriented placement | engine_storage_layout |
| locality_class | Locality classification | Required | Cache, spatial, partition, or traversal-lane | engine_storage_layout |
| cache_locality | Cache behavior | Optional | Shapes layout decisions | engine_storage_layout |
| spatial_locality | Spatial behavior | Optional | Shapes layout decisions | engine_storage_layout |
| partition_locality | Partition behavior | Optional | Shapes layout decisions | engine_storage_layout |
| traversal_lane_locality | Traversal behavior | Optional | Shapes layout decisions | engine_storage_layout |

## Invariant Rules

- Layout class is chosen per workload, not by universal storage dogma
- Chunk layout is legal only when traversal plan freezes signature, access mode, and invalidation law
- Chunk layout may not redefine public identity or registry truth
- Prepared dense traversal is legal for critical execution lanes only when locality class is explicit
- Query-plan invalidation must be frozen for prepared dense traversal
- Public identity and structural truth remain stable and sparse-safe
- Memory shape must stay independent from access model, mutation staging, and query logic

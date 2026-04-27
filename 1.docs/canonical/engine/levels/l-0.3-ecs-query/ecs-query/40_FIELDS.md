# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| component_set | Component type set | Required | Explicit and closed; compile invalid only on structural mismatch | engine_ecs_query |
| access_mode | Read/Write legality | Required | Explicit; invalid on legality widening | engine_ecs_query |
| locality_class | Locality classification | Required | Explicit and frozen; invalid on locality change only | engine_ecs_query |
| partitionability | Partition class | Required | Explicit; invalid on partition-class change | engine_ecs_query |
| cache_key | Descriptor identity | Required | Explicit and stable; invalid on descriptor identity change | engine_ecs_query |
| filter_constraints | Filter set | Optional | Deterministic narrowing rules | engine_ecs_query |
| join_rules | Join composition | Optional | Multi-set traversal legality | engine_ecs_query |
| read_rights | Access rights (read) | Required | Explicit and typed | engine_ecs_query |
| write_rights | Access rights (write) | Required | Explicit and typed | engine_ecs_query |
| publication_rights | Publication access | Optional | Explicit when present | engine_ecs_query |
| scratch_class | Scratch buffer class | Optional | Explicit and bind-time checked | engine_ecs_query |

## Invariant Rules

- Component sets must not be inferred on bind
- Access mode must not allow caller-convention legality violations
- Locality class must not allow implicit widening
- Partitionability must not allow per-iteration repartitioning
- Cache key must not allow hidden per-call mutation
- Access rights must be explicit, typed, and checked at traversal bind
- Hidden write widening through helper wrappers is illegal

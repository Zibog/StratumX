# Boundary Preservation

## Canonical Rule

`engine_storage_layout` provides physical organization of storage. Memory shape must stay independent from access model, mutation staging, and query logic to maintain layout as a standalone pressure axis.

## Upward Boundary

**Exports to layers above:**
- Layout descriptors (canonical memory-organization model)
- Chunk descriptors (dense chunk layout structures)
- Column descriptors (column-oriented storage organization)
- Locality profiles (cache and locality-oriented storage structure)

**Canonical consumers:**
- `engine_storage_access` — Access model layer
- `engine_storage_mutation` — Mutation staging layer
- `engine_ecs_registry` — Registry layer
- `engine_ecs` — ECS assembly layer
- `engine_runtime` — Runtime kernel

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts and types

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Chunk layout redefining public identity or registry truth
- Chunk layout without frozen signature, access mode, and invalidation law
- Prepared dense traversal for critical execution lanes without explicit locality class
- Prepared dense traversal without frozen query-plan invalidation
- Layout class chosen by universal storage dogma instead of per-workload decision
- Direct coupling to access model, mutation staging, or query logic
- Bypassing core contracts for type-level storage assumptions

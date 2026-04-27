# Boundary Preservation

## Canonical Rule

`engine_ecs` provides ECS assembly and entity/component composition truth. It must stay separate from raw registry storage and from query/traversal execution to keep composition semantics explicit.

## Upward Boundary

**Exports to layers above:**
- Entity descriptors (composed entity truth)
- Component bundles (grouped component state)
- Composition templates (reusable assembly patterns)

**Canonical consumers:**
- `engine_runtime` — Runtime kernel
- `engine_ecs_query` — Query and traversal layer (consumes assembled descriptors)
- `engine_world` — World truth layer

## Downward Boundary

**Imports from layers below:**
- `engine_ecs_registry` — Structural registration tables and presence maps
- `engine_core` — Base contracts
- `engine_identity` — Identity primitives
- `engine_handle` — Stable references

## Lateral Boundary

**Interaction with sibling layers:**
- Coordinates with `engine_storage_layout` for component storage shape
- Coordinates with `engine_storage_mutation` for bulk composition updates

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Assembly layer owning raw registry storage (belongs in ecs_registry)
- Assembly layer owning query/traversal logic (belongs in ecs_query)
- Assembly layer owning storage access patterns (belongs in storage_access)
- Direct entity reference without identity/handle primitives
- Bypassing ecs_registry for structural membership queries

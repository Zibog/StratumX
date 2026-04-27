# Boundary Preservation

## Canonical Rule

`engine_ecs_registry` provides structural ECS registration truth. It must stay separate from traversal and execution to keep structural membership explicit and stable.

## Upward Boundary

**Exports to layers above:**
- Registry tables (structural registration state)
- Presence maps (component presence truth)
- Membership descriptors (entity-component mappings)

**Canonical consumers:**
- `engine_ecs_query` — Query and traversal layer
- `engine_ecs` — ECS assembly layer
- `engine_world` — World truth layer
- `engine_runtime` — Runtime kernel

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_identity` — Identity primitives
- `engine_handle` — Stable references
- `engine_storage_layout` — Storage shape context

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Registry owning steady-state traversal logic (belongs in ecs_query)
- Registry owning convenience lookups (belongs in higher layers)
- Registry owning compiled lane execution (belongs in runtime)
- Direct access to storage access or mutation layers (must go through storage_layout)
- Bypassing identity or handle layers for entity references

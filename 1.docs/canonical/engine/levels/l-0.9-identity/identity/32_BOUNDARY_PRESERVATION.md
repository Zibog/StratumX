# Boundary Preservation

## Canonical Rule

`engine_identity` provides identity substrate for entities and components. The engine must separate who exists from how data is stored or accessed, making identity a standalone pressure boundary.

## Upward Boundary

**Exports to layers above:**
- EntityId (stable entity identity tokens)
- ComponentId (stable component identity tokens)
- Generation domains (generation-aware identity lifecycle)
- Identity descriptors (typed identity semantics)

**Canonical consumers:**
- `engine_handle` — Handle system layer
- `engine_ecs_registry` — Registry layer
- `engine_ecs_query` — Query and traversal layer
- `engine_ecs` — ECS assembly layer
- `engine_world_spatial` — Spatial world layer
- `engine_world` — World truth layer
- `engine_runtime` — Runtime kernel
- `engine_agents` — Agent systems

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base types and contracts

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Entity identity as dense traversal token
- Entity identity carrying execution-lane locality assumptions
- Immediate recycling of freshly retired identities into same execution epoch
- Insufficient generation width for stale-handle aliasing protection
- Mixing entity, component, and adjacent identity spaces without typed domains
- Direct coupling to storage or access layers
- Bypassing core contracts for base types

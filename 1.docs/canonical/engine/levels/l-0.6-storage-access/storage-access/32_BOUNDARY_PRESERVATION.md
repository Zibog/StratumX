# Boundary Preservation

## Canonical Rule

`engine_storage_access` provides deterministic read/write access model over storage layout. Access must stay independent from layout and mutation staging to keep read/write legality explicit and schedulable.

## Upward Boundary

**Exports to layers above:**
- Read views (immutable storage views)
- Write windows (controlled mutable access windows)
- Access descriptors (access legality and mode descriptors)
- Traversal entrypoints (deterministic traversal entry surfaces)

**Canonical consumers:**
- `engine_storage_mutation` — Mutation staging layer
- `engine_ecs_query` — Query and traversal layer
- `engine_ecs` — ECS assembly layer
- `engine_runtime` — Runtime kernel
- `engine_agents` — Agent systems

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_handle` — Stable references
- `engine_storage_layout` — Physical storage structure

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Unregistered mutable bypasses (all write windows must be bound to access descriptors)
- Ad hoc bind-time compile on cache hit
- Hidden write widening after bind
- Hidden region widening after bind
- Pool swap after bind for scratch ownership
- Direct write entry in traversal (must use staged mutation handoff)
- Allocating new plan on cache hit
- Widening locality or access rights after bind
- Rewriting partition boundaries during iteration
- Direct access to identity layer (must go through handle)

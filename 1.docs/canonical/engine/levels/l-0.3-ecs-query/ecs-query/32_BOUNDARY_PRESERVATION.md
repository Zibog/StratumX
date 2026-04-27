# Boundary Preservation

## Canonical Rule

`engine_ecs_query` provides deterministic query and traversal model for ECS operations. It must remain separate from registry state and execution orchestration to preserve traversal logic independence.

## Upward Boundary

**Exports to layers above:**
- Query descriptors (compiled ECS query plans)
- Filters (traversal filter constraints)
- Joins (multi-set traversal composition)
- Access descriptors (read/write rights for query plans)

**Canonical consumers:**
- `engine_ecs` — ECS assembly layer
- `engine_world` — World truth layer
- `engine_runtime` — Runtime kernel
- `engine_agents` — Agent systems

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_identity` — Identity primitives
- `engine_handle` — Stable references
- `engine_storage_access` — Access windows and views
- `engine_ecs_registry` — Structural membership truth

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Cache-hit descriptors compiling new plans
- Hidden access widening through helper wrappers
- Inferred component sets on bind
- Runtime repartitioning by surprise
- Hidden per-call mutation of cache identity
- Direct access to storage mutation or layout layers (must go through storage_access)
- Bypassing registry for structural truth

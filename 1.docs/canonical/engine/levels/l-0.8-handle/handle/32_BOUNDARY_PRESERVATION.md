# Boundary Preservation

## Canonical Rule

`engine_handle` provides stable temporal references over identity-bearing structures. Handle lifecycle must stay isolated from identity and storage to prevent temporal reference drift.

## Upward Boundary

**Exports to layers above:**
- Handle types (stable temporal reference types)
- Validation results (generation-aware validation outcomes)
- Invalidation rules (handle invalidation state transitions)
- Generation checks (stale-handle detection)

**Canonical consumers:**
- `engine_storage_access` — Access model layer
- `engine_storage_mutation` — Mutation staging layer
- `engine_ecs_registry` — Registry layer
- `engine_ecs_query` — Query and traversal layer
- `engine_ecs` — ECS assembly layer
- `engine_world` — World truth layer
- `engine_runtime` — Runtime kernel
- `engine_material` — Material layer

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_identity` — Identity domains and typed ids

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Dense execution handles escaping compiled traversal plans or owning batches
- Repeated stable-handle validation inside steady-state compiled traversal
- Validation outside boundary entry, diagnostics, or plan-build boundaries
- Temporal reference drift through coupling to storage lifecycle
- Direct coupling to storage layers (must remain independent)
- Bypassing identity layer for identity domains

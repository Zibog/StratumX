# engine_ecs

## Stack Position

L-0.2. ECS Assembly

## Primary Role

Assembled ECS substrate.

## Canonical Scope

Thin assembly surface that binds identity, handles, storage, registry, and query into one ECS-facing boundary.

## Boundary Rationale

ECS assembly must stay thin so it remains a substrate boundary rather than regrowing into a monolith.

## Canonical Consumers

- `engine_world`
- `engine_runtime`
- `engine_agents`
- `engine_imaging`
- `engine_acoustics`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_identity` — Identity primitives.
- `engine_handle` — Stable references.
- `engine_storage_layout` — Storage structure.
- `engine_storage_access` — Access windows.
- `engine_storage_mutation` — Mutation staging.
- `engine_ecs_registry` — Structural truth.
- `engine_ecs_query` — Traversal model.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Assembly Surface | Thin ECS assembly surface. | `40_ASSEMBLY_SURFACE.md` |
| Integration Boundary | Boundary between lower ECS substrate and world/runtime consumers. | `41_INTEGRATION_BOUNDARY.md` |
| ECS Substrate | Assembled ECS-facing substrate. | `42_ECS_SUBSTRATE.md` |

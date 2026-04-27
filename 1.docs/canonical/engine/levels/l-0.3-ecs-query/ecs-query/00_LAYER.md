# engine_ecs_query

## Stack Position

L-0.3. ECS Query

## Primary Role

Deterministic query and traversal model.

## Canonical Scope

Query descriptors, filters, joins, and access descriptors for ECS traversal.

## Boundary Rationale

Traversal logic has its own pressure axis and must stay separate from registry state and final execution orchestration.

## Canonical Consumers

- `engine_ecs`
- `engine_world`
- `engine_runtime`
- `engine_agents`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_identity` — Identity primitives.
- `engine_handle` — Stable references.
- `engine_storage_access` — Access windows and views.
- `engine_ecs_registry` — Structural membership truth.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Query Descriptors | Canonical query descriptor model. | `40_QUERY_DESCRIPTORS.md` |
| Filters | Filter model for ECS traversal. | `41_FILTERS.md` |
| Joins | Join model for multi-set traversal. | `42_JOINS.md` |
| Access Descriptors | Read/write descriptors bound to query traversal. | `43_ACCESS_DESCRIPTORS.md` |

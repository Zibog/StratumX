# engine_ecs_registry

## Stack Position

L-0.4. ECS Registry

## Primary Role

Structural ECS registration truth.

## Canonical Scope

Entity/component presence, membership maps, and structural registration state.

## Boundary Rationale

Registry truth must stay separate from traversal and execution to keep structural membership explicit and stable.

## Canonical Consumers

- `engine_ecs_query`
- `engine_ecs`
- `engine_world`
- `engine_runtime`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_identity` — Identity primitives.
- `engine_handle` — Stable references.
- `engine_storage_layout` — Storage shape context.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Registry Model | Canonical structural registration model. | `40_REGISTRY_MODEL.md` |
| Component Presence | Presence truth for component membership. | `41_COMPONENT_PRESENCE.md` |
| Membership Maps | Structural mapping between entities and component sets. | `42_MEMBERSHIP_MAPS.md` |

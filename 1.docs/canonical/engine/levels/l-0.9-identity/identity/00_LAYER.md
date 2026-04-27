# engine_identity

## Stack Position

L-0.9. Identity Primitives

## Primary Role

Identity substrate for entities and components.

## Canonical Scope

Entity identity, component identity, generation domains, and typed identity semantics.

## Boundary Rationale

Identity needs its own pressure boundary because the engine must separate who exists from how data is stored or accessed.

## Canonical Consumers

- `engine_handle`
- `engine_ecs_registry`
- `engine_ecs_query`
- `engine_ecs`
- `engine_world_spatial`
- `engine_world`
- `engine_runtime`
- `engine_agents`

## Downward Dependencies

- `engine_core` — Base types and contracts.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| EntityId | Primary entity identity type. | `40_ENTITY_ID.md` |
| ComponentId | Primary component identity type. | `41_COMPONENT_ID.md` |
| Generation Model | Generation-aware identity lifecycle semantics. | `42_GENERATION_MODEL.md` |
| Identity Domains | Typed separation of identity namespaces. | `43_IDENTITY_DOMAINS.md` |

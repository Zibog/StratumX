# engine_handle

## Stack Position

L-0.8. Handle System

## Primary Role

Stable temporal references over identity-bearing structures.

## Canonical Scope

Stable handle types, validation, invalidation, and stale-handle detection.

## Boundary Rationale

Handle lifecycle is a separate pressure axis from identity and storage and must stay isolated to prevent temporal reference drift.

## Canonical Consumers

- `engine_storage_access`
- `engine_storage_mutation`
- `engine_ecs_registry`
- `engine_ecs_query`
- `engine_ecs`
- `engine_world`
- `engine_runtime`
- `engine_material`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_identity` — Identity domains and typed ids.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Handle Types | Stable temporal reference types. | `40_HANDLE_TYPES.md` |
| Validation Model | Rules for checking handle validity. | `41_VALIDATION_MODEL.md` |
| Invalidation Model | Rules for handle invalidation. | `42_INVALIDATION_MODEL.md` |
| Stale Handle Detection | Detection of stale temporal references. | `43_STALE_HANDLE_DETECTION.md` |

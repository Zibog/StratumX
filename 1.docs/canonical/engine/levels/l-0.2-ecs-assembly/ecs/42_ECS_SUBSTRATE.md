# Ecs Substrate

## Role

Assembled ECS substrate.

## Canonical Definition

`Ecs Substrate` is a canonical element of `engine_ecs` inside `L-0.2. ECS Assembly`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Full ECS-facing substrate created by composing registry, query, storage, handle, and identity layers.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_ecs`:

- `engine_core`
- `engine_identity`
- `engine_handle`
- `engine_storage_layout`
- `engine_storage_access`
- `engine_storage_mutation`
- `engine_ecs_registry`
- `engine_ecs_query`

## Layer Links

- parent crate: `engine_ecs`
- level: `L-0.2. ECS Assembly`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

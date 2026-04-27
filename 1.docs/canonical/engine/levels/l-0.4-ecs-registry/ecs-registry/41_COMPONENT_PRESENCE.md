# Component Presence

## Role

Presence truth.

## Canonical Definition

`Component Presence` is a canonical element of `engine_ecs_registry` inside `L-0.4. ECS Registry`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Presence model describing whether a component is structurally attached to an entity.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_ecs_registry`:

- `engine_core`
- `engine_identity`
- `engine_handle`
- `engine_storage_layout`

## Layer Links

- parent crate: `engine_ecs_registry`
- level: `L-0.4. ECS Registry`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

# Membership Maps

## Role

Entity-component membership maps.

## Canonical Definition

`Membership Maps` is a canonical element of `engine_ecs_registry` inside `L-0.4. ECS Registry`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Mappings that bind entities to structural component membership sets.

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

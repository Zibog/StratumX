# World State

## Role

Authoritative world root.

## Canonical Definition

`World State` is a canonical element of `engine_world` inside `L0. World Truth`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Canonical root state holding authoritative world truth and world-owned state composition.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_world`:

- `engine_core`
- `engine_handle`
- `engine_ecs`
- `engine_world_spatial`
- `engine_world_region`

## Layer Links

- parent crate: `engine_world`
- level: `L0. World Truth`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

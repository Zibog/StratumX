# Region Versioning

## Role

Region version model.

## Canonical Definition

`Region Versioning` is a canonical element of `engine_world_region` inside `L-0.05. World Region`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Version semantics attached to region and chunk boundaries for safe progression.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_world_region`:

- `engine_core`
- `engine_world_spatial`

## Layer Links

- parent crate: `engine_world_region`
- level: `L-0.05. World Region`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

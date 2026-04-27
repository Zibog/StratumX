# Image Synthesis

## Role

Final image synthesis.

## Canonical Definition

`Image Synthesis` is a canonical element of `engine_imaging` inside `L3.1. Synthesis Systems`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Formation of the image-facing output from extracted scene, visibility, and lighting products.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_imaging`:

- `engine_core`
- `engine_world`
- `engine_ecs`
- `engine_material`
- `engine_world_spatial`
- `engine_world_region`

## Layer Links

- parent crate: `engine_imaging`
- level: `L3.1. Synthesis Systems`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

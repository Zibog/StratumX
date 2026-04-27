# Transformation Pipelines

## Role

Generation post-processing.

## Canonical Definition

`Transformation Pipelines` is a canonical element of `engine_generation` inside `L3.0. Model Systems`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Transform stages that normalize or reshape raw generated products.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_generation`:

- `engine_core`
- `engine_world`
- `engine_inference`

## Layer Links

- parent crate: `engine_generation`
- level: `L3.0. Model Systems`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

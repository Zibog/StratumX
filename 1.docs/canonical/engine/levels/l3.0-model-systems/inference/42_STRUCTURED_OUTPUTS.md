# Structured Outputs

## Role

Normalized inference outputs.

## Canonical Definition

`Structured Outputs` is a canonical element of `engine_inference` inside `L3.0. Model Systems`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Structured, non-authoritative outputs returned from inference work for downstream consumers.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_inference`:

- `engine_core`
- `engine_world`
- `engine_ecs`

## Layer Links

- parent crate: `engine_inference`
- level: `L3.0. Model Systems`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

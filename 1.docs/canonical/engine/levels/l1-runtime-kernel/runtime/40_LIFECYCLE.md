# Lifecycle

## Role

Runtime lifecycle state.

## Canonical Definition

`Lifecycle` is a canonical element of `engine_runtime` inside `L1. Runtime Kernel`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Start, stop, pause, resume, and state-transition model for the active runtime authority.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_runtime`:

- `engine_core`
- `engine_handle`
- `engine_ecs`
- `engine_world`

## Layer Links

- parent crate: `engine_runtime`
- level: `L1. Runtime Kernel`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

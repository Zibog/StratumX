# Headless Profile

## Role

Headless runtime profile.

## Canonical Definition

`Headless Profile` is a canonical element of `engine_runtime_headless` inside `L1. Runtime Kernel`. It exists as one explicit part of the engine stack and does not transfer its ownership class into another crate or level.

## Data Model

Simulation-first execution profile built on the shared runtime constitution.
The headless profile owns no presentation queue and uses the `headless-20` memory and residency envelope, including replay/history burden, as frozen by the absolute-budget and memory/residency constitutions.

## Dependencies

This element depends on the canonical lower boundaries required by `engine_runtime_headless`:

- `engine_runtime`
- `engine_world`
- `engine_ecs`

## Layer Links

- parent crate: `engine_runtime_headless`
- level: `L1. Runtime Kernel`
- layer document: `00_LAYER.md`
- libraries: `10_LIBRARIES.md`
- dependencies: `20_DEPENDENCIES.md`
- communication: `30_COMMUNICATION.md`
- threading: `31_THREADING.md`

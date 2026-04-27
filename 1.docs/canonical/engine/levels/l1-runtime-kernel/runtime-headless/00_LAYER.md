# engine_runtime_headless

## Stack Position

L1. Runtime Kernel

## Primary Role

Headless simulation runtime profile.

## Canonical Scope

Simulation-first runtime wiring, headless cadence, headless execution outputs, and headless role variants.

## Boundary Rationale

Headless execution needs its own profile so simulation-first runs remain clean, deterministic, and free from realtime orchestration noise.

## Canonical Consumers

- `engine_startup`

## Downward Dependencies

- `engine_runtime` — Execution constitution.
- `engine_world` — World truth boundary.
- `engine_ecs` — ECS substrate.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Headless Profile | Canonical headless profile definition. | `40_HEADLESS_PROFILE.md` |
| Simulation Cadence | Headless time and cadence model. | `41_SIMULATION_CADENCE.md` |
| Headless Outputs | Outputs of headless execution. | `42_HEADLESS_OUTPUTS.md` |
| Headless Role Model | Offline and dedicated-server role variants. | `43_HEADLESS_ROLE_MODEL.md` |

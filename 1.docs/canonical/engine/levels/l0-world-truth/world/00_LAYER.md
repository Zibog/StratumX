# engine_world

## Stack Position

L0. World Truth

## Primary Role

Authoritative world owner.

## Canonical Scope

World state, snapshots, read model, and controlled apply boundary.

## Boundary Rationale

The engine needs one world owner so final truth, snapshots, and apply legality stay centralized and deterministic.

## Canonical Consumers

- `engine_material`
- `engine_runtime`
- `engine_kinetics`
- `engine_field`
- `engine_agents`
- `engine_inference`
- `engine_generation`
- `engine_imaging`
- `engine_acoustics`
- `engine_startup`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_handle` — Stable references.
- `engine_ecs` — ECS substrate.
- `engine_world_spatial` — Spatial substrate.
- `engine_world_region` — Region substrate.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| World State | Authoritative root world state. | `40_WORLD_STATE.md` |
| Snapshots | Immutable world snapshots. | `41_SNAPSHOTS.md` |
| Read Model | Versioned read-facing world model. | `42_READ_MODEL.md` |
| Apply Boundary | Controlled boundary for integrating staged changes. | `43_APPLY_BOUNDARY.md` |

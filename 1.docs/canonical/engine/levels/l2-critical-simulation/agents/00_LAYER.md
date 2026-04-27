# engine_agents

## Stack Position

L2. Critical Simulation Families

## Primary Role

Agent and society simulation family.

## Canonical Scope

Navigation, perception, decision, society, and schedule as one agent-centric simulation family.

## Boundary Rationale

These systems share an agent-centric, read-heavy, batch-oriented pressure axis distinct from physics and distributed fields.

## Canonical Consumers

- `engine_runtime`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_ecs` — ECS substrate.
- `engine_world` — World truth boundary.
- `engine_world_region` — Region substrate.
- `engine_runtime` — Runtime legality windows and phase ownership.
- `engine_memory_control` — Allocation class and pressure boundary for family-local working sets.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Navigation | Movement and path products. | `40_NAVIGATION.md` |
| Perception | Sensory and awareness products. | `41_PERCEPTION.md` |
| Decision | Action selection and local planning products. | `42_DECISION.md` |
| Society | Social graph, influence, and relation products. | `43_SOCIETY.md` |
| Schedule | Temporal obligations and schedule products. | `44_SCHEDULE.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.

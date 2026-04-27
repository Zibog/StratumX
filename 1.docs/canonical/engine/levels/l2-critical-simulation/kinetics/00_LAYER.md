# engine_kinetics

## Stack Position

L2. Critical Simulation Families

## Primary Role

Local dynamics and contact simulation family.

## Canonical Scope

Collision, rigid body dynamics, trajectory solve, and impact response.

## Boundary Rationale

These compute paths share a local, fast-step, contact-and-motion pressure axis and belong together as one critical simulation family.

## Canonical Consumers

- `engine_runtime`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_material` — Shared world property substrate.
- `engine_world_spatial` — Spatial substrate.
- `engine_world_region` — Partition substrate.
- `engine_runtime` — Runtime legality windows and phase ownership.
- `engine_memory_control` — Allocation class and pressure boundary for family-local working sets.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Collision | Contact detection and contact-state production. | `40_COLLISION.md` |
| Rigid Body | Body motion, mass, impulse, and constraint solve. | `41_RIGIDBODY.md` |
| Trajectory | Trajectory solve under force and medium influence. | `42_TRAJECTORY.md` |
| Impact | Material-aware impact, penetration, ricochet, and deformation response. | `43_IMPACT.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.

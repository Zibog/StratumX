# engine_field

## Stack Position

L2. Critical Simulation Families

## Primary Role

Distributed physical field simulation family.

## Canonical Scope

Fluid, thermal, terrain, structural, and atmospheric field solve over world space.

## Boundary Rationale

These systems share a distributed field pressure axis and region-oriented cadence distinct from local kinetics and agent compute.

## Canonical Consumers

- `engine_runtime`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_material` — Shared world property substrate.
- `engine_world_spatial` — Spatial substrate.
- `engine_world_region` — Region substrate.
- `engine_runtime` — Runtime legality windows and phase ownership.
- `engine_memory_control` — Allocation class and pressure boundary for field-local working sets.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Fluid Field | Fluid distribution, accumulation, containment, and leakage solve. | `40_FLUID_FIELD.md` |
| Thermal Field | Heat transfer, ignition, drying, and combustion-adjacent thermal solve. | `41_THERMAL_FIELD.md` |
| Terrain Field | Terrain deformation and substrate transformation solve. | `42_TERRAIN_FIELD.md` |
| Structural Field | Load, support, fracture, and collapse field solve. | `43_STRUCTURAL_FIELD.md` |
| Atmospheric Field | Pressure, wind, humidity, precipitation, and atmospheric transport solve. | `44_ATMOSPHERIC_FIELD.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.

# Boundary Preservation

## Canonical Rule

`engine_field` preserves the boundary between distributed physical field simulation and the world/runtime substrate beneath it.

## Preserved Earlier Boundaries

The following earlier standalone boundaries remain visible as explicit internal field documents:
- `engine_fluids`
- `engine_terrain`

## Widened Earlier Boundaries

The following earlier boundaries are preserved as widened field elements:
- `engine_fire` -> `Thermal Field`
- `engine_destruction` -> `Structural Field`
- `engine_weather` -> `Atmospheric Field`

## Reason

These systems share distributed field-style execution and are best preserved as one family with explicit internal boundaries.

## Upward Boundary

Exports to layers above:
- Field deltas
- Region deltas
- Structural products
- Atmospheric products

## Downward Boundary

Imports from layers below:
- `engine_core` — Base contracts
- `engine_world` — World truth boundary
- `engine_material` — Shared world property substrate
- `engine_world_spatial` — Spatial substrate
- `engine_world_region` — Region substrate
- `engine_runtime` — Runtime legality windows and phase ownership
- `engine_memory_control` — Allocation class and pressure boundary for field-local working sets

## Forbidden Crossings

- Must not bypass world/apply law to mutate world truth directly
- Must not widen into planet-scale full solve every tick
- Must not steal ownership of foreign-family truth surfaces (kinetics, agents, synthesis)
- Must not create hidden secondary solver loops outside canonical solve order
- Must not normalize unbounded cross-chunk spill or debris storms
- Must not consume near entitlement for far/decorative field work under simulation-tier law

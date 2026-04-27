# Boundary Preservation

## Canonical Rule

`engine_kinetics` preserves the boundary between local dynamics/contact simulation and the world/runtime substrate beneath it.

## Preserved Earlier Boundaries

`engine_collision` and `engine_rigidbody` are preserved as explicit internal boundaries.

## Widened Earlier Boundary

`engine_ballistics` is widened and split into:
- `Trajectory`
- `Impact`

This preserves ballistic computation while separating flight and impact concerns.

## Reason

The family boundary is narrower in ownership type and clearer in execution cost than three standalone crates.

## Upward Boundary

Exports to layers above:
- Contact products
- Motion deltas
- Trajectory products
- Impact products

## Downward Boundary

Imports from layers below:
- `engine_core` — Base contracts
- `engine_world` — World truth boundary
- `engine_material` — Shared world property substrate
- `engine_world_spatial` — Spatial substrate
- `engine_world_region` — Partition substrate
- `engine_runtime` — Runtime legality windows and phase ownership
- `engine_memory_control` — Allocation class and pressure boundary for family-local working sets

## Forbidden Crossings

- Must not bypass world/apply law to mutate world truth directly
- Must not widen into global monolithic solve or unbounded island scope
- Must not steal ownership of foreign-family truth surfaces (field, agents, material)
- Must not create unbounded contact caches or projectile history retention
- Must not normalize always-max fidelity for all projectiles regardless of tier
- Must not consume near entitlement for far/offscreen kinetics work under simulation-tier law

# Boundary Preservation

## Canonical Rule

`engine_agents` preserves the boundary between agent-centric simulation (navigation, perception, decision, society, schedule) and the runtime/world substrate beneath it.

## Preserved Earlier Boundaries

The following earlier standalone boundaries remain explicit inside `engine_agents`:
- `engine_navigation`
- `engine_perception`
- `engine_schedule`

## Widened Earlier Boundaries

The following earlier boundaries are preserved as widened internal elements:
- `engine_ai` -> `Decision`
- `engine_social` -> `Society`

## Reason

The family boundary keeps agent and society computation in one execution class while preserving exact internal functional splits.

## Upward Boundary

Exports to layers above:
- Navigation products
- Perception products
- Action intents
- Society deltas
- Schedule products

## Downward Boundary

Imports from layers below:
- `engine_core` — Base contracts
- `engine_ecs` — ECS substrate
- `engine_world` — World truth boundary
- `engine_world_region` — Region substrate
- `engine_runtime` — Runtime legality windows and phase ownership
- `engine_memory_control` — Allocation class and pressure boundary for family-local working sets

## Forbidden Crossings

- Must not bypass world/apply law to mutate world truth directly
- Must not widen into monolithic global planners or unbounded perception systems
- Must not steal ownership of foreign-family truth surfaces (kinetics, field, material)
- Must not create hidden coordination state or invalidation storms outside canonical solve order
- Must not normalize combat-rate society work or per-frame schedule recompute
- Must not consume near entitlement for far/offscreen agent work under simulation-tier law

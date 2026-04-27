# Boundary Preservation

## Canonical Rule

`engine_world` is the authoritative world owner. Final truth, snapshots, and apply legality must stay centralized and deterministic.

## Upward Boundary

**Exports to layers above:**
- World state (authoritative root world state)
- Snapshots (immutable world snapshots)
- Read model (versioned read-facing world view)
- Apply boundary (controlled integration boundary for staged changes)

**Canonical consumers:**
- `engine_material` — Material layer
- `engine_runtime` — Runtime kernel
- `engine_kinetics` — Kinetics simulation
- `engine_field` — Field simulation
- `engine_agents` — Agent systems
- `engine_inference` — Inference systems
- `engine_generation` — Generation systems
- `engine_imaging` — Imaging systems
- `engine_acoustics` — Acoustics systems
- `engine_startup` — Startup orchestration

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_handle` — Stable references
- `engine_ecs` — ECS substrate
- `engine_world_spatial` — Spatial substrate
- `engine_world_region` — Region substrate

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Whole-world monolithic integration when legal segmented apply exists
- Recursive segment spawning inside apply
- Publication order depending on hash-map iteration or worker race
- Micro-segmentation used to hide monolithic apply cost
- Exceeding max segments per authoritative tick (256)
- Exceeding max family fan-out per segment (8)
- Exceeding publish-order passes across all segments (2 ordered passes)
- Bypassing apply boundary for world state mutation
- Mutable access to snapshots (must remain immutable)

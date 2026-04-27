# Boundary Preservation

## Canonical Rule

`engine_world_region` preserves the boundary between spatial substrate and region/chunk partitioning.

## Upward Boundary

Exports to layers above:
- Region descriptors
- Chunk descriptors
- Dirty tracking metadata
- Region version boundaries

## Downward Boundary

Imports from layers below:
- `engine_core` — Base contracts
- `engine_world_spatial` — Spatial substrate

## Forbidden Crossings

- Must not merge region partitioning into spatial coordinate logic
- Must not expose chunk grain implementation details upward beyond canonical descriptors
- Must not bypass spatial substrate when defining region addressing

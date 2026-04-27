# Boundary Preservation

## Canonical Rule

`engine_world_spatial` preserves the boundary between base contracts and world-specific spatial substrate.

## Upward Boundary

Exports to layers above:
- Spatial descriptors
- Transforms
- Spatial addresses
- World-space relations

## Downward Boundary

Imports from layers below:
- `engine_core` — Base types and contracts
- `engine_identity` — Identity primitives
- `engine_handle` — Stable references where spatially indexed

## Forbidden Crossings

- Must not merge spatial substrate into region partitioning logic
- Must not expose coordinate implementation details beyond canonical spatial descriptors
- Must not bypass identity/handle primitives when defining spatial references

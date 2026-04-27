# engine_world_spatial

## Stack Position

L-0.1. World Spatial

## Primary Role

World-specific spatial substrate.

## Canonical Scope

Coordinates, transforms, spatial addressing, and spatial relations.

## Boundary Rationale

Spatial truth is a distinct world-facing substrate and must stay separate from regioning and final world ownership.

## Canonical Consumers

- `engine_world_region`
- `engine_world`
- `engine_material`
- `engine_kinetics`
- `engine_field`
- `engine_agents`
- `engine_imaging`
- `engine_acoustics`

## Downward Dependencies

- `engine_core` — Base types and contracts.
- `engine_identity` — Identity primitives.
- `engine_handle` — Stable references where spatially indexed.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Coordinates | World-space coordinate model. | `40_COORDINATES.md` |
| Transforms | Transform model for world-space relations. | `41_TRANSFORMS.md` |
| Spatial Addressing | Addressing model over world space. | `42_SPATIAL_ADDRESSING.md` |
| Spatial Relations | Canonical relations in world space. | `43_SPATIAL_RELATIONS.md` |

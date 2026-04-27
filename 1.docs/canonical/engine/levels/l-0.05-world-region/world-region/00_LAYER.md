# engine_world_region

## Stack Position

L-0.05. World Region

## Primary Role

Region and chunk substrate.

## Canonical Scope

Region model, chunk model, dirty regions, and region versioning.

## Boundary Rationale

Regioning is its own locality and partition pressure axis and must stay explicit for scheduling and apply legality.

## Canonical Consumers

- `engine_world`
- `engine_runtime`
- `engine_field`
- `engine_imaging`
- `engine_acoustics`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world_spatial` — Spatial substrate.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Region Model | Canonical region model. | `40_REGION_MODEL.md` |
| Chunk Model | Canonical chunk model. | `41_CHUNK_MODEL.md` |
| Dirty Regions | Dirty-region tracking substrate. | `42_DIRTY_REGIONS.md` |
| Region Versioning | Version model over region boundaries. | `43_REGION_VERSIONING.md` |

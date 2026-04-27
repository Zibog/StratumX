# engine_content

## Stack Position

L3.2. Resource Systems

## Primary Role

Resource and content processing family.

## Canonical Scope

Ingest, transform, metadata, resource products, runtime-ready pack products, and streaming-chunk products.

## Boundary Rationale

Content processing is a pipeline/resource pressure axis and should stay separate from simulation, model systems, runtime resource ownership, and synthesis systems.

## Canonical Consumers

- `engine_startup`
- `engine_generation`
- `engine_imaging`
- `engine_acoustics`

Runtime resource services consume startup-mediated or externally prepared runtime-pack inputs shaped by this crate, but that relation is product-level rather than a direct crate dependency.

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_material` — Shared world property descriptors for runtime-ready products.
- `engine_world_region` — Region/chunk descriptors for pack products.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Ingest | Source-side ingest model. | `40_INGEST.md` |
| Transform | Resource transform and preparation model. | `41_TRANSFORM.md` |
| Metadata | Metadata extraction and descriptor model. | `42_METADATA.md` |
| Resource Products | Canonical products emitted by content processing. | `43_RESOURCE_PRODUCTS.md` |
| Runtime Pack Products | Runtime-ready pack and bundle product model. | `44_RUNTIME_PACK_PRODUCTS.md` |
| Streaming Chunk Products | Region/chunk keyed runtime product model. | `45_STREAMING_CHUNK_PRODUCTS.md` |

## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.

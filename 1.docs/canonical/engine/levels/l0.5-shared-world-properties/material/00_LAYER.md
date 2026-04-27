# engine_material

## Stack Position

L0.5. Shared World Property Substrate

## Primary Role

Shared world property substrate.

## Canonical Scope

Material descriptors, physical property domains, response profiles, and lookup model shared across upper families.

## Boundary Rationale

Material truth is cross-domain and should not live as a regular simulation family. It is a read-heavy substrate used by simulation and synthesis families.

## Canonical Consumers

- `engine_kinetics`
- `engine_field`
- `engine_imaging`
- `engine_acoustics`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_handle` — Stable references where material instances are addressed.
- `engine_world` — World-bound material context.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Material Descriptors | Canonical material description model. | `40_MATERIAL_DESCRIPTORS.md` |
| Property Domains | Domains of physical, thermal, fluid, structural, acoustic, and appearance properties. | `41_PROPERTY_DOMAINS.md` |
| Response Profiles | Shared response profiles consumed by upper families. | `42_RESPONSE_PROFILES.md` |
| Lookup Model | Parallel-safe material lookup model. | `43_LOOKUP_MODEL.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.

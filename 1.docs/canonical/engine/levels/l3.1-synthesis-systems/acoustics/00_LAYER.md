# engine_acoustics

## Stack Position

L3.1. Synthesis Systems

## Primary Role

Acoustic synthesis family.

## Canonical Scope

Propagation, reflection, occlusion, material response, acoustic outputs, and runtime resource-service integration for acoustic assets and buffers.

## Boundary Rationale

Acoustic synthesis is a heavy perception-facing compute class and should stay above critical simulation while remaining tightly coupled to world and material truth.

## Canonical Consumers

- `engine_startup`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_ecs` — ECS substrate.
- `engine_material` — Shared world property substrate.
- `engine_world_spatial` — Spatial substrate.
- `engine_world_region` — Region substrate.
- `engine_residency_control` — Runtime residency control.
- `engine_transfer_control` — Runtime transfer control.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Propagation | Acoustic propagation model. | `40_PROPAGATION.md` |
| Reflection and Occlusion | Reflection and occlusion products. | `41_REFLECTION_AND_OCCLUSION.md` |
| Material Response | Acoustic material response model. | `42_MATERIAL_RESPONSE.md` |
| Acoustic Outputs | Final acoustic output model. | `43_ACOUSTIC_OUTPUTS.md` |
| Runtime Buffering | Runtime acoustic buffering model. | `44_RUNTIME_BUFFERING.md` |
| Streaming Audio Residency | Runtime residency consumption model for acoustic assets. | `45_STREAMING_AUDIO_RESIDENCY.md` |

## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.

# engine_startup

## Stack Position

L4. Startup

## Primary Role

Startup and assembly.

## Canonical Scope

Bootstrap, profile selection, network role selection, runtime wiring, resource/network service wiring, tracing initialization, and public `L5` bridge export surface publication.

## Boundary Rationale

Startup is the highest engine-owned layer because it selects the runtime profile, selects legal roles, wires families and services, publishes the narrow public bridge bind for `L5`, and launches execution without becoming part of world truth or runtime law.

## Canonical Consumers

- No engine-owned consumer above this layer. This crate terminates at the top of the engine stack.
- External canonical bridge consumer: `L5` binds only to the public export surfaces published here.

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_runtime` — Runtime constitution.
- `engine_runtime_headless` — Headless runtime profile.
- `engine_runtime_realtime` — Realtime runtime profile.
- `engine_stream_control` — Runtime stream service wiring.
- `engine_residency_control` — Runtime residency service wiring.
- `engine_memory_control` — Runtime memory service wiring.
- `engine_transfer_control` — Runtime transfer service wiring.
- `engine_kinetics` — Critical simulation family wiring.
- `engine_field` — Critical simulation family wiring.
- `engine_agents` — Critical simulation family wiring.
- `engine_net_transport` — Network transport wiring.
- `engine_net_sync` — Network sync wiring.
- `engine_net_latency` — Network latency wiring.
- `engine_inference` — Model system wiring.
- `engine_generation` — Model system wiring.
- `engine_imaging` — Synthesis system wiring.
- `engine_acoustics` — Synthesis system wiring.
- `engine_content` — Resource system wiring.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Bootstrap | Engine bootstrap sequence. | `40_BOOTSTRAP.md` |
| Profile Selection | Runtime profile selection model. | `41_PROFILE_SELECTION.md` |
| Runtime Wiring | Wiring of runtime and family surfaces. | `42_RUNTIME_WIRING.md` |
| Tracing Initialization | Startup tracing and diagnostics initialization. | `43_TRACING_INITIALIZATION.md` |
| Network Role Selection | Legal network role selection model. | `44_NETWORK_ROLE_SELECTION.md` |
| Resource Service Wiring | Wiring of runtime resource services. | `45_RESOURCE_SERVICE_WIRING.md` |
| L5 Bridge Export Surfaces | Public export surfaces published for the external bridge bind. | `46_L5_BRIDGE_EXPORT_SURFACES.md` |
| L5 Export Epoch And Invalidation | Public bridge epoch and invalidation model. | `47_L5_EXPORT_EPOCH_AND_INVALIDATION.md` |

## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.

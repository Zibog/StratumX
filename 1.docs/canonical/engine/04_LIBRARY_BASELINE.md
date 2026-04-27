# External Library Baseline

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Crate | External Dependencies |
|---|---|
| engine_core | glam, serde, thiserror |
| engine_identity | slotmap, serde |
| engine_handle | slotmap, serde |
| engine_storage_layout | smallvec, bitflags, serde |
| engine_storage_access | smallvec, bitflags |
| engine_storage_mutation | smallvec, bitflags |
| engine_ecs_registry | serde |
| engine_ecs_query | smallvec, bitflags |
| engine_ecs | serde |
| engine_world_spatial | glam, serde |
| engine_world_region | serde, bitflags |
| engine_world | serde, bincode, glam |
| engine_material | serde, bitflags, smallvec |
| engine_runtime | tracing, rayon, crossbeam, crossbeam-deque, parking_lot, smallvec |
| engine_runtime_headless | tracing, smallvec |
| engine_runtime_realtime | tracing, smallvec |
| engine_stream_control | crossbeam, smallvec, tracing |
| engine_residency_control | bitflags, smallvec, tracing |
| engine_memory_control | parking_lot, smallvec, tracing, bumpalo, mimalloc |
| engine_transfer_control | bytemuck, smallvec, tracing |
| engine_kinetics | rapier3d, glam, serde, bitflags, smallvec, tracing |
| engine_field | glam, serde, bitflags, smallvec, tracing |
| engine_agents | glam, serde, bitflags, smallvec, tracing |
| engine_net_transport | quinn, bytes, bitflags, smallvec, tracing |
| engine_net_sync | serde, bitflags, smallvec, tracing |
| engine_net_latency | bitflags, smallvec, tracing |
| engine_inference | serde, serde_json, thiserror, smallvec, tracing |
| engine_generation | serde, serde_json, thiserror, smallvec, tracing |
| engine_imaging | wgpu, bytemuck, glam, serde, smallvec, tracing |
| engine_acoustics | glam, serde, smallvec, bitflags, tracing |
| engine_content | walkdir, image, gltf, serde, thiserror, smallvec, tracing, zstd |
| engine_startup | tracing, tracing-subscriber, serde, serde_json, backtrace |

## Canonical Adjuncts

The following adjuncts are implementation-legal where the owning crate documents them explicitly:

- `criterion` for microbench and regression-gate proof;
- `loom` for runtime-owned concurrent primitive validation;
- `tracy-client` for explicit frame and phase markers;
- `io-uring` as a Linux-only optional IO backend beneath `engine_stream_control`;
- seekable zstd pack products beneath `engine_content` and `engine_stream_control`;
- KTX2/Basis-class GPU-ready texture products beneath `engine_content` and `engine_residency_control`.

Each crate-level libraries document defines exact role mapping for every dependency.

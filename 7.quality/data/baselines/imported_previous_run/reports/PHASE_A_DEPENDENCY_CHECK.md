# Phase A — Canonical Dependency Check

- Dependency legality verdict: **PASS**
- Crates checked: **32**
- Dependency failures: **0**

| Crate | Canonical deps | Actual deps | Status |
|---|---|---|---|
| engine_core | none | none | pass |
| engine_identity | `engine_core` | `engine_core` | pass |
| engine_handle | `engine_core`, `engine_identity` | `engine_core`, `engine_identity` | pass |
| engine_storage_layout | `engine_core` | `engine_core` | pass |
| engine_storage_access | `engine_core`, `engine_handle`, `engine_storage_layout` | `engine_core`, `engine_handle`, `engine_storage_layout` | pass |
| engine_storage_mutation | `engine_core`, `engine_handle`, `engine_storage_access`, `engine_storage_layout` | `engine_core`, `engine_handle`, `engine_storage_access`, `engine_storage_layout` | pass |
| engine_ecs_registry | `engine_core`, `engine_handle`, `engine_identity`, `engine_storage_layout` | `engine_core`, `engine_handle`, `engine_identity`, `engine_storage_layout` | pass |
| engine_ecs_query | `engine_core`, `engine_ecs_registry`, `engine_handle`, `engine_identity`, `engine_storage_access` | `engine_core`, `engine_ecs_registry`, `engine_handle`, `engine_identity`, `engine_storage_access` | pass |
| engine_ecs | `engine_core`, `engine_ecs_query`, `engine_ecs_registry`, `engine_handle`, `engine_identity`, `engine_storage_access`, `engine_storage_layout`, `engine_storage_mutation` | `engine_core`, `engine_ecs_query`, `engine_ecs_registry`, `engine_handle`, `engine_identity`, `engine_storage_access`, `engine_storage_layout`, `engine_storage_mutation` | pass |
| engine_world_spatial | `engine_core`, `engine_handle`, `engine_identity` | `engine_core`, `engine_handle`, `engine_identity` | pass |
| engine_world_region | `engine_core`, `engine_world_spatial` | `engine_core`, `engine_world_spatial` | pass |
| engine_world | `engine_core`, `engine_ecs`, `engine_handle`, `engine_world_region`, `engine_world_spatial` | `engine_core`, `engine_ecs`, `engine_handle`, `engine_world_region`, `engine_world_spatial` | pass |
| engine_material | `engine_core`, `engine_handle`, `engine_world` | `engine_core`, `engine_handle`, `engine_world` | pass |
| engine_runtime | `engine_core`, `engine_ecs`, `engine_handle`, `engine_world`, `engine_world_region` | `engine_core`, `engine_ecs`, `engine_handle`, `engine_world`, `engine_world_region` | pass |
| engine_runtime_headless | `engine_ecs`, `engine_runtime`, `engine_world` | `engine_ecs`, `engine_runtime`, `engine_world` | pass |
| engine_runtime_realtime | `engine_ecs`, `engine_runtime`, `engine_world` | `engine_ecs`, `engine_runtime`, `engine_world` | pass |
| engine_stream_control | `engine_core`, `engine_handle`, `engine_runtime`, `engine_world`, `engine_world_region` | `engine_core`, `engine_handle`, `engine_runtime`, `engine_world`, `engine_world_region` | pass |
| engine_residency_control | `engine_core`, `engine_handle`, `engine_memory_control`, `engine_stream_control`, `engine_world`, `engine_world_region` | `engine_core`, `engine_handle`, `engine_memory_control`, `engine_stream_control`, `engine_world`, `engine_world_region` | pass |
| engine_memory_control | `engine_core`, `engine_handle`, `engine_runtime`, `engine_storage_layout`, `engine_world` | `engine_core`, `engine_handle`, `engine_runtime`, `engine_storage_layout`, `engine_world` | pass |
| engine_transfer_control | `engine_core`, `engine_handle`, `engine_memory_control`, `engine_stream_control` | `engine_core`, `engine_handle`, `engine_memory_control`, `engine_stream_control` | pass |
| engine_kinetics | `engine_core`, `engine_material`, `engine_memory_control`, `engine_runtime`, `engine_world`, `engine_world_region`, `engine_world_spatial` | `engine_core`, `engine_material`, `engine_memory_control`, `engine_runtime`, `engine_world`, `engine_world_region`, `engine_world_spatial` | pass |
| engine_field | `engine_core`, `engine_material`, `engine_memory_control`, `engine_runtime`, `engine_world`, `engine_world_region`, `engine_world_spatial` | `engine_core`, `engine_material`, `engine_memory_control`, `engine_runtime`, `engine_world`, `engine_world_region`, `engine_world_spatial` | pass |
| engine_agents | `engine_core`, `engine_ecs`, `engine_memory_control`, `engine_runtime`, `engine_world`, `engine_world_region` | `engine_core`, `engine_ecs`, `engine_memory_control`, `engine_runtime`, `engine_world`, `engine_world_region` | pass |
| engine_net_transport | `engine_core`, `engine_handle`, `engine_runtime` | `engine_core`, `engine_handle`, `engine_runtime` | pass |
| engine_net_sync | `engine_core`, `engine_handle`, `engine_identity`, `engine_net_transport`, `engine_runtime`, `engine_world`, `engine_world_region` | `engine_core`, `engine_handle`, `engine_identity`, `engine_net_transport`, `engine_runtime`, `engine_world`, `engine_world_region` | pass |
| engine_net_latency | `engine_core`, `engine_handle`, `engine_net_transport`, `engine_runtime`, `engine_world`, `engine_world_region` | `engine_core`, `engine_handle`, `engine_net_transport`, `engine_runtime`, `engine_world`, `engine_world_region` | pass |
| engine_inference | `engine_core`, `engine_ecs`, `engine_world` | `engine_core`, `engine_ecs`, `engine_world` | pass |
| engine_generation | `engine_core`, `engine_inference`, `engine_world` | `engine_core`, `engine_inference`, `engine_world` | pass |
| engine_imaging | `engine_core`, `engine_ecs`, `engine_material`, `engine_residency_control`, `engine_transfer_control`, `engine_world`, `engine_world_region`, `engine_world_spatial` | `engine_core`, `engine_ecs`, `engine_material`, `engine_residency_control`, `engine_transfer_control`, `engine_world`, `engine_world_region`, `engine_world_spatial` | pass |
| engine_acoustics | `engine_core`, `engine_ecs`, `engine_material`, `engine_residency_control`, `engine_transfer_control`, `engine_world`, `engine_world_region`, `engine_world_spatial` | `engine_core`, `engine_ecs`, `engine_material`, `engine_residency_control`, `engine_transfer_control`, `engine_world`, `engine_world_region`, `engine_world_spatial` | pass |
| engine_content | `engine_core`, `engine_material`, `engine_world_region` | `engine_core`, `engine_material`, `engine_world_region` | pass |
| engine_startup | `engine_acoustics`, `engine_agents`, `engine_content`, `engine_core`, `engine_field`, `engine_generation`, `engine_imaging`, `engine_inference`, `engine_kinetics`, `engine_memory_control`, `engine_net_latency`, `engine_net_sync`, `engine_net_transport`, `engine_residency_control`, `engine_runtime`, `engine_runtime_headless`, `engine_runtime_realtime`, `engine_stream_control`, `engine_transfer_control`, `engine_world` | `engine_acoustics`, `engine_agents`, `engine_content`, `engine_core`, `engine_field`, `engine_generation`, `engine_imaging`, `engine_inference`, `engine_kinetics`, `engine_memory_control`, `engine_net_latency`, `engine_net_sync`, `engine_net_transport`, `engine_residency_control`, `engine_runtime`, `engine_runtime_headless`, `engine_runtime_realtime`, `engine_stream_control`, `engine_transfer_control`, `engine_world` | pass |

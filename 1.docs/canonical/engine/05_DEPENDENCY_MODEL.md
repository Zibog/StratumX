# Dependency Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical Dependency Overview

- `engine_core` -> none
- `engine_identity` -> engine_core
- `engine_handle` -> engine_core, engine_identity
- `engine_storage_layout` -> engine_core
- `engine_storage_access` -> engine_core, engine_handle, engine_storage_layout
- `engine_storage_mutation` -> engine_core, engine_handle, engine_storage_layout, engine_storage_access
- `engine_ecs_registry` -> engine_core, engine_identity, engine_handle, engine_storage_layout
- `engine_ecs_query` -> engine_core, engine_identity, engine_handle, engine_storage_access, engine_ecs_registry
- `engine_ecs` -> engine_core, engine_identity, engine_handle, engine_storage_layout, engine_storage_access, engine_storage_mutation, engine_ecs_registry, engine_ecs_query
- `engine_world_spatial` -> engine_core, engine_identity, engine_handle
- `engine_world_region` -> engine_core, engine_world_spatial
- `engine_world` -> engine_core, engine_handle, engine_ecs, engine_world_spatial, engine_world_region
- `engine_material` -> engine_core, engine_handle, engine_world
- `engine_runtime` -> engine_core, engine_handle, engine_ecs, engine_world, engine_world_region
- `engine_runtime_headless` -> engine_runtime, engine_world, engine_ecs
- `engine_runtime_realtime` -> engine_runtime, engine_world, engine_ecs
- `engine_stream_control` -> engine_core, engine_handle, engine_world, engine_world_region, engine_runtime
- `engine_residency_control` -> engine_core, engine_handle, engine_world, engine_world_region, engine_stream_control, engine_memory_control
- `engine_memory_control` -> engine_core, engine_handle, engine_storage_layout, engine_world, engine_runtime
- `engine_transfer_control` -> engine_core, engine_handle, engine_stream_control, engine_memory_control
- `engine_kinetics` -> engine_core, engine_world, engine_material, engine_world_spatial, engine_world_region, engine_runtime, engine_memory_control
- `engine_field` -> engine_core, engine_world, engine_material, engine_world_spatial, engine_world_region, engine_runtime, engine_memory_control
- `engine_agents` -> engine_core, engine_ecs, engine_world, engine_world_region, engine_runtime, engine_memory_control
- `engine_net_transport` -> engine_core, engine_handle, engine_runtime
- `engine_net_sync` -> engine_core, engine_identity, engine_handle, engine_world, engine_world_region, engine_runtime, engine_net_transport
- `engine_net_latency` -> engine_core, engine_handle, engine_world, engine_world_region, engine_runtime, engine_net_transport
- `engine_inference` -> engine_core, engine_world, engine_ecs
- `engine_generation` -> engine_core, engine_world, engine_inference
- `engine_imaging` -> engine_core, engine_world, engine_ecs, engine_material, engine_world_spatial, engine_world_region, engine_residency_control, engine_transfer_control
- `engine_acoustics` -> engine_core, engine_world, engine_ecs, engine_material, engine_world_spatial, engine_world_region, engine_residency_control, engine_transfer_control
- `engine_content` -> engine_core, engine_material, engine_world_region
- `engine_startup` -> engine_core, engine_world, engine_runtime, engine_runtime_headless, engine_runtime_realtime, engine_stream_control, engine_residency_control, engine_memory_control, engine_transfer_control, engine_kinetics, engine_field, engine_agents, engine_net_transport, engine_net_sync, engine_net_latency, engine_inference, engine_generation, engine_imaging, engine_acoustics, engine_content

## Canonical Dependency Law

The stack is downward-dependent: every higher layer consumes lower substrates, lower services, or startup-mediated legal inputs without reversing world ownership, execution ownership, runtime resource ownership, or network ownership.

`engine_content` may produce prepared resource products and runtime-pack products, but `L1.5` runtime resource services consume those products only through `engine_startup`-mediated or externally prepared inputs. Product compatibility does not create a direct crate dependency. No `L1.5` crate may depend upward into `L3.x`.

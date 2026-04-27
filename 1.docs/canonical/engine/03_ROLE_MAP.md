# Role Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Level | Crate | Primary Role |
|---|---|---|
| L-1. Foundation | engine_core | Minimal engine foundation. |
| L-0.9. Identity Primitives | engine_identity | Identity substrate for entities and components. |
| L-0.8. Handle System | engine_handle | Stable temporal references over identity-bearing structures. |
| L-0.7. Storage Layout | engine_storage_layout | Physical organization of storage. |
| L-0.6. Storage Access | engine_storage_access | Deterministic read/write access model over storage layout. |
| L-0.5. Storage Mutation | engine_storage_mutation | Staging and packaging of storage-side mutations. |
| L-0.4. ECS Registry | engine_ecs_registry | Structural ECS registration truth. |
| L-0.3. ECS Query | engine_ecs_query | Deterministic query and traversal model. |
| L-0.2. ECS Assembly | engine_ecs | Assembled ECS substrate. |
| L-0.1. World Spatial | engine_world_spatial | World-specific spatial substrate. |
| L-0.05. World Region | engine_world_region | Region and chunk substrate. |
| L0. World Truth | engine_world | Authoritative world owner. |
| L0.5. Shared World Property Substrate | engine_material | Shared world property substrate. |
| L1. Runtime Kernel | engine_runtime | Execution constitution and global parallel ownership. |
| L1. Runtime Kernel | engine_runtime_headless | Headless simulation runtime profile. |
| L1. Runtime Kernel | engine_runtime_realtime | Realtime interactive runtime profile. |
| L1.5. Runtime Resource Services | engine_stream_control | Runtime streaming, activation, and IO scheduling control. |
| L1.5. Runtime Resource Services | engine_residency_control | CPU/GPU residency ownership and pressure visibility. |
| L1.5. Runtime Resource Services | engine_memory_control | Allocators, pools, lifetimes, and pressure-response ownership. |
| L1.5. Runtime Resource Services | engine_transfer_control | Decode, staging, upload, and fence-bound transfer control. |
| L2. Critical Simulation Families | engine_kinetics | Local dynamics and contact simulation family. |
| L2. Critical Simulation Families | engine_field | Distributed physical field simulation family. |
| L2. Critical Simulation Families | engine_agents | Agent and society simulation family. |
| L2.5. Network Runtime Services | engine_net_transport | Gameplay network transport service. |
| L2.5. Network Runtime Services | engine_net_sync | Interest, snapshot, delta, and ack service. |
| L2.5. Network Runtime Services | engine_net_latency | Prediction, reconciliation, rewind, and latency-control service. |
| L3.0. Model Systems | engine_inference | Model inference boundary. |
| L3.0. Model Systems | engine_generation | Model-driven generation boundary. |
| L3.1. Synthesis Systems | engine_imaging | Image synthesis family. |
| L3.1. Synthesis Systems | engine_acoustics | Acoustic synthesis family. |
| L3.2. Resource Systems | engine_content | Resource and content processing family. |
| L4. Startup | engine_startup | Startup and assembly. |

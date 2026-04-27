# Canonical Stack

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

```text
L-1. Foundation
    engine_core
L-0.9. Identity Primitives
    engine_identity
L-0.8. Handle System
    engine_handle
L-0.7. Storage Layout
    engine_storage_layout
L-0.6. Storage Access
    engine_storage_access
L-0.5. Storage Mutation
    engine_storage_mutation
L-0.4. ECS Registry
    engine_ecs_registry
L-0.3. ECS Query
    engine_ecs_query
L-0.2. ECS Assembly
    engine_ecs
L-0.1. World Spatial
    engine_world_spatial
L-0.05. World Region
    engine_world_region
L0. World Truth
    engine_world
L0.5. Shared World Property Substrate
    engine_material
L1. Runtime Kernel
    engine_runtime
    engine_runtime_headless
    engine_runtime_realtime
L1.5. Runtime Resource Services
    engine_stream_control
    engine_residency_control
    engine_memory_control
    engine_transfer_control
L2. Critical Simulation Families
    engine_kinetics
    engine_field
    engine_agents
L2.5. Network Runtime Services
    engine_net_transport
    engine_net_sync
    engine_net_latency
L3.0. Model Systems
    engine_inference
    engine_generation
L3.1. Synthesis Systems
    engine_imaging
    engine_acoustics
L3.2. Resource Systems
    engine_content
L4. Startup
    engine_startup
```

## Summary

- `L-1..L0` define substrate and world truth.
- `L0.5` defines shared world properties used across upper families.
- `L1` owns execution constitution and global parallel ownership.
- `L1.5` owns runtime streaming, residency, memory, and transfer control.
- `L2` owns critical world-progression simulation.
- `L2.5` owns online transport, sync, and latency-control services.
- `L3.0` owns model-facing compute.
- `L3.1` owns perception-facing synthesis.
- `L3.2` owns offline and startup-facing resource processing.
- `L4` owns bootstrap and launch.

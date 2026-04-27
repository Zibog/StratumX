# Phase A — Canonical Stack and Dependency Closure

- Canonical engine crates required: **32**
- Workspace members declared: **44**
- Canonical closure verdict: **FAIL**

| Crate | Path | In workspace | Cargo.toml | src/lib.rs | tests/ | Status |
|---|---|---|---|---|---|---|
| engine_core | `2.engine/l-1-foundation/core/engine_core` | no | no | no | no | fail |
| engine_identity | `2.engine/l-0.9-identity/identity/engine_identity` | no | no | no | no | fail |
| engine_handle | `2.engine/l-0.8-handle/handle/engine_handle` | no | no | no | no | fail |
| engine_storage_layout | `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout` | no | no | no | no | fail |
| engine_storage_access | `2.engine/l-0.6-storage-access/storage-access/engine_storage_access` | no | no | no | no | fail |
| engine_storage_mutation | `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation` | no | no | no | no | fail |
| engine_ecs_registry | `2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry` | no | no | no | no | fail |
| engine_ecs_query | `2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query` | no | no | no | no | fail |
| engine_ecs | `2.engine/l-0.2-ecs-assembly/ecs/engine_ecs` | no | no | no | no | fail |
| engine_world_spatial | `2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial` | no | no | no | no | fail |
| engine_world_region | `2.engine/l-0.05-world-region/world-region/engine_world_region` | no | no | no | no | fail |
| engine_world | `2.engine/l0-world-truth/world/engine_world` | no | no | no | no | fail |
| engine_material | `2.engine/l0.5-shared-world-property-substrate/material/engine_material` | no | no | no | no | fail |
| engine_runtime | `2.engine/l1-runtime-kernel/runtime/engine_runtime` | no | no | no | no | fail |
| engine_runtime_headless | `2.engine/l1-runtime-kernel/runtime-headless/engine_runtime_headless` | no | no | no | no | fail |
| engine_runtime_realtime | `2.engine/l1-runtime-kernel/runtime-realtime/engine_runtime_realtime` | no | no | no | no | fail |
| engine_stream_control | `2.engine/l1.5-runtime-resource-services/stream-control/engine_stream_control` | no | no | no | no | fail |
| engine_residency_control | `2.engine/l1.5-runtime-resource-services/residency-control/engine_residency_control` | no | no | no | no | fail |
| engine_memory_control | `2.engine/l1.5-runtime-resource-services/memory-control/engine_memory_control` | no | no | no | no | fail |
| engine_transfer_control | `2.engine/l1.5-runtime-resource-services/transfer-control/engine_transfer_control` | no | no | no | no | fail |
| engine_kinetics | `2.engine/l2-critical-simulation-families/kinetics/engine_kinetics` | no | no | no | no | fail |
| engine_field | `2.engine/l2-critical-simulation-families/field/engine_field` | no | no | no | no | fail |
| engine_agents | `2.engine/l2-critical-simulation-families/agents/engine_agents` | no | no | no | no | fail |
| engine_net_transport | `2.engine/l2.5-network-runtime-services/net-transport/engine_net_transport` | no | no | no | no | fail |
| engine_net_sync | `2.engine/l2.5-network-runtime-services/net-sync/engine_net_sync` | no | no | no | no | fail |
| engine_net_latency | `2.engine/l2.5-network-runtime-services/net-latency/engine_net_latency` | no | no | no | no | fail |
| engine_inference | `2.engine/l3.0-model-systems/inference/engine_inference` | no | no | no | no | fail |
| engine_generation | `2.engine/l3.0-model-systems/generation/engine_generation` | no | no | no | no | fail |
| engine_imaging | `2.engine/l3.1-synthesis-systems/imaging/engine_imaging` | no | no | no | no | fail |
| engine_acoustics | `2.engine/l3.1-synthesis-systems/acoustics/engine_acoustics` | no | no | no | no | fail |
| engine_content | `2.engine/l3.2-resource-systems/content/engine_content` | no | no | no | no | fail |
| engine_startup | `2.engine/l4-startup/startup/engine_startup` | no | no | no | no | fail |

## Missing workspace members

- `2.engine/l-1-foundation/core/engine_core`
- `2.engine/l-0.9-identity/identity/engine_identity`
- `2.engine/l-0.8-handle/handle/engine_handle`
- `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout`
- `2.engine/l-0.6-storage-access/storage-access/engine_storage_access`
- `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation`
- `2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry`
- `2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query`
- `2.engine/l-0.2-ecs-assembly/ecs/engine_ecs`
- `2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial`
- `2.engine/l-0.05-world-region/world-region/engine_world_region`
- `2.engine/l0-world-truth/world/engine_world`
- `2.engine/l0.5-shared-world-property-substrate/material/engine_material`
- `2.engine/l1-runtime-kernel/runtime/engine_runtime`
- `2.engine/l1-runtime-kernel/runtime-headless/engine_runtime_headless`
- `2.engine/l1-runtime-kernel/runtime-realtime/engine_runtime_realtime`
- `2.engine/l1.5-runtime-resource-services/stream-control/engine_stream_control`
- `2.engine/l1.5-runtime-resource-services/residency-control/engine_residency_control`
- `2.engine/l1.5-runtime-resource-services/memory-control/engine_memory_control`
- `2.engine/l1.5-runtime-resource-services/transfer-control/engine_transfer_control`
- `2.engine/l2-critical-simulation-families/kinetics/engine_kinetics`
- `2.engine/l2-critical-simulation-families/field/engine_field`
- `2.engine/l2-critical-simulation-families/agents/engine_agents`
- `2.engine/l2.5-network-runtime-services/net-transport/engine_net_transport`
- `2.engine/l2.5-network-runtime-services/net-sync/engine_net_sync`
- `2.engine/l2.5-network-runtime-services/net-latency/engine_net_latency`
- `2.engine/l3.0-model-systems/inference/engine_inference`
- `2.engine/l3.0-model-systems/generation/engine_generation`
- `2.engine/l3.1-synthesis-systems/imaging/engine_imaging`
- `2.engine/l3.1-synthesis-systems/acoustics/engine_acoustics`
- `2.engine/l3.2-resource-systems/content/engine_content`
- `2.engine/l4-startup/startup/engine_startup`

## Missing required files

- `2.engine/l-1-foundation/core/engine_core`
- `2.engine/l-0.9-identity/identity/engine_identity`
- `2.engine/l-0.8-handle/handle/engine_handle`
- `2.engine/l-0.7-storage-layout/storage-layout/engine_storage_layout`
- `2.engine/l-0.6-storage-access/storage-access/engine_storage_access`
- `2.engine/l-0.5-storage-mutation/storage-mutation/engine_storage_mutation`
- `2.engine/l-0.4-ecs-registry/ecs-registry/engine_ecs_registry`
- `2.engine/l-0.3-ecs-query/ecs-query/engine_ecs_query`
- `2.engine/l-0.2-ecs-assembly/ecs/engine_ecs`
- `2.engine/l-0.1-world-spatial/world-spatial/engine_world_spatial`
- `2.engine/l-0.05-world-region/world-region/engine_world_region`
- `2.engine/l0-world-truth/world/engine_world`
- `2.engine/l0.5-shared-world-property-substrate/material/engine_material`
- `2.engine/l1-runtime-kernel/runtime/engine_runtime`
- `2.engine/l1-runtime-kernel/runtime-headless/engine_runtime_headless`
- `2.engine/l1-runtime-kernel/runtime-realtime/engine_runtime_realtime`
- `2.engine/l1.5-runtime-resource-services/stream-control/engine_stream_control`
- `2.engine/l1.5-runtime-resource-services/residency-control/engine_residency_control`
- `2.engine/l1.5-runtime-resource-services/memory-control/engine_memory_control`
- `2.engine/l1.5-runtime-resource-services/transfer-control/engine_transfer_control`
- `2.engine/l2-critical-simulation-families/kinetics/engine_kinetics`
- `2.engine/l2-critical-simulation-families/field/engine_field`
- `2.engine/l2-critical-simulation-families/agents/engine_agents`
- `2.engine/l2.5-network-runtime-services/net-transport/engine_net_transport`
- `2.engine/l2.5-network-runtime-services/net-sync/engine_net_sync`
- `2.engine/l2.5-network-runtime-services/net-latency/engine_net_latency`
- `2.engine/l3.0-model-systems/inference/engine_inference`
- `2.engine/l3.0-model-systems/generation/engine_generation`
- `2.engine/l3.1-synthesis-systems/imaging/engine_imaging`
- `2.engine/l3.1-synthesis-systems/acoustics/engine_acoustics`
- `2.engine/l3.2-resource-systems/content/engine_content`
- `2.engine/l4-startup/startup/engine_startup`

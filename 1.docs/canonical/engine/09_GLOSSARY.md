# Glossary

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical Terms

| Term | Class | Scope | Canonical Document | Meaning |
|---|---|---|---|---|
| lower stack | umbrella term | L-1..L0 | `02_CANONICAL_STACK.md` | Fixed substrate from foundation through authoritative world truth. |
| upper stack | umbrella term | L0.5..L4 | `02_CANONICAL_STACK.md` | Everything above the fixed lower stack. |
| runtime family | umbrella term | L1 | `02_CANONICAL_STACK.md` | `engine_runtime`, `engine_runtime_headless`, and `engine_runtime_realtime` taken together. |
| critical simulation families | umbrella term | L2 | `02_CANONICAL_STACK.md` | `engine_kinetics`, `engine_field`, and `engine_agents` taken together. |
| service layers | umbrella term | L3.0..L3.2 | `02_CANONICAL_STACK.md` | Model systems, synthesis systems, and resource systems taken together. |
| runtime resource services | umbrella term | L1.5 | `constitutions/STRATUMX_NAMING_AND_TERMINOLOGY_FREEZE.md` | `engine_stream_control`, `engine_residency_control`, `engine_memory_control`, and `engine_transfer_control` taken together. |
| network runtime services | umbrella term | L2.5 | `constitutions/STRATUMX_NAMING_AND_TERMINOLOGY_FREEZE.md` | `engine_net_transport`, `engine_net_sync`, and `engine_net_latency` taken together. |
| world instance | execution term | engine-wide | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md` | One authoritative `engine_world` instance together with its legal runtime authority and execution context. |
| runtime authority | execution term | L1 | `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md` | The single active runtime owner permitted to drive one world instance. |
| degradation decision | operational term | L3 / runtime-facing | `constitutions/STRATUMX_DEGRADATION_POLICY_LAW.md` | Explicit runtime-visible decision that reduces optional service-layer work while preserving canonical law. |
| compatibility law | persistence term | persistence / startup | `constitutions/STRATUMX_PERSISTENCE_COMPATIBILITY_LAW.md` | Canonical rule set that determines whether persisted payloads may restore into a legal engine assembly. |
| runtime-profile-safe restoration state | persistence term | world / startup | `constitutions/STRATUMX_DATA_AND_STATE_CONSTITUTION.md` | Minimal restoration package legal under a selected runtime profile. |
| profile-safe restoration selectors | persistence term | startup / restoration | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical selectors required to restore state under a legal runtime profile without restoring transient runtime internals. |
| startup-ready assembly decision set | configuration term | L4 startup | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Final validated startup product used to launch one legal runtime profile and enablement set. |
| legal units | configuration term | engine-wide | `constitutions/STRATUMX_ENABLEMENT_LEGALITY_MATRIX.md` | Full enablement scope used by configuration legality: lower stack, material substrate, runtime family, critical simulation families, service layers, and startup. |
| enablement configuration | configuration term | engine-wide | `constitutions/STRATUMX_CONFIGURATION_CONSTITUTION.md` | Configuration object that chooses optional legal units while preserving all mandatory legal units. |
| legal enablement set | configuration term | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical validated result of enablement configuration after legality checks. |
| always-on units | configuration term | engine-wide | `constitutions/STRATUMX_ENABLEMENT_LEGALITY_MATRIX.md` | Units mandatory in every legal engine assembly. |
| canonical stack version marker | persistence term | engine-wide | `constitutions/STRATUMX_STACK_VERSION_SOURCE.md` | Canonical version identity carried by persistence payloads and startup decisions to bind them to one engine canon revision. |
| required family markers | persistence term | persistence / restoration | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Required markers that declare which family identities and boundary revisions a payload depends on. |
| compatibility flags | persistence term | persistence / restoration | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Explicit flags that describe migration, strictness, and restoration legality conditions for a payload. |
| legal assembly decision | startup term | L4 startup | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonically valid startup decision proving one assembly is legal under runtime, enablement, and restoration law. |
| legal restoration decision | startup term | L4 startup | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonically valid startup decision proving one persistence payload may restore into one legal engine assembly. |
| stack version source | governance term | engine-wide | `constitutions/STRATUMX_STACK_VERSION_SOURCE.md` | Single source of truth for the canonical stack version marker. |
| runtime-pack inputs | resource term | startup / L1.5 | `05_DEPENDENCY_MODEL.md` | Startup-mediated or externally prepared runtime-pack products consumed by runtime resource services without creating an upward crate dependency into `engine_content`. |
| canonical consumers | navigation term | layer documents | `levels/l-1-foundation/core/00_LAYER.md`, `levels/l-0.9-identity/identity/00_LAYER.md`, and all other level-specific `00_LAYER.md` documents | Crates or layers that legally consume one canonical crate through dependency, runtime-governed execution, startup wiring, or validated product inputs. |
| benchmark-common-cpu | benchmark term | constitutions | `constitutions/STRATUMX_LOCKED_BASELINE_TABLE.md` | Capture-profile identifier for CPU-only common baseline rows that are not presentation-, backend-, or role-sensitive. |

## Level Entries

| Level | Canonical Document | Meaning |
|---|---|---|
| L-1. Foundation | `levels/l-1-foundation/00_LEVEL.md` | Base engine foundation. |
| L-0.9. Identity Primitives | `levels/l-0.9-identity/00_LEVEL.md` | Identity substrate. |
| L-0.8. Handle System | `levels/l-0.8-handle/00_LEVEL.md` | Stable temporal reference layer. |
| L-0.7. Storage Layout | `levels/l-0.7-storage-layout/00_LEVEL.md` | Physical storage organization. |
| L-0.6. Storage Access | `levels/l-0.6-storage-access/00_LEVEL.md` | Deterministic access law. |
| L-0.5. Storage Mutation | `levels/l-0.5-storage-mutation/00_LEVEL.md` | Staged storage mutation law. |
| L-0.4. ECS Registry | `levels/l-0.4-ecs-registry/00_LEVEL.md` | Structural ECS truth. |
| L-0.3. ECS Query | `levels/l-0.3-ecs-query/00_LEVEL.md` | Query and traversal law. |
| L-0.2. ECS Assembly | `levels/l-0.2-ecs-assembly/00_LEVEL.md` | Assembled ECS substrate. |
| L-0.1. World Spatial | `levels/l-0.1-world-spatial/00_LEVEL.md` | Spatial world substrate. |
| L-0.05. World Region | `levels/l-0.05-world-region/00_LEVEL.md` | Region and chunk substrate. |
| L0. World Truth | `levels/l0-world-truth/00_LEVEL.md` | Authoritative world truth. |
| L0.5. Shared World Property Substrate | `levels/l0.5-shared-world-properties/00_LEVEL.md` | Shared world property substrate. |
| L1. Runtime Kernel | `levels/l1-runtime-kernel/00_LEVEL.md` | Execution-owning runtime layer. |
| L1.5. Runtime Resource Services | `levels/l1.5-runtime-resource-services/00_LEVEL.md` | Runtime streaming, residency, memory, and transfer services. |
| L2. Critical Simulation Families | `levels/l2-critical-simulation/00_LEVEL.md` | Mandatory world-progression families. |
| L2.5. Network Runtime Services | `levels/l2.5-network-runtime-services/00_LEVEL.md` | Runtime transport, sync, and latency-control services. |
| L3.0. Model Systems | `levels/l3.0-model-systems/00_LEVEL.md` | Model-facing service layer. |
| L3.1. Synthesis Systems | `levels/l3.1-synthesis-systems/00_LEVEL.md` | Image and acoustic synthesis layer. |
| L3.2. Resource Systems | `levels/l3.2-resource-systems/00_LEVEL.md` | Resource processing layer. |
| L4. Startup | `levels/l4-startup/00_LEVEL.md` | Startup and assembly layer. |

## Crate Entries

| Crate | Level | Canonical Document | Meaning |
|---|---|---|---|
| engine_core | L-1. Foundation | `levels/l-1-foundation/core/00_LAYER.md` | Minimal engine foundation. |
| engine_identity | L-0.9. Identity Primitives | `levels/l-0.9-identity/identity/00_LAYER.md` | Identity substrate for entities and components. |
| engine_handle | L-0.8. Handle System | `levels/l-0.8-handle/handle/00_LAYER.md` | Stable temporal references. |
| engine_storage_layout | L-0.7. Storage Layout | `levels/l-0.7-storage-layout/storage-layout/00_LAYER.md` | Physical organization of storage. |
| engine_storage_access | L-0.6. Storage Access | `levels/l-0.6-storage-access/storage-access/00_LAYER.md` | Deterministic read/write access model. |
| engine_storage_mutation | L-0.5. Storage Mutation | `levels/l-0.5-storage-mutation/storage-mutation/00_LAYER.md` | Staged storage mutation model. |
| engine_ecs_registry | L-0.4. ECS Registry | `levels/l-0.4-ecs-registry/ecs-registry/00_LAYER.md` | Structural ECS registration truth. |
| engine_ecs_query | L-0.3. ECS Query | `levels/l-0.3-ecs-query/ecs-query/00_LAYER.md` | Deterministic ECS query model. |
| engine_ecs | L-0.2. ECS Assembly | `levels/l-0.2-ecs-assembly/ecs/00_LAYER.md` | Assembled ECS substrate. |
| engine_world_spatial | L-0.1. World Spatial | `levels/l-0.1-world-spatial/world-spatial/00_LAYER.md` | Spatial world substrate. |
| engine_world_region | L-0.05. World Region | `levels/l-0.05-world-region/world-region/00_LAYER.md` | Region and chunk substrate. |
| engine_world | L0. World Truth | `levels/l0-world-truth/world/00_LAYER.md` | Authoritative world owner. |
| engine_material | L0.5. Shared World Property Substrate | `levels/l0.5-shared-world-properties/material/00_LAYER.md` | Shared world property substrate. |
| engine_runtime | L1. Runtime Kernel | `levels/l1-runtime-kernel/runtime/00_LAYER.md` | Runtime constitution and execution owner. |
| engine_runtime_headless | L1. Runtime Kernel | `levels/l1-runtime-kernel/runtime-headless/00_LAYER.md` | Headless execution profile. |
| engine_runtime_realtime | L1. Runtime Kernel | `levels/l1-runtime-kernel/runtime-realtime/00_LAYER.md` | Realtime execution profile. |
| engine_stream_control | L1.5. Runtime Resource Services | `levels/l1.5-runtime-resource-services/stream-control/00_LAYER.md` | Runtime streaming, activation, and IO scheduling control. |
| engine_residency_control | L1.5. Runtime Resource Services | `levels/l1.5-runtime-resource-services/residency-control/00_LAYER.md` | CPU/GPU residency ownership and pressure visibility. |
| engine_memory_control | L1.5. Runtime Resource Services | `levels/l1.5-runtime-resource-services/memory-control/00_LAYER.md` | Allocators, pools, lifetimes, and pressure-response ownership. |
| engine_transfer_control | L1.5. Runtime Resource Services | `levels/l1.5-runtime-resource-services/transfer-control/00_LAYER.md` | Decode, staging, upload, and fence-bound transfer control. |
| engine_kinetics | L2. Critical Simulation Families | `levels/l2-critical-simulation/kinetics/00_LAYER.md` | Local dynamics and contact simulation family. |
| engine_field | L2. Critical Simulation Families | `levels/l2-critical-simulation/field/00_LAYER.md` | Distributed physical field simulation family. |
| engine_agents | L2. Critical Simulation Families | `levels/l2-critical-simulation/agents/00_LAYER.md` | Agent and society simulation family. |
| engine_net_transport | L2.5. Network Runtime Services | `levels/l2.5-network-runtime-services/net-transport/00_LAYER.md` | Gameplay network transport service. |
| engine_net_sync | L2.5. Network Runtime Services | `levels/l2.5-network-runtime-services/net-sync/00_LAYER.md` | Interest, snapshot, delta, and ack service. |
| engine_net_latency | L2.5. Network Runtime Services | `levels/l2.5-network-runtime-services/net-latency/00_LAYER.md` | Prediction, reconciliation, rewind, and latency-control service. |
| engine_inference | L3.0. Model Systems | `levels/l3.0-model-systems/inference/00_LAYER.md` | Model inference boundary. |
| engine_generation | L3.0. Model Systems | `levels/l3.0-model-systems/generation/00_LAYER.md` | Model-driven generation boundary. |
| engine_imaging | L3.1. Synthesis Systems | `levels/l3.1-synthesis-systems/imaging/00_LAYER.md` | Image synthesis family. |
| engine_acoustics | L3.1. Synthesis Systems | `levels/l3.1-synthesis-systems/acoustics/00_LAYER.md` | Acoustic synthesis family. |
| engine_content | L3.2. Resource Systems | `levels/l3.2-resource-systems/content/00_LAYER.md` | Resource processing family. |
| engine_startup | L4. Startup | `levels/l4-startup/startup/00_LAYER.md` | Startup and assembly owner. |

## Alias Terms

| Earlier / Alias Term | Canonical Term |
|---|---|
| `runtime family` | `runtime family` |
| `service systems` | `service layers` |
| `always-on families` | `always-on units` |
| `family enablement configuration` | `enablement configuration` |
| `startup-ready assembly decisions` | `startup-ready assembly decision set` |

## Historical File Alias Notes

These are historical literal file names preserved only for migration reading. They are not live canonical paths.

| Historical Literal File | Canonical Live File |
|---|---|
| `levels/l-1-foundation/core/40_MATH.md` | `levels/l-1-foundation/core/40_MATH_BACKBONE.md` |
| `levels/l-1-foundation/core/41_TYPES.md` | `levels/l-1-foundation/core/41_BASE_TYPES.md` |
| `levels/l-1-foundation/core/42_ERRORS.md` | `levels/l-1-foundation/core/42_ERROR_MODEL.md` |
| `levels/l-1-foundation/core/43_TRAITS.md` | `levels/l-1-foundation/core/43_CORE_CONTRACTS.md` |
| `levels/l-0.8-handle/handle/41_VALIDATION.md` | `levels/l-0.8-handle/handle/41_VALIDATION_MODEL.md` |
| `levels/l-0.8-handle/handle/42_INVALIDATION.md` | `levels/l-0.8-handle/handle/42_INVALIDATION_MODEL.md` |
| `levels/l-0.6-storage-access/storage-access/41_WRITE_VIEWS.md` | `levels/l-0.6-storage-access/storage-access/41_WRITE_WINDOWS.md` |


## Canonical Shape Classes

| Term | Class | Scope | Canonical Document | Meaning |
|---|---|---|---|---|
| TypeSet | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical grouped set of foundational public types exposed as one bounded surface. |
| ErrorSet | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical grouped set of public error classes exposed as one bounded surface. |
| Token | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical lightweight public identity or handle token. |
| Query | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical query-facing public selection or traversal noun. |
| Window | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical bounded public access or mutation window. |
| Batch | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical grouped transport unit for staged outputs, apply payloads, or bulk work. |
| Request | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical external or service-facing input request shape. |
| Plan | shape class | engine-wide | `constitutions/STRATUMX_CANONICAL_SHAPES.md` | Canonical ordered execution, launch, or assembly planning shape. |

## API Boundary Terms

| Term | Class | Scope | Canonical Document | Meaning |
|---|---|---|---|---|
| primary public boundary form | API term | engine-wide | `constitutions/STRATUMX_API_CONTRACT_LAW.md` | Legal canonical public posture class selected for one crate. |
| primary public boundary type | API term | engine-wide | `constitutions/STRATUMX_API_CONTRACT_LAW.md` | Concrete public type that instantiates the selected primary boundary form for one crate. |
| allowed boundary forms | API term | engine-wide | `constitutions/STRATUMX_API_CONTRACT_LAW.md` | Closed canonical set of legal boundary forms. |

## Performance Terms

| Term | Class | Scope | Canonical Document | Meaning |
|---|---|---|---|---|
| critical execution lane | performance term | engine-wide | `constitutions/STRATUMX_PERFORMANCE_CONSTITUTION.md` | A latency-sensitive, allocation-sensitive, cache-sensitive execution path inside a canonical crate that is subject to the strictest performance law. |
| performance-critical substrate | performance term | engine-wide | `constitutions/STRATUMX_CRATE_PERFORMANCE_BUDGETS.md` | A substrate crate whose dominant implementation burden is keeping critical execution lanes lean, deterministic, and locality-preserving. |
| traversal plan | performance term | lower stack | `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md` | Compiled, cacheable execution descriptor for one legal query signature. |
| dense execution handle | performance term | lower stack | `constitutions/STRATUMX_CRITICAL_EXECUTION_LANE_AND_TRAVERSAL_LAW.md` | Internal dense token used only inside one compiled traversal lane or batch. |
| simulation tier | performance term | L2 / runtime | `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md` | Frozen fidelity and cadence class that governs how one world subset is progressed. |
| segmented apply | execution term | L0 / L1 | `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md` | Region-scoped, island-scoped, family-tagged authoritative world integration instead of monolithic whole-world apply. |
| residency hysteresis | resource term | L1.5 | `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md` | Mandatory anti-oscillation rule for hot/warm/cold resource state transitions. |
| quantization class | network term | L2.5 | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md` | Frozen encoding class that every exported sync field must declare. |
| rewindable domain | network term | L2.5 | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md` | Closed domain set that may legally participate in short-history rewind. |
| visibility hierarchy | synthesis term | L3.1 | `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` | Hierarchy-aware culling structure used to bound image synthesis cost. |
| replay-safe envelope | determinism term | L3.0 / engine-wide | `constitutions/STRATUMX_REPLAY_AND_DETERMINISM_CONSTITUTION.md` | Typed output form that is either deterministic itself or explicitly excluded from deterministic world mutation. |

## Operational Terms

| Term | Class | Scope | Canonical Document | Meaning |
|---|---|---|---|---|
| stream activation | resource term | L1.5 | `levels/l1.5-runtime-resource-services/stream-control/40_STREAM_ACTIVATION.md` | Runtime decision to raise or drop a world cell/chunk into the active working set. |
| residency | resource term | L1.5 | `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md` | Canonical CPU/GPU presence state of runtime-ready resources. |
| transfer control | resource term | L1.5 | `levels/l1.5-runtime-resource-services/transfer-control/00_LAYER.md` | Decode, staging, upload, and fence-bound release orchestration. |
| prediction | network term | L2.5 | `levels/l2.5-network-runtime-services/net-latency/41_PREDICTION_AND_RECONCILIATION.md` | Local speculative execution over owned inputs while preserving server authority. |
| reconciliation | network term | L2.5 | `levels/l2.5-network-runtime-services/net-latency/41_PREDICTION_AND_RECONCILIATION.md` | Ordered correction of predicted local state after authoritative confirmation. |
| rewind | network term | L2.5 | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md` | Short-history authoritative reconstruction used for validation without replacing world truth ownership. |

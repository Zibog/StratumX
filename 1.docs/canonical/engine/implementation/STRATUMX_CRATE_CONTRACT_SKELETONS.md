# STRATUMX_CRATE_CONTRACT_SKELETONS

## 1. Purpose

This document defines minimal public boundary skeletons for canonical crates.

## 2. Boundary Form Law

Each crate exposes one primary public boundary form and one primary public boundary type.
Allowed boundary forms and role-to-form legality are defined in:
- `constitutions/STRATUMX_API_CONTRACT_LAW.md`

## 3. Supporting Shape-Class Law

Supporting public types must derive from canonical shape classes defined in:
- `constitutions/STRATUMX_CANONICAL_SHAPES.md`

Canonical supporting shape classes include:
- Config
- Descriptor
- Snapshot
- ReadView
- Context
- Delta
- Result
- Metrics
- ContractSet
- TypeSet
- ErrorSet
- Token
- Query
- Window
- Batch
- Request
- Plan

Domain nouns are legal only when they are explicit instantiations of one canonical shape class.

## 4. Canonical API Contract Law Reference

This document is implementation-facing and derives from:
- `constitutions/STRATUMX_API_CONTRACT_LAW.md`
- `constitutions/STRATUMX_CANONICAL_SHAPES.md`

It does not redefine architectural law or public API authority.

## 5. Legality Rule

Every skeleton row must satisfy all of the following without exception:
- the canonical role class must be one legal role label from `STRATUMX_API_CONTRACT_LAW.md`;
- the primary public boundary form must be legal for that role class;
- the primary public boundary type must instantiate exactly one selected primary boundary form;
- every supporting public type must map to one canonical shape class;
- no row may introduce a competing role class, boundary form, or shape class.

## 6. Canonical Skeletons

| Crate | Canonical Role Class | Primary Public Boundary Form | Primary Public Boundary Type | Minimal Supporting Public Types | Supporting Shape-Class Mapping |
|---|---|---|---|---|---|
| engine_core | foundation substrate | backbone | MathBackbone | BaseTypeSet, ErrorModel, CoreContractSet | BaseTypeSet=TypeSet; ErrorModel=ErrorSet; CoreContractSet=ContractSet |
| engine_identity | identity substrate | registry | IdentityRegistry | EntityId, ComponentId, IdentityDescriptor | EntityId=Token; ComponentId=Token; IdentityDescriptor=Descriptor |
| engine_handle | handle substrate | registry | HandleRegistry | EntityHandle, HandleValidationResult | EntityHandle=Token; HandleValidationResult=Result |
| engine_storage_layout | layout substrate | layout | StorageLayout | LayoutDescriptor, ChunkLayoutDescriptor | LayoutDescriptor=Descriptor; ChunkLayoutDescriptor=Descriptor |
| engine_storage_access | access substrate | window | AccessWindowSet | EcsReadView, EcsWriteWindow | EcsReadView=ReadView; EcsWriteWindow=Window |
| engine_storage_mutation | mutation substrate model | model | MutationStagingModel | MutationDelta, MutationBatch | MutationDelta=Delta; MutationBatch=Batch |
| engine_ecs_registry | physical or metadata registry substrate | registry | EcsRegistry | RegistrationDescriptor, MembershipDescriptor | RegistrationDescriptor=Descriptor; MembershipDescriptor=Descriptor |
| engine_ecs_query | query substrate model | model | EcsQueryModel | EcsQuery, TraversalDescriptor | EcsQuery=Query; TraversalDescriptor=Descriptor |
| engine_ecs | assembled substrate surface | facade | EcsFacade | EcsQuery, EcsReadView, EcsWriteWindow | EcsQuery=Query; EcsReadView=ReadView; EcsWriteWindow=Window |
| engine_world_spatial | spatial substrate model | model | WorldSpatialModel | SpatialDescriptor, SpatialReadView | SpatialDescriptor=Descriptor; SpatialReadView=ReadView |
| engine_world_region | region substrate model | model | WorldRegionModel | RegionDescriptor, RegionReadView | RegionDescriptor=Descriptor; RegionReadView=ReadView |
| engine_world | authoritative owner | owner | World | WorldConfig, WorldSnapshot, WorldApplyBatch, WorldApplyResult | WorldConfig=Config; WorldSnapshot=Snapshot; WorldApplyBatch=Batch; WorldApplyResult=Result |
| engine_material | physical or metadata registry substrate | registry | MaterialRegistry | MaterialConfig, MaterialDescriptor | MaterialConfig=Config; MaterialDescriptor=Descriptor |
| engine_runtime | authoritative owner | owner | RuntimeKernel | RuntimeConfig, ExecutionContext, ExecutionResult, RuntimeDiagnostics | RuntimeConfig=Config; ExecutionContext=Context; ExecutionResult=Result; RuntimeDiagnostics=Metrics |
| engine_runtime_headless | runtime profile | profile | HeadlessRuntimeProfile | HeadlessRuntimeConfig, HeadlessExecutionResult | HeadlessRuntimeConfig=Config; HeadlessExecutionResult=Result |
| engine_runtime_realtime | runtime profile | profile | RealtimeRuntimeProfile | RealtimeRuntimeConfig, RealtimeExecutionResult | RealtimeRuntimeConfig=Config; RealtimeExecutionResult=Result |
| engine_stream_control | runtime service | service | StreamControlService | StreamControlConfig, StreamRequest, StreamResult | StreamControlConfig=Config; StreamRequest=Request; StreamResult=Result |
| engine_residency_control | runtime service | service | ResidencyControlService | ResidencyConfig, ResidencyDescriptor, ResidencyMetrics | ResidencyConfig=Config; ResidencyDescriptor=Descriptor; ResidencyMetrics=Metrics |
| engine_memory_control | runtime service | service | MemoryControlService | MemoryConfig, AllocationDescriptor, MemoryMetrics | MemoryConfig=Config; AllocationDescriptor=Descriptor; MemoryMetrics=Metrics |
| engine_transfer_control | runtime service | service | TransferControlService | TransferConfig, TransferRequest, TransferResult | TransferConfig=Config; TransferRequest=Request; TransferResult=Result |
| engine_kinetics | critical simulation family | family | KineticsFamily | KineticsConfig, KineticsContext, KineticsDelta, KineticsMetrics | KineticsConfig=Config; KineticsContext=Context; KineticsDelta=Delta; KineticsMetrics=Metrics |
| engine_field | critical simulation family | family | FieldFamily | FieldConfig, FieldContext, FieldDelta, FieldMetrics | FieldConfig=Config; FieldContext=Context; FieldDelta=Delta; FieldMetrics=Metrics |
| engine_agents | critical simulation family | family | AgentsFamily | AgentsConfig, AgentsContext, AgentsDelta, AgentsMetrics | AgentsConfig=Config; AgentsContext=Context; AgentsDelta=Delta; AgentsMetrics=Metrics |
| engine_net_transport | runtime service | service | NetTransportService | NetTransportConfig, NetPacketEnvelope, NetTransportMetrics | NetTransportConfig=Config; NetPacketEnvelope=Batch; NetTransportMetrics=Metrics |
| engine_net_sync | runtime service | service | NetSyncService | NetSyncConfig, SyncSnapshot, SyncDelta, NetSyncMetrics | NetSyncConfig=Config; SyncSnapshot=Snapshot; SyncDelta=Delta; NetSyncMetrics=Metrics |
| engine_net_latency | runtime service | service | NetLatencyService | NetLatencyConfig, PredictionContext, ReconcileResult, NetLatencyMetrics | NetLatencyConfig=Config; PredictionContext=Context; ReconcileResult=Result; NetLatencyMetrics=Metrics |
| engine_inference | model or synthesis service | service | InferenceService | InferenceConfig, InferenceRequest, InferenceResult | InferenceConfig=Config; InferenceRequest=Request; InferenceResult=Result |
| engine_generation | model or synthesis service | service | GenerationService | GenerationConfig, GenerationRequest, GenerationResult | GenerationConfig=Config; GenerationRequest=Request; GenerationResult=Result |
| engine_imaging | model or synthesis service | service | ImagingService | ImagingConfig, ImagingRequest, ImagingResult | ImagingConfig=Config; ImagingRequest=Request; ImagingResult=Result |
| engine_acoustics | model or synthesis service | service | AcousticsService | AcousticsConfig, AcousticsRequest, AcousticsResult | AcousticsConfig=Config; AcousticsRequest=Request; AcousticsResult=Result |
| engine_content | resource processing layer | pipeline | ContentPipeline | ContentConfig, ContentRequest, ContentResult | ContentConfig=Config; ContentRequest=Request; ContentResult=Result |
| engine_startup | authoritative owner | owner | StartupAssembly | StartupConfig, StartupReadyAssemblyDecisionSet, RuntimeLaunchPlan | StartupConfig=Config; StartupReadyAssemblyDecisionSet=Result; RuntimeLaunchPlan=Plan |

## 7. Canonical Rule

Implementation may add internal helper types, but may not widen the public boundary beyond what the constitutional API law allows without explicit canon change.

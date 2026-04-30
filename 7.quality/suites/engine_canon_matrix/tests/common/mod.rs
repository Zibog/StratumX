#![allow(unused_imports, dead_code)]
pub use engine_acoustics::{AcousticsConfig, AcousticsRequest, AcousticsService};
pub use engine_agents::{
    AgentId, AgentIntentKind, AgentState, AgentsConfig, AgentsContext, AgentsFamily,
    AgentsSubstrate,
};
pub use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
};
pub use engine_core::contracts::Invariant;
pub use engine_core::{ComponentTypeId, Generation, Tick};
pub use engine_ecs::EcsSubstrate;
pub use engine_ecs_query::{
    AccessDescriptor as QueryAccessDescriptor, Partitionability, QueryAccessMode, QueryDescriptor,
    QueryLocality,
};
pub use engine_ecs_registry::RegistryModel;
pub use engine_field::{
    FieldConfig, FieldContext, FieldFamily, FieldId, FieldSampleRef, FieldUpdateDelta,
    ScalarFieldSubstrate,
};
pub use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService, ModelDescriptor,
    ModelWeights,
};
pub use engine_handle::{
    StableComponentHandle, StableEntityHandle, ValidationContext, ValidationResult,
};
pub use engine_identity::{ComponentId, EntityId, IdentityAllocator, IdentityDomain};
pub use engine_imaging::{ImagingConfig, ImagingRequest, ImagingService};
pub use engine_inference::{InferenceConfig, InferenceModel, InferenceRequest, InferenceService};
pub use engine_kinetics::{KinematicBody, KinematicBodyId, KineticsSubstrate};
pub use engine_kinetics::{KineticsConfig, KineticsContext, KineticsFamily};
pub use engine_material::{
    ceramic_tile, concrete, ConsequenceTier, MaterialArchetypeId, MaterialConfig,
    MaterialDescriptor, MaterialId, MaterialInstanceProfile, MaterialLayer, MaterialRegistry,
    MaterialResponseProfile, MaterialStack, MaterialStackId, MaterialStateModifier,
    MaterialTriggerClass, PropertyDomain, ReactionRow, ResponseFamilyGroup, ResponseFamilyRow,
    ResponseProfileId, SurfaceFamilyProfile, TerritoryFamily,
};
pub use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};
pub use engine_net_latency::{
    LatencyBucket, NetLatencyConfig, NetLatencyService, PredictionContext,
};
pub use engine_net_sync::{NetSyncConfig, NetSyncService};
pub use engine_net_transport::{
    ConnectionHandle, NetPacketEnvelope, NetTransportConfig, NetTransportService, PacketDescriptor,
    PacketLane,
};
pub use engine_residency_control::{
    ResidencyConfig, ResidencyControlService, ResidencyDescriptor, ResidencySet,
};
pub use engine_runtime::{
    BudgetEnvelope, DegradeStep, DomainBudgetUsage, PresentableFrame, PressureAxis, PressureBucket,
    ReplayInputFrame, ReplayOutputDigest, RuntimeConfig, RuntimeKernel, RuntimeMode,
    RuntimeProfile,
};
pub use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
pub use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
pub use engine_startup::{NetworkRole, ServiceWiring, StartupAssembly, StartupConfig};
pub use engine_storage_access::{
    make_read_view, make_write_window, traversal_entry_bind,
    AccessDescriptor as StorageAccessDescriptor, AccessMode, ScratchClass, TraversalPlanId,
};
pub use engine_storage_layout::{
    ChunkAccessMode, ChunkDescriptor, ChunkInvalidationLaw, ColumnDescriptor, LayoutClass,
    LocalityClass, StorageLayoutDescriptor,
};
pub use engine_storage_mutation::{
    make_apply_payload, ChangeSet, DeferredWrite, FamilyTag, IdempotenceClass, MutationBuffer,
    RegionTag,
};
pub use engine_stream_control::{
    StreamControlConfig, StreamControlService, StreamReason, StreamRequest,
};
pub use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};
pub use engine_world::{ApplySegment, WorldState, MAX_FAMILY_FANOUT_PER_SEGMENT};
pub use engine_world_region::{DirtyFlags, RegionSubstrate};
pub use engine_world_spatial::{
    address_for_world_coordinate, classify_relation, compose_transform, ChunkAddress,
    RegionAddress, SpatialRelation, Transform, WorldCoordinate,
};
pub use glam::{Quat, Vec3};
pub use smallvec::smallvec;

pub fn materials() -> MaterialRegistry {
    let mut registry = MaterialRegistry::new(MaterialConfig {
        fallback_descriptor: MaterialDescriptor {
            material_id: MaterialId(0),
            label: "fallback".to_string(),
            property_domains: vec![PropertyDomain::Physical],
            response_profile: ResponseProfileId(0),
        },
        default_reaction: ReactionRow {
            response_profile: ResponseProfileId(0),
            coefficients: [1, 1, 1, 1],
        },
    });
    registry
        .register_descriptor(MaterialDescriptor {
            material_id: MaterialId(1),
            label: "stone".to_string(),
            property_domains: vec![PropertyDomain::Physical],
            response_profile: ResponseProfileId(1),
        })
        .unwrap();
    registry.register_reaction(ReactionRow {
        response_profile: ResponseProfileId(1),
        coefficients: [2, 2, 2, 2],
    });
    registry
}

pub fn startup_wiring() -> ServiceWiring {
    ServiceWiring {
        streaming: true,
        residency: true,
        memory: true,
        transfer: true,
        simulation: true,
        networking: true,
        modeling: true,
        synthesis: true,
    }
}

pub fn floor_chunk(v: f32) -> i32 {
    (v / 32.0_f32).floor() as i32
}

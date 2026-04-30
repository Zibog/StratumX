use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine_acoustics::{AcousticsConfig, AcousticsRequest, AcousticsService};
use engine_agents::{AgentsConfig, AgentsContext, AgentsFamily};
use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
};
use engine_core::{ComponentTypeId, Generation, Tick};
use engine_ecs::EcsSubstrate;
use engine_ecs_query::{
    AccessDescriptor as QueryAccessDescriptor, Partitionability, QueryAccessMode, QueryDescriptor,
    QueryLocality,
};
use engine_ecs_registry::RegistryModel;
use engine_field::{FieldConfig, FieldContext, FieldFamily};
use engine_generation::{
    GenerationConfig, GenerationContext, GenerationRequest, GenerationService, ModelDescriptor,
    ModelWeights,
};
use engine_handle::{StableComponentHandle, StableEntityHandle, ValidationContext};
use engine_identity::{ComponentId, EntityId, IdentityAllocator, IdentityDomain};
use engine_imaging::{ImagingConfig, ImagingRequest, ImagingService};
use engine_inference::{InferenceConfig, InferenceModel, InferenceRequest, InferenceService};
use engine_kinetics::{KineticsConfig, KineticsContext, KineticsFamily};
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_memory_control::{AllocationDescriptor, MemoryConfig, MemoryControlService};
use engine_net_latency::{NetLatencyConfig, NetLatencyService, PredictionContext};
use engine_net_sync::{NetSyncConfig, NetSyncService};
use engine_net_transport::{
    ConnectionHandle, NetPacketEnvelope, NetTransportConfig, NetTransportService, PacketDescriptor,
    PacketLane,
};
use engine_residency_control::{
    ResidencyConfig, ResidencyControlService, ResidencyDescriptor, ResidencySet,
};
use engine_runtime::{PresentableFrame, RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_startup::{NetworkRole, ServiceWiring, StartupAssembly, StartupConfig};
use engine_storage_access::{
    make_read_view, make_write_window, traversal_entry_bind,
    AccessDescriptor as StorageAccessDescriptor, AccessMode, ScratchClass, TraversalPlanId,
};
use engine_storage_layout::{
    ChunkAccessMode, ChunkDescriptor, ChunkInvalidationLaw, LayoutClass, LocalityClass,
    StorageLayoutDescriptor,
};
use engine_storage_mutation::{
    make_apply_payload, DeferredWrite, FamilyTag, IdempotenceClass, MutationBuffer, RegionTag,
};
use engine_stream_control::{
    StreamControlConfig, StreamControlService, StreamReason, StreamRequest,
};
use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};
use engine_world::{ApplySegment, WorldState};
use engine_world_region::{DirtyFlags, RegionSubstrate};
use engine_world_spatial::{
    address_for_world_coordinate, ChunkAddress, RegionAddress, WorldCoordinate,
};
use glam::Vec3;
use smallvec::smallvec;

const FAST_BATCH: usize = 4096;
const STANDARD_BATCH: usize = 1024;
const HEAVY_BATCH: usize = 256;

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(60)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(4))
}

fn materials() -> MaterialRegistry {
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

fn bench_core(c: &mut Criterion) {
    c.bench_function("engine_core_hot_type_ops", |b| {
        b.iter(|| {
            let mut last = (Generation::INITIAL, Tick(0), ComponentTypeId(0));
            for _ in 0..FAST_BATCH {
                last = (
                    black_box(Generation::INITIAL).next(),
                    Tick(black_box(1)),
                    ComponentTypeId(black_box(2)),
                );
            }
            black_box(last)
        })
    });
}

fn bench_identity(c: &mut Criterion) {
    c.bench_function("engine_identity_issue_retire", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for _ in 0..STANDARD_BATCH {
                let mut alloc = IdentityAllocator::new(IdentityDomain::Entity);
                let id = alloc.issue_entity().unwrap();
                black_box(alloc.retire_entity(id));
                count += 1;
            }
            black_box(count)
        })
    });
}

fn bench_handle(c: &mut Criterion) {
    c.bench_function("engine_handle_validate", |b| {
        b.iter(|| {
            let entity = StableEntityHandle::new(EntityId {
                slot: 1,
                generation: Generation::INITIAL,
            });
            let component = StableComponentHandle::new(ComponentId {
                slot: 1,
                generation: Generation::INITIAL,
            });
            for _ in 0..FAST_BATCH {
                black_box(entity.validate(
                    EntityId {
                        slot: 1,
                        generation: Generation::INITIAL,
                    },
                    ValidationContext::BoundaryEntry,
                ));
                black_box(component.validate(
                    ComponentId {
                        slot: 1,
                        generation: Generation::INITIAL,
                    },
                    ValidationContext::Diagnostics,
                ));
            }
        })
    });
}

fn bench_storage_layout(c: &mut Criterion) {
    c.bench_function("engine_storage_layout_validate", |b| {
        b.iter(|| {
            use engine_core::contracts::Invariant;
            let descriptor = StorageLayoutDescriptor {
                layout_class: LayoutClass::ChunkDense,
                chunk: Some(ChunkDescriptor {
                    signature: smallvec![ComponentTypeId(1), ComponentTypeId(2)],
                    access_mode: ChunkAccessMode::READ | ChunkAccessMode::STAGED_WRITE,
                    invalidation_law: ChunkInvalidationLaw::FrozenAtCreation,
                }),
                columns: smallvec![],
                locality_class: LocalityClass::TraversalLane,
            };
            for _ in 0..FAST_BATCH {
                descriptor.check_invariants().unwrap();
                black_box(());
            }
        })
    });
}

fn bench_storage_access(c: &mut Criterion) {
    c.bench_function("engine_storage_access_bind", |b| {
        b.iter(|| {
            let handle = StableEntityHandle::new(EntityId {
                slot: 1,
                generation: Generation::INITIAL,
            });
            let descriptor = StorageAccessDescriptor {
                mode: AccessMode::MIXED,
                plan_id: TraversalPlanId(1),
                locality: LocalityClass::Cache,
                scratch: ScratchClass::Owned,
                staged_mutation_handoff: true,
            };
            for _ in 0..STANDARD_BATCH {
                let _ = make_read_view(descriptor.clone(), handle).unwrap();
                let _ = make_write_window(descriptor.clone(), handle).unwrap();
                traversal_entry_bind(&descriptor, false, false, LocalityClass::Cache).unwrap();
                black_box(());
            }
        })
    });
}

fn bench_storage_mutation(c: &mut Criterion) {
    c.bench_function("engine_storage_mutation_assemble", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut buffer = MutationBuffer::new();
                buffer.stage_write(DeferredWrite {
                    component: ComponentTypeId(1),
                    bytes: smallvec![1, 2, 3],
                    idempotence: IdempotenceClass::Idempotent,
                });
                let set = buffer.into_change_set(smallvec![ComponentTypeId(9)]);
                black_box(make_apply_payload(FamilyTag(1), RegionTag(1), 1, set).unwrap());
            }
        })
    });
}

fn bench_ecs_registry(c: &mut Criterion) {
    c.bench_function("engine_ecs_registry_membership", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut registry = RegistryModel::new();
                let entity = EntityId {
                    slot: 1,
                    generation: Generation::INITIAL,
                };
                let component = ComponentTypeId(1);
                registry.register_entity(entity);
                registry.register_component_class(component);
                registry.attach_component(entity, component).unwrap();
                black_box(registry.membership(entity));
            }
        })
    });
}

fn bench_ecs_query(c: &mut Criterion) {
    c.bench_function("engine_ecs_query_execute", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let mut registry = RegistryModel::new();
                let entity = EntityId {
                    slot: 1,
                    generation: Generation::INITIAL,
                };
                let component = ComponentTypeId(1);
                registry.register_entity(entity);
                registry.register_component_class(component);
                registry.attach_component(entity, component).unwrap();
                let query = QueryDescriptor {
                    component_set: smallvec![component],
                    locality: QueryLocality::Spatial,
                    partitionability: Partitionability::Chunk,
                    cache_key: 1,
                    access: QueryAccessDescriptor {
                        mode: QueryAccessMode::READ,
                        publication_rights: false,
                        scratch: None,
                    },
                    filters: smallvec![],
                    joins: smallvec![],
                };
                black_box(query.execute(&registry).unwrap());
            }
        })
    });
}

fn bench_ecs(c: &mut Criterion) {
    c.bench_function("engine_ecs_facade", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let mut ecs = EcsSubstrate::new();
                let entity = EntityId {
                    slot: 1,
                    generation: Generation::INITIAL,
                };
                let component = ComponentTypeId(1);
                ecs.register_entity(entity);
                ecs.register_component_class(component);
                ecs.attach_component(entity, component).unwrap();
                black_box(ecs.entity_descriptor(entity).unwrap());
            }
        })
    });
}

fn bench_world_spatial(c: &mut Criterion) {
    c.bench_function("engine_world_spatial_address", |b| {
        b.iter(|| {
            for _ in 0..FAST_BATCH {
                black_box(address_for_world_coordinate(WorldCoordinate {
                    meters: Vec3::new(64.0, 64.0, 16.0),
                }));
            }
        })
    });
}

fn bench_world_region(c: &mut Criterion) {
    c.bench_function("engine_world_region_dirty_tracking", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut regions = RegionSubstrate::default();
                let region = RegionAddress {
                    x: 0,
                    y: 0,
                    slab_z: 0,
                };
                let chunk = ChunkAddress {
                    region,
                    chunk_x: 0,
                    chunk_y: 0,
                };
                regions.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
                black_box(regions.region_version(region));
            }
        })
    });
}

fn bench_world(c: &mut Criterion) {
    c.bench_function("engine_world_snapshot_apply", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut world = WorldState::new();
                world
                    .apply(
                        &[ApplySegment {
                            region_key: (0, 0, 0),
                            family_tags: vec![1],
                        }],
                        1,
                    )
                    .unwrap();
                black_box(world.snapshot_bytes(1).unwrap());
            }
        })
    });
}

fn bench_material(c: &mut Criterion) {
    let registry = materials();
    c.bench_function("engine_material_lookup", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                black_box(registry.lookup(MaterialId(1)));
            }
        })
    });
}

fn bench_runtime(c: &mut Criterion) {
    c.bench_function("engine_runtime_tick", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let mut runtime = RuntimeKernel::new(
                    WorldState::new(),
                    RuntimeConfig {
                        profile: RuntimeProfile::Interactive60,
                        max_apply_segments_per_tick: 4,
                        publish_passes: 1,
                    },
                );
                runtime
                    .enqueue_apply_segment(ApplySegment {
                        region_key: (0, 0, 0),
                        family_tags: vec![1],
                    })
                    .unwrap();
                runtime
                    .enqueue_presentable_frame(PresentableFrame {
                        frame_id: 1,
                        visibility_freshness_frames: 1,
                    })
                    .unwrap();
                black_box(runtime.run_tick().unwrap());
            }
        })
    });
}

fn bench_runtime_shells(c: &mut Criterion) {
    c.bench_function("engine_runtime_headless_step", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let mut profile = HeadlessRuntimeProfile::new(
                    WorldState::new(),
                    HeadlessRuntimeConfig {
                        snapshot_segment_count: 1,
                        emit_snapshot_bytes: true,
                    },
                );
                black_box(profile.step().unwrap());
            }
        })
    });
    c.bench_function("engine_runtime_realtime_step", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let mut profile = RealtimeRuntimeProfile::new(
                    WorldState::new(),
                    RealtimeRuntimeConfig {
                        target_fps: 60,
                        visibility_freshness_frames: 1,
                        enqueue_presentable_frames: true,
                    },
                )
                .unwrap();
                black_box(profile.step().unwrap());
            }
        })
    });
}

fn bench_l15(c: &mut Criterion) {
    c.bench_function("engine_stream_control_tick", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut service = StreamControlService::new(StreamControlConfig {
                    max_inflight_requests: 4,
                    prefetch_radius_regions: 1,
                });
                service
                    .queue_request(StreamRequest {
                        region_key: (0, 0, 0),
                        priority: 1,
                        reason: StreamReason::Visibility,
                    })
                    .unwrap();
                black_box(service.tick());
            }
        })
    });
    c.bench_function("engine_residency_control_metrics", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let memory = MemoryControlService::new(MemoryConfig {
                    heap_budget_bytes: 100,
                    staging_budget_bytes: 100,
                });
                let mut service = ResidencyControlService::new(ResidencyConfig {
                    resident_item_budget: 4,
                    streaming_item_budget: 4,
                });
                service.pin(ResidencyDescriptor {
                    asset_key: 1,
                    residency_set: ResidencySet::Hot,
                });
                black_box(service.metrics(&memory));
            }
        })
    });
    c.bench_function("engine_memory_control_cycle", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut service = MemoryControlService::new(MemoryConfig {
                    heap_budget_bytes: 100,
                    staging_budget_bytes: 100,
                });
                service
                    .reserve_heap(&AllocationDescriptor {
                        allocation_id: 1,
                        bytes: 50,
                        layout_class: None,
                    })
                    .unwrap();
                service.release_heap(25);
                black_box(service.metrics());
            }
        })
    });
    c.bench_function("engine_transfer_control_submit", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut service = TransferControlService::new(TransferConfig {
                    max_inflight_decodes: 2,
                    max_inflight_uploads: 2,
                });
                black_box(
                    service
                        .submit(TransferRequest {
                            asset_key: 1,
                            compressed_bytes: 10,
                            decoded_bytes: 20,
                            upload_bytes: 30,
                        })
                        .unwrap(),
                );
            }
        })
    });
}

fn bench_simulation(c: &mut Criterion) {
    let materials = materials();
    c.bench_function("engine_kinetics_simulate", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let family = KineticsFamily::new(KineticsConfig {
                    max_contacts: 8,
                    max_projectiles: 8,
                });
                black_box(
                    family
                        .simulate(
                            &WorldState::new(),
                            &materials,
                            KineticsContext {
                                tick: Tick(0),
                                region_key: (0, 0, 0),
                                contact_count: 1,
                                projectile_count: 1,
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_field_simulate", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let family = FieldFamily::new(FieldConfig {
                    max_region_deltas: 8,
                });
                black_box(
                    family
                        .simulate(
                            &WorldState::new(),
                            &materials,
                            FieldContext {
                                tick: Tick(0),
                                region_key: (0, 0, 0),
                                region_delta_count: 1,
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_agents_simulate", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let family = AgentsFamily::new(AgentsConfig {
                    max_action_intents: 8,
                });
                black_box(
                    family
                        .simulate(
                            &EcsSubstrate::new(),
                            &WorldState::new(),
                            AgentsContext {
                                tick: Tick(0),
                                region_key: (0, 0, 0),
                                action_intent_count: 1,
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
}

fn bench_network(c: &mut Criterion) {
    c.bench_function("engine_net_transport_send", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut runtime = RuntimeKernel::new(
                    WorldState::new(),
                    RuntimeConfig {
                        profile: RuntimeProfile::Headless20,
                        max_apply_segments_per_tick: 1,
                        publish_passes: 1,
                    },
                );
                let mut service = NetTransportService::new(NetTransportConfig {
                    max_packet_size_bytes: 64,
                });
                service.open_session(ConnectionHandle(1));
                black_box(
                    service
                        .send(
                            &mut runtime,
                            NetPacketEnvelope {
                                connection: ConnectionHandle(1),
                                descriptor: PacketDescriptor {
                                    reliable: true,
                                    ordered: true,
                                },
                                lane: PacketLane::Control,
                                payload: vec![1, 2, 3],
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_net_sync_snapshot", |b| {
        b.iter(|| {
            let mut service = NetSyncService::new(NetSyncConfig {
                max_interest_regions: 4,
            });
            for _ in 0..STANDARD_BATCH {
                black_box(
                    service
                        .snapshot(&WorldState::new(), vec![(0, 0, 0)])
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_net_latency_metrics", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let mut service =
                    NetLatencyService::new(NetLatencyConfig { history_size: 4 }).unwrap();
                service.record_sample(30);
                service.record_sample(50);
                black_box((
                    service.metrics(),
                    service.reconcile(PredictionContext {
                        authoritative_tick: Tick(1),
                        predicted_tick: Tick(2),
                    }),
                ));
            }
        })
    });
}

fn bench_models_and_services(c: &mut Criterion) {
    let inference = InferenceService::new(
        InferenceConfig { max_batch_items: 4 },
        InferenceModel {
            model_id: "mock".to_string(),
        },
    );
    c.bench_function("engine_inference_infer", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                black_box(
                    inference
                        .infer_detached(InferenceRequest {
                            prompt: "hello world".to_string(),
                            batch_size: 1,
                        })
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_generation_generate", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let generation = GenerationService::new(
                    GenerationConfig {
                        max_output_chars: 256,
                    },
                    GenerationContext {
                        descriptor: ModelDescriptor {
                            model_family: "gen".to_string(),
                        },
                        weights: ModelWeights {
                            checksum: "abc".to_string(),
                        },
                    },
                );
                black_box(
                    generation
                        .generate(
                            &WorldState::new(),
                            &inference,
                            GenerationRequest {
                                prompt: "hello".to_string(),
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_imaging_render", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let materials = materials();
                let residency = ResidencyControlService::new(ResidencyConfig {
                    resident_item_budget: 4,
                    streaming_item_budget: 4,
                });
                let mut transfer = TransferControlService::new(TransferConfig {
                    max_inflight_decodes: 2,
                    max_inflight_uploads: 2,
                });
                let imaging = ImagingService::new(ImagingConfig {
                    max_render_targets: 4,
                    max_upload_bytes: 64,
                });
                black_box(
                    imaging
                        .render(
                            &WorldState::new(),
                            &EcsSubstrate::new(),
                            &materials,
                            &residency,
                            &mut transfer,
                            ImagingRequest {
                                render_target_id: 1,
                                view_region: (0, 0, 0),
                                upload_bytes: 8,
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_acoustics_synthesize", |b| {
        b.iter(|| {
            for _ in 0..HEAVY_BATCH {
                let materials = materials();
                let residency = ResidencyControlService::new(ResidencyConfig {
                    resident_item_budget: 4,
                    streaming_item_budget: 4,
                });
                let mut transfer = TransferControlService::new(TransferConfig {
                    max_inflight_decodes: 2,
                    max_inflight_uploads: 2,
                });
                let acoustics = AcousticsService::new(AcousticsConfig {
                    max_sources: 4,
                    max_stream_upload_bytes: 64,
                });
                black_box(
                    acoustics
                        .synthesize(
                            &WorldState::new(),
                            &EcsSubstrate::new(),
                            &materials,
                            &residency,
                            &mut transfer,
                            AcousticsRequest {
                                source_count: 2,
                                stream_upload_bytes: 8,
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_content_ingest", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let pipeline = ContentPipeline::new(ContentConfig { max_packs: 4 });
                black_box(
                    pipeline
                        .ingest(ContentRequest {
                            descriptor: ContentDescriptor {
                                content_id: 1,
                                label: "asset".to_string(),
                            },
                            bytes: vec![1, 2, 3],
                            locator: ContentLocator {
                                uri: "mem://asset".to_string(),
                            },
                        })
                        .unwrap(),
                );
            }
        })
    });
    c.bench_function("engine_startup_launch_plan", |b| {
        b.iter(|| {
            for _ in 0..STANDARD_BATCH {
                let startup = StartupAssembly::new(StartupConfig {
                    profile: RuntimeProfile::Headless20,
                    network_role: NetworkRole::HeadlessHost,
                    runtime_manifests: vec![],
                    service_wiring: ServiceWiring {
                        streaming: true,
                        residency: true,
                        memory: true,
                        transfer: true,
                        simulation: true,
                        networking: true,
                        modeling: true,
                        synthesis: true,
                    },
                });
                black_box(startup.runtime_launch_plan().unwrap());
            }
        })
    });
}

criterion_group!(
    name = engine_benches;
    config = criterion_config();
    targets =
        bench_core,
        bench_identity,
        bench_handle,
        bench_storage_layout,
        bench_storage_access,
        bench_storage_mutation,
        bench_ecs_registry,
        bench_ecs_query,
        bench_ecs,
        bench_world_spatial,
        bench_world_region,
        bench_world,
        bench_material,
        bench_runtime,
        bench_runtime_shells,
        bench_l15,
        bench_simulation,
        bench_network,
        bench_models_and_services
);
criterion_main!(engine_benches);

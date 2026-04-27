use criterion::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use engine_agents::{AgentsConfig, AgentsContext, AgentsFamily};
use engine_content::{
    ContentConfig, ContentDescriptor, ContentLocator, ContentPipeline, ContentRequest,
};
use engine_core::Tick;
use engine_ecs::EcsSubstrate;
use engine_field::{FieldConfig, FieldContext, FieldFamily};
use engine_kinetics::{KineticsConfig, KineticsContext, KineticsFamily};
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_memory_control::{AllocationDescriptor, MemoryConfig, MemoryControlService};
use engine_net_sync::{NetSyncConfig, NetSyncService};
use engine_net_transport::{
    ConnectionHandle, NetPacketEnvelope, NetTransportConfig, NetTransportService, PacketDescriptor,
    PacketLane,
};
use engine_runtime::{PresentableFrame, RuntimeConfig, RuntimeKernel, RuntimeProfile};
use engine_stream_control::{
    StreamControlConfig, StreamControlService, StreamReason, StreamRequest,
};
use engine_transfer_control::{TransferConfig, TransferControlService, TransferRequest};
use engine_world::{ApplySegment, WorldState};

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

fn bench_runtime_world_batch(c: &mut Criterion) {
    c.bench_function("scenario_runtime_world_batch", |b| {
        b.iter(|| {
            let mut runtime = RuntimeKernel::new(
                WorldState::new(),
                RuntimeConfig {
                    profile: RuntimeProfile::Interactive60,
                    max_apply_segments_per_tick: 32,
                    publish_passes: 1,
                },
            );
            for idx in 0..32 {
                runtime
                    .enqueue_apply_segment(ApplySegment {
                        region_key: (idx, 0, 0),
                        family_tags: vec![1, 2, 3],
                    })
                    .unwrap();
            }
            runtime
                .enqueue_presentable_frame(PresentableFrame {
                    frame_id: 1,
                    visibility_freshness_frames: 1,
                })
                .unwrap();
            black_box(runtime.run_tick().unwrap());
        })
    });
}

fn bench_resource_pipeline_batch(c: &mut Criterion) {
    c.bench_function("scenario_resource_pipeline_batch", |b| {
        b.iter(|| {
            let mut memory = MemoryControlService::new(MemoryConfig {
                heap_budget_bytes: 8_192,
                staging_budget_bytes: 8_192,
            });
            let mut transfer = TransferControlService::new(TransferConfig {
                max_inflight_decodes: 64,
                max_inflight_uploads: 64,
            });
            let mut stream = StreamControlService::new(StreamControlConfig {
                max_inflight_requests: 64,
                prefetch_radius_regions: 1,
            });
            for idx in 0..32u64 {
                memory
                    .reserve_heap(&AllocationDescriptor {
                        allocation_id: idx,
                        bytes: 64,
                        layout_class: None,
                    })
                    .unwrap();
                transfer
                    .submit(TransferRequest {
                        asset_key: idx,
                        compressed_bytes: 32,
                        decoded_bytes: 64,
                        upload_bytes: 64,
                    })
                    .unwrap();
                stream
                    .queue_request(StreamRequest {
                        region_key: (idx as i32, 0, 0),
                        priority: 1,
                        reason: StreamReason::Visibility,
                    })
                    .unwrap();
            }
            black_box((memory.metrics(), transfer.complete_upload(), stream.tick()));
        })
    });
}

fn bench_simulation_batch(c: &mut Criterion) {
    c.bench_function("scenario_simulation_batch", |b| {
        b.iter(|| {
            let mats = materials();
            let agents = AgentsFamily::new(AgentsConfig {
                max_action_intents: 256,
            });
            let field = FieldFamily::new(FieldConfig {
                max_region_deltas: 256,
            });
            let kinetics = KineticsFamily::new(KineticsConfig {
                max_contacts: 256,
                max_projectiles: 256,
            });
            for idx in 0..32 {
                black_box(
                    agents
                        .simulate(
                            &EcsSubstrate::new(),
                            &WorldState::new(),
                            AgentsContext {
                                tick: Tick(0),
                                region_key: (idx, 0, 0),
                                action_intent_count: 4,
                            },
                        )
                        .unwrap(),
                );
                black_box(
                    field
                        .simulate(
                            &WorldState::new(),
                            &mats,
                            FieldContext {
                                tick: Tick(0),
                                region_key: (idx, 0, 0),
                                region_delta_count: 4,
                            },
                        )
                        .unwrap(),
                );
                black_box(
                    kinetics
                        .simulate(
                            &WorldState::new(),
                            &mats,
                            KineticsContext {
                                tick: Tick(0),
                                region_key: (idx, 0, 0),
                                contact_count: 4,
                                projectile_count: 4,
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
}

fn bench_network_batch(c: &mut Criterion) {
    c.bench_function("scenario_network_batch", |b| {
        b.iter(|| {
            let sync = NetSyncService::new(NetSyncConfig {
                max_interest_regions: 64,
            });
            let mut transport = NetTransportService::new(NetTransportConfig {
                max_packet_size_bytes: 16_384,
            });
            let mut runtime = RuntimeKernel::new(
                WorldState::new(),
                RuntimeConfig {
                    profile: RuntimeProfile::Interactive60,
                    max_apply_segments_per_tick: 1,
                    publish_passes: 1,
                },
            );
            transport.open_session(ConnectionHandle(1));
            let (snapshot, _) = sync
                .snapshot(&WorldState::new(), vec![(0, 0, 0); 16])
                .unwrap();
            for _ in 0..16 {
                let payload = bincode::serialize(&snapshot).unwrap();
                black_box(
                    transport
                        .send(
                            &mut runtime,
                            NetPacketEnvelope {
                                connection: ConnectionHandle(1),
                                descriptor: PacketDescriptor {
                                    reliable: true,
                                    ordered: true,
                                },
                                lane: PacketLane::State,
                                payload,
                            },
                        )
                        .unwrap(),
                );
            }
        })
    });
}

fn bench_content_batch(c: &mut Criterion) {
    c.bench_function("scenario_content_batch", |b| {
        b.iter(|| {
            let pipeline = ContentPipeline::new(ContentConfig { max_packs: 64 });
            for idx in 0..32u64 {
                let bytes = vec![1u8; 128];
                let result = pipeline
                    .ingest(ContentRequest {
                        descriptor: ContentDescriptor {
                            content_id: idx + 1,
                            label: "asset".to_string(),
                        },
                        bytes,
                        locator: ContentLocator {
                            uri: format!("mem://asset/{idx}"),
                        },
                    })
                    .unwrap();
                black_box(pipeline.build_runtime_pack_product(&result.manifest));
            }
        })
    });
}

criterion_group!(
    engine_scenarios,
    bench_runtime_world_batch,
    bench_resource_pipeline_batch,
    bench_simulation_batch,
    bench_network_batch,
    bench_content_batch
);
criterion_main!(engine_scenarios);

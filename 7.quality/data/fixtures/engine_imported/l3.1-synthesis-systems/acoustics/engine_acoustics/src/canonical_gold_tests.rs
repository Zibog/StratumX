#![allow(unused_imports)]
use super::*;
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};

fn materials() -> MaterialRegistry {
    MaterialRegistry::new(MaterialConfig {
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
    })
}
#[test]
fn synthesize_rejects_too_many_sources() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 1,
        max_stream_upload_bytes: 8,
    });
    assert!(s
        .synthesize(
            &engine_world::WorldState::new(),
            &engine_ecs::EcsSubstrate::new(),
            &materials(),
            &engine_residency_control::ResidencyControlService::new(
                engine_residency_control::ResidencyConfig {
                    resident_item_budget: 8,
                    streaming_item_budget: 8
                }
            ),
            &mut transfer,
            AcousticsRequest {
                source_count: 2,
                stream_upload_bytes: 0
            }
        )
        .is_err());
}
#[test]
fn synthesize_rejects_upload_overflow() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 1,
    });
    assert!(s
        .synthesize(
            &engine_world::WorldState::new(),
            &engine_ecs::EcsSubstrate::new(),
            &materials(),
            &engine_residency_control::ResidencyControlService::new(
                engine_residency_control::ResidencyConfig {
                    resident_item_budget: 8,
                    streaming_item_budget: 8
                }
            ),
            &mut transfer,
            AcousticsRequest {
                source_count: 1,
                stream_upload_bytes: 2
            }
        )
        .is_err());
}
#[test]
fn synthesize_returns_source_count() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 8,
    });
    assert_eq!(
        s.synthesize(
            &engine_world::WorldState::new(),
            &engine_ecs::EcsSubstrate::new(),
            &materials(),
            &engine_residency_control::ResidencyControlService::new(
                engine_residency_control::ResidencyConfig {
                    resident_item_budget: 8,
                    streaming_item_budget: 8
                }
            ),
            &mut transfer,
            AcousticsRequest {
                source_count: 3,
                stream_upload_bytes: 0
            }
        )
        .unwrap()
        .propagated_sources,
        3
    );
}
#[test]
fn synthesize_accepts_zero_upload() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 8,
    });
    assert!(s
        .synthesize(
            &engine_world::WorldState::new(),
            &engine_ecs::EcsSubstrate::new(),
            &materials(),
            &engine_residency_control::ResidencyControlService::new(
                engine_residency_control::ResidencyConfig {
                    resident_item_budget: 8,
                    streaming_item_budget: 8
                }
            ),
            &mut transfer,
            AcousticsRequest {
                source_count: 1,
                stream_upload_bytes: 0
            }
        )
        .is_ok());
}
#[test]
fn synthesize_emits_one_frame() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 8,
    });
    assert_eq!(
        s.synthesize(
            &engine_world::WorldState::new(),
            &engine_ecs::EcsSubstrate::new(),
            &materials(),
            &engine_residency_control::ResidencyControlService::new(
                engine_residency_control::ResidencyConfig {
                    resident_item_budget: 8,
                    streaming_item_budget: 8
                }
            ),
            &mut transfer,
            AcousticsRequest {
                source_count: 1,
                stream_upload_bytes: 0
            }
        )
        .unwrap()
        .synthesized_frames,
        1
    );
}

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
fn render_rejects_target_overflow() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 1,
        max_upload_bytes: 8,
    });
    assert!(s
        .render(
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
            ImagingRequest {
                render_target_id: 2,
                view_region: (0, 0, 0),
                upload_bytes: 0
            }
        )
        .is_err());
}
#[test]
fn render_rejects_upload_overflow() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 1,
    });
    assert!(s
        .render(
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
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 2
            }
        )
        .is_err());
}
#[test]
fn render_reports_uploaded_bytes() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 8,
    });
    assert_eq!(
        s.render(
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
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 3
            }
        )
        .unwrap()
        .upload_bytes,
        3
    );
}
#[test]
fn render_accepts_zero_upload() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 8,
    });
    assert!(s
        .render(
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
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 0
            }
        )
        .is_ok());
}
#[test]
fn render_emits_one_frame() {
    let mut transfer = engine_transfer_control::TransferControlService::new(
        engine_transfer_control::TransferConfig {
            max_inflight_decodes: 8,
            max_inflight_uploads: 8,
        },
    );
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 8,
    });
    assert_eq!(
        s.render(
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
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 0
            }
        )
        .unwrap()
        .rendered_frames,
        1
    );
}

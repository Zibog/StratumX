use engine_ecs::EcsSubstrate;
use engine_imaging::{ImagingConfig, ImagingRequest, ImagingService};
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_residency_control::{ResidencyConfig, ResidencyControlService};
use engine_transfer_control::{TransferConfig, TransferControlService};
use engine_world::WorldState;

#[test]
fn imaging_service_emits_frame_result() {
    let imaging = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 256,
    });
    let materials = MaterialRegistry::new(MaterialConfig {
        fallback_descriptor: MaterialDescriptor {
            material_id: MaterialId(0),
            label: "fallback".to_string(),
            property_domains: vec![PropertyDomain::Appearance],
            response_profile: ResponseProfileId(0),
        },
        default_reaction: ReactionRow {
            response_profile: ResponseProfileId(0),
            coefficients: [1, 1, 1, 1],
        },
    });
    let residency = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 4,
        streaming_item_budget: 4,
    });
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 2,
        max_inflight_uploads: 2,
    });
    let result = imaging
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials,
            &residency,
            &mut transfer,
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 64,
            },
        )
        .unwrap();
    assert_eq!(result.rendered_frames, 1);
}

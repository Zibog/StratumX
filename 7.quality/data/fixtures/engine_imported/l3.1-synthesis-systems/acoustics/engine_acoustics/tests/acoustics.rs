use engine_acoustics::{AcousticsConfig, AcousticsRequest, AcousticsService};
use engine_ecs::EcsSubstrate;
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_residency_control::{ResidencyConfig, ResidencyControlService};
use engine_transfer_control::{TransferConfig, TransferControlService};
use engine_world::WorldState;

#[test]
fn acoustics_service_emits_audio_result() {
    let acoustics = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 128,
    });
    let materials = MaterialRegistry::new(MaterialConfig {
        fallback_descriptor: MaterialDescriptor {
            material_id: MaterialId(0),
            label: "fallback".to_string(),
            property_domains: vec![PropertyDomain::Acoustic],
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
    let result = acoustics
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials,
            &residency,
            &mut transfer,
            AcousticsRequest {
                source_count: 2,
                stream_upload_bytes: 64,
            },
        )
        .unwrap();
    assert_eq!(result.propagated_sources, 2);
}

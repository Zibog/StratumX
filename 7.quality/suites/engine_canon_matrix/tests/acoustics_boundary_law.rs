use engine_acoustics::{
    AcousticFailure, AcousticFailureReason, AcousticInputs, AcousticReceipt, AcousticTier,
    AcousticsConfig, AcousticsRequest, AcousticsService, AcousticsTransportOutcome,
};
use engine_core::EngineCoreError;
use engine_ecs::EcsSubstrate;
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_residency_control::{ResidencyConfig, ResidencyControlService};
use engine_transfer_control::{TransferConfig, TransferControlService};
use engine_world::WorldState;

fn test_materials() -> MaterialRegistry {
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
        coefficients: [4, 4, 4, 4],
    });
    registry
}

fn synthesize(
    service: &AcousticsService,
    materials: &MaterialRegistry,
    material_id: MaterialId,
    source_count: usize,
) -> Result<AcousticReceipt, AcousticFailure> {
    let world = WorldState::new();
    let ecs = EcsSubstrate::new();
    let residency = ResidencyControlService::new(ResidencyConfig {
        resident_item_budget: 8,
        streaming_item_budget: 8,
    });
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    service.synthesize(
        AcousticsRequest {
            source_count,
            stream_upload_bytes: source_count.saturating_mul(64),
        },
        AcousticInputs {
            world: &world,
            ecs: &ecs,
            materials,
            residency: &residency,
            transfer: &mut transfer,
            material_id,
        },
    )
}

#[test]
fn acoustics_primary_synthesize_is_material_aware() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();

    let fallback = synthesize(&service, &materials, MaterialId(0), 3).unwrap();
    let stone = synthesize(&service, &materials, MaterialId(1), 3).unwrap();

    assert_eq!(fallback.selected_tier, AcousticTier::EventOnly);
    assert_eq!(stone.selected_tier, AcousticTier::ReducedPropagation);
    assert_ne!(fallback.material_policy_id, stone.material_policy_id);
    assert_ne!(fallback.deterministic_digest, stone.deterministic_digest);
}

#[test]
fn acoustics_primary_synthesize_returns_receipt() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();
    let receipt = synthesize(&service, &materials, MaterialId(1), 3).unwrap();

    assert_eq!(receipt.synthesized_frames, 1);
    assert_eq!(receipt.source_count, 3);
    assert_eq!(receipt.propagated_sources, 3);
    assert!(receipt.stream_upload_accepted);
}

#[test]
fn acoustics_transport_only_does_not_accept_material_registry() {
    let _: fn(
        &AcousticsService,
        &mut TransferControlService,
        AcousticsRequest,
    ) -> engine_acoustics::AcousticResult<AcousticsTransportOutcome> =
        AcousticsService::synthesize_transport_only;
}

#[test]
fn acoustics_missing_material_returns_typed_failure_or_explicit_fallback() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();
    let receipt = synthesize(&service, &materials, MaterialId(99), 3).unwrap();

    assert!(receipt.used_material_fallback);
    assert_eq!(receipt.selected_tier, AcousticTier::EventOnly);
    assert_eq!(receipt.emitter_count, 1);
}

#[test]
fn acoustics_missing_material_uses_explicit_fallback() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();
    let receipt = synthesize(&service, &materials, MaterialId(99), 3).unwrap();

    assert!(receipt.used_material_fallback);
    assert_eq!(receipt.selected_tier, AcousticTier::EventOnly);
}

#[test]
fn acoustics_failure_reason_is_typed_primary_api() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();
    let result = synthesize(&service, &materials, MaterialId(1), 0);

    assert_eq!(
        result.unwrap_err().reason,
        AcousticFailureReason::InvalidSourceCount
    );
}

#[test]
fn same_acoustic_input_produces_same_digest() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();
    let receipt1 = synthesize(&service, &materials, MaterialId(1), 3).unwrap();
    let receipt2 = synthesize(&service, &materials, MaterialId(1), 3).unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn legacy_engine_core_error_bridge_preserves_reason_text() {
    let bridge: EngineCoreError =
        AcousticFailure::for_reason(AcousticFailureReason::InvalidSourceCount).into();
    assert_eq!(
        bridge,
        EngineCoreError::InvalidDescriptor("source count must be non-zero")
    );
}

use engine_acoustics::{AcousticTier, AcousticsConfig, AcousticsService};
use engine_core::EngineCoreError;
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};

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

#[test]
fn acoustics_material_profile_changes_output_and_digest() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();

    let fallback = service
        .synthesize_with_receipt(&materials, MaterialId(0), 3)
        .unwrap();
    let stone = service
        .synthesize_with_receipt(&materials, MaterialId(1), 3)
        .unwrap();

    assert_eq!(fallback.selected_tier, AcousticTier::EventOnly);
    assert_eq!(stone.selected_tier, AcousticTier::ReducedPropagation);
    assert_eq!(fallback.emitter_count, 1);
    assert_eq!(stone.emitter_count, 2);
    assert_ne!(fallback.deterministic_digest, stone.deterministic_digest);
}

#[test]
fn acoustics_missing_material_uses_explicit_fallback() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();

    let receipt = service
        .synthesize_with_receipt(&materials, MaterialId(99), 3)
        .unwrap();

    assert!(receipt.used_material_fallback);
    assert_eq!(receipt.selected_tier, AcousticTier::EventOnly);
    assert_eq!(receipt.emitter_count, 1);
    assert!(receipt.stream_upload_accepted);
}

#[test]
fn acoustics_budget_pressure_selects_reduced_tier() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();

    let low_source = service
        .synthesize_with_receipt(&materials, MaterialId(0), 2)
        .unwrap();
    let high_source = service
        .synthesize_with_receipt(&materials, MaterialId(0), 8)
        .unwrap();

    assert_eq!(low_source.selected_tier, AcousticTier::EventOnly);
    assert_eq!(high_source.selected_tier, AcousticTier::EventOnly);
    assert_eq!(low_source.emitter_count, 1);
    assert_eq!(high_source.emitter_count, 1);

    let stone_low = service
        .synthesize_with_receipt(&materials, MaterialId(1), 2)
        .unwrap();
    let stone_high = service
        .synthesize_with_receipt(&materials, MaterialId(1), 8)
        .unwrap();

    assert_eq!(stone_low.selected_tier, AcousticTier::ReducedPropagation);
    assert_eq!(stone_high.selected_tier, AcousticTier::ReducedPropagation);
    assert!(stone_high.deterministic_digest != stone_low.deterministic_digest);
}

#[test]
fn acoustics_zero_source_is_rejected_with_exact_reason() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();

    let result = service.synthesize_with_receipt(&materials, MaterialId(1), 0);

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "source count must be non-zero",
        ))
    );
}

#[test]
fn same_acoustic_input_produces_same_digest() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();

    let receipt1 = service
        .synthesize_with_receipt(&materials, MaterialId(1), 3)
        .unwrap();
    let receipt2 = service
        .synthesize_with_receipt(&materials, MaterialId(1), 3)
        .unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn different_source_count_changes_digest() {
    let service = AcousticsService::new(AcousticsConfig {
        max_sources: 10,
        max_stream_upload_bytes: 1024,
    });
    let materials = test_materials();

    let receipt1 = service
        .synthesize_with_receipt(&materials, MaterialId(1), 2)
        .unwrap();
    let receipt2 = service
        .synthesize_with_receipt(&materials, MaterialId(1), 4)
        .unwrap();

    assert_ne!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

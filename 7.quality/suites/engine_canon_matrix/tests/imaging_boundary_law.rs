use engine_core::EngineCoreError;
use engine_imaging::{ImagingConfig, ImagingService, ImagingTier};
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
            property_domains: vec![PropertyDomain::Physical, PropertyDomain::Thermal],
            response_profile: ResponseProfileId(1),
        })
        .unwrap();
    registry
}

#[test]
fn imaging_material_policy_changes_primary_output_and_digest() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();

    let fallback = service
        .render_with_receipt(&materials, MaterialId(0), 128)
        .unwrap();
    let stone = service
        .render_with_receipt(&materials, MaterialId(1), 128)
        .unwrap();

    assert_eq!(fallback.selected_tier, ImagingTier::Impostor);
    assert_eq!(stone.selected_tier, ImagingTier::Full);
    assert_ne!(fallback.material_policy_id, stone.material_policy_id);
    assert_ne!(fallback.deterministic_digest, stone.deterministic_digest);
}

#[test]
fn imaging_missing_material_uses_explicit_fallback_policy() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();

    let receipt = service
        .render_with_receipt(&materials, MaterialId(99), 64)
        .unwrap();

    assert!(receipt.used_material_fallback);
    assert_eq!(receipt.selected_tier, ImagingTier::Impostor);
    assert!(receipt.upload_accepted);
}

#[test]
fn imaging_budget_pressure_changes_tier_and_resource_count() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1000,
    });
    let materials = test_materials();

    let low_budget = service
        .render_with_receipt(&materials, MaterialId(1), 40)
        .unwrap();
    let high_budget = service
        .render_with_receipt(&materials, MaterialId(1), 200)
        .unwrap();

    assert_eq!(low_budget.selected_tier, ImagingTier::Impostor);
    assert_eq!(high_budget.selected_tier, ImagingTier::Full);
    assert!(high_budget.resource_count > low_budget.resource_count);
}

#[test]
fn imaging_upload_over_budget_is_rejected_with_exact_reason() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 128,
    });
    let materials = test_materials();

    let result = service.render_with_receipt(&materials, MaterialId(1), 256);

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "upload bytes exceed configured imaging ceiling",
        ))
    );
}

#[test]
fn same_imaging_input_produces_same_digest() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();

    let receipt1 = service
        .render_with_receipt(&materials, MaterialId(1), 256)
        .unwrap();
    let receipt2 = service
        .render_with_receipt(&materials, MaterialId(1), 256)
        .unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn different_upload_bytes_change_digest() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();

    let receipt1 = service
        .render_with_receipt(&materials, MaterialId(1), 128)
        .unwrap();
    let receipt2 = service
        .render_with_receipt(&materials, MaterialId(1), 256)
        .unwrap();

    assert_ne!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

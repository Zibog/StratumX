use engine_core::EngineCoreError;
use engine_ecs::EcsSubstrate;
use engine_imaging::{
    ImagingConfig, ImagingFailure, ImagingFailureReason, ImagingInputs, ImagingRequest,
    ImagingService, ImagingTier, ImagingTransportOutcome,
};
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
            property_domains: vec![PropertyDomain::Physical, PropertyDomain::Thermal],
            response_profile: ResponseProfileId(1),
        })
        .unwrap();
    registry
}

fn render(
    service: &ImagingService,
    materials: &MaterialRegistry,
    material_id: MaterialId,
    upload_bytes: usize,
) -> Result<engine_imaging::ImagingReceipt, ImagingFailure> {
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
    service.render(
        ImagingRequest {
            render_target_id: 1,
            view_region: (0, 0, 0),
            upload_bytes,
        },
        ImagingInputs {
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
fn imaging_primary_render_is_material_aware() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();

    let fallback = render(&service, &materials, MaterialId(0), 128).unwrap();
    let stone = render(&service, &materials, MaterialId(1), 128).unwrap();

    assert_eq!(fallback.selected_tier, ImagingTier::Impostor);
    assert_eq!(stone.selected_tier, ImagingTier::Full);
    assert_ne!(fallback.material_policy_id, stone.material_policy_id);
    assert_ne!(fallback.deterministic_digest, stone.deterministic_digest);
}

#[test]
fn imaging_primary_render_returns_receipt() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();
    let receipt = render(&service, &materials, MaterialId(1), 256).unwrap();

    assert_eq!(receipt.rendered_frames, 1);
    assert_eq!(receipt.upload_bytes, 256);
    assert!(!receipt.used_material_fallback);
    assert!(receipt.upload_accepted);
}

#[test]
fn imaging_transport_only_does_not_accept_material_registry() {
    let _: fn(
        &ImagingService,
        &mut TransferControlService,
        ImagingRequest,
    ) -> engine_imaging::ImagingResult<ImagingTransportOutcome> =
        ImagingService::render_transport_only;
}

#[test]
fn imaging_missing_material_returns_typed_failure_or_explicit_fallback() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();
    let receipt = render(&service, &materials, MaterialId(99), 64).unwrap();

    assert!(receipt.used_material_fallback);
    assert_eq!(receipt.selected_tier, ImagingTier::Impostor);
    assert!(receipt.upload_accepted);
}

#[test]
fn imaging_missing_material_uses_explicit_fallback_policy() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();
    let receipt = render(&service, &materials, MaterialId(99), 64).unwrap();

    assert!(receipt.used_material_fallback);
    assert_eq!(receipt.selected_tier, ImagingTier::Impostor);
}

#[test]
fn imaging_failure_reason_is_typed_primary_api() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 128,
    });
    let materials = test_materials();
    let result = render(&service, &materials, MaterialId(1), 256);

    assert_eq!(
        result.unwrap_err().reason,
        ImagingFailureReason::UploadBytesExceeded
    );
}

#[test]
fn same_imaging_input_produces_same_digest() {
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 10,
        max_upload_bytes: 1024,
    });
    let materials = test_materials();

    let receipt1 = render(&service, &materials, MaterialId(1), 256).unwrap();
    let receipt2 = render(&service, &materials, MaterialId(1), 256).unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn legacy_engine_core_error_bridge_preserves_reason_text() {
    let bridge: EngineCoreError =
        ImagingFailure::for_reason(ImagingFailureReason::UploadBytesExceeded).into();
    assert_eq!(
        bridge,
        EngineCoreError::InvalidDescriptor("upload bytes exceed configured imaging ceiling")
    );
}

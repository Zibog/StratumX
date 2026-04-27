//! Imaging: проверка системы рендеринга

mod common;
use common::*;
use engine_imaging::{LightSource, ShadowCaster};

#[test]
fn render_pipeline() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 4,
        max_inflight_uploads: 4,
    });
    let service = ImagingService::new(ImagingConfig {
        max_render_targets: 4,
        max_upload_bytes: 64,
    });

    let result = service
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 4,
                streaming_item_budget: 4,
            }),
            &mut transfer,
            ImagingRequest {
                render_target_id: 2,
                view_region: (0, 0, 0),
                upload_bytes: 16,
            },
        )
        .unwrap();

    let upload = transfer.complete_upload().unwrap();
    assert_eq!(result.rendered_frames, 1);
    assert_eq!(upload.asset_key, 2);
    assert_eq!(upload.upload_bytes, 16);
}

#[test]
fn material_rendering() {
    let lookup = materials().lookup(MaterialId(999));

    assert!(lookup.used_fallback);
    assert_eq!(lookup.descriptor.label, "fallback");
    assert_eq!(lookup.reaction.response_profile, ResponseProfileId(0));
}

#[test]
fn lighting_system() {
    let light = LightSource::explosion([0.0, 0.0, 0.0], 50_000.0);
    let caster = ShadowCaster {
        position: [0.0, 0.0, 2.0],
        bounds: [0.5, 0.5, 0.5],
        casts_shadow: true,
    };

    assert!(
        light.intensity_at_position([0.0, 0.0, 1.0])
            > light.intensity_at_position([0.0, 0.0, 10.0])
    );
    assert!(caster.blocks_light(light.position, [0.0, 0.0, 2.1]));
}

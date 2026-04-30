#[test]
fn imaging_render_case_18() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 64,
    });
    let r = s
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 2,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 2);
}
#[test]
fn imaging_render_case_19() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 64,
    });
    let r = s
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 3,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 3);
}
#[test]
fn imaging_render_case_20() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 64,
    });
    let r = s
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 4,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 4);
}
#[test]
fn imaging_render_case_21() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 64,
    });
    let r = s
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 5,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 5);
}
#[test]
fn imaging_render_case_22() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 64,
    });
    let r = s
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 6,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 6);
}
#[test]
fn imaging_render_case_23() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = ImagingService::new(ImagingConfig {
        max_render_targets: 8,
        max_upload_bytes: 64,
    });
    let r = s
        .render(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            ImagingRequest {
                render_target_id: 1,
                view_region: (0, 0, 0),
                upload_bytes: 7,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 7);
}

#[test]
fn imaging_render_case_0() {
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
                upload_bytes: 0,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 0);
}
#[test]
fn imaging_render_case_1() {
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
                upload_bytes: 1,
            },
        )
        .unwrap();
    assert_eq!(r.upload_bytes, 1);
}
#[test]
fn imaging_render_case_2() {
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
fn imaging_render_case_3() {
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
fn imaging_render_case_4() {
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
fn imaging_render_case_5() {
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

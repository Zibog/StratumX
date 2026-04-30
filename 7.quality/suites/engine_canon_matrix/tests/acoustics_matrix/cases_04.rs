#[test]
fn acoustics_synthesize_case_18() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 64,
    });
    let r = s
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            AcousticsRequest {
                source_count: 3,
                stream_upload_bytes: 2,
            },
        )
        .unwrap();
    assert_eq!(r.propagated_sources, 3);
}
#[test]
fn acoustics_synthesize_case_19() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 64,
    });
    let r = s
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            AcousticsRequest {
                source_count: 4,
                stream_upload_bytes: 3,
            },
        )
        .unwrap();
    assert_eq!(r.propagated_sources, 4);
}
#[test]
fn acoustics_synthesize_case_20() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 64,
    });
    let r = s
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            AcousticsRequest {
                source_count: 1,
                stream_upload_bytes: 4,
            },
        )
        .unwrap();
    assert_eq!(r.propagated_sources, 1);
}
#[test]
fn acoustics_synthesize_case_21() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 64,
    });
    let r = s
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            AcousticsRequest {
                source_count: 2,
                stream_upload_bytes: 5,
            },
        )
        .unwrap();
    assert_eq!(r.propagated_sources, 2);
}
#[test]
fn acoustics_synthesize_case_22() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 64,
    });
    let r = s
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            AcousticsRequest {
                source_count: 3,
                stream_upload_bytes: 6,
            },
        )
        .unwrap();
    assert_eq!(r.propagated_sources, 3);
}
#[test]
fn acoustics_synthesize_case_23() {
    let mut transfer = TransferControlService::new(TransferConfig {
        max_inflight_decodes: 8,
        max_inflight_uploads: 8,
    });
    let s = AcousticsService::new(AcousticsConfig {
        max_sources: 8,
        max_stream_upload_bytes: 64,
    });
    let r = s
        .synthesize(
            &WorldState::new(),
            &EcsSubstrate::new(),
            &materials(),
            &ResidencyControlService::new(ResidencyConfig {
                resident_item_budget: 8,
                streaming_item_budget: 8,
            }),
            &mut transfer,
            AcousticsRequest {
                source_count: 4,
                stream_upload_bytes: 7,
            },
        )
        .unwrap();
    assert_eq!(r.propagated_sources, 4);
}

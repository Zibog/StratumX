#[test]
fn acoustics_synthesize_case_24() {
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
                stream_upload_bytes: 0,
            },
        )
        .unwrap();
    assert_eq!(r.propagated_sources, 1);
}

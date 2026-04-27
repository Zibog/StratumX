use engine_runtime::{
    ConnectionKey, PresentableFrame, RuntimeConfig, RuntimeKernel, RuntimePhase, RuntimeProfile,
    CONNECTION_PUBLICATION_BYTES_CEILING,
};
use engine_world::{ApplySegment, WorldState};

#[test]
fn headless_runtime_rejects_presentation_queue() {
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(runtime
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}

#[test]
fn run_tick_clears_transient_queues_and_reaches_diagnostics_phase() {
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 4,
            publish_passes: 1,
        },
    );
    runtime
        .enqueue_apply_segment(ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        })
        .unwrap();
    runtime
        .enqueue_transfer_completion(engine_runtime::TransferCompletion {
            transfer_id: 1,
            bytes: 4,
        })
        .unwrap();
    runtime
        .enqueue_connection_publication(ConnectionKey(1), vec![1, 2, 3])
        .unwrap();
    runtime
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1,
        })
        .unwrap();
    let result = runtime.run_tick().unwrap();
    let diagnostics = runtime.diagnostics();
    assert_eq!(result.applied_segments, 1);
    assert_eq!(diagnostics.last_phase, RuntimePhase::Diagnostics);
    assert_eq!(diagnostics.transfer_completion_queue_depth, 0);
    assert_eq!(diagnostics.presentable_frame_depth, 0);
}

#[test]
fn connection_publication_enforces_byte_ceiling() {
    let mut runtime = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    let payload = vec![7u8; CONNECTION_PUBLICATION_BYTES_CEILING];
    runtime
        .enqueue_connection_publication(ConnectionKey(7), payload)
        .unwrap();
    assert!(runtime
        .enqueue_connection_publication(ConnectionKey(7), vec![1])
        .is_err());
}

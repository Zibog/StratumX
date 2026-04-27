#![allow(unused_imports)]
use super::*;
use engine_world::ApplySegment;

#[test]
fn apply_queue_accepts_valid_segment() {
    let mut r = RuntimeKernel::new(
        engine_world::WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(r
        .enqueue_apply_segment(ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1]
        })
        .is_ok());
}
#[test]
fn headless_profile_rejects_presentable_frame() {
    let mut r = RuntimeKernel::new(
        engine_world::WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(r
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn connection_publication_enforces_byte_limit() {
    let mut r = RuntimeKernel::new(
        engine_world::WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(r
        .enqueue_connection_publication(
            ConnectionKey(1),
            vec![0u8; CONNECTION_PUBLICATION_BYTES_CEILING + 1]
        )
        .is_err());
}
#[test]
fn run_tick_clears_queues() {
    let mut r = RuntimeKernel::new(
        engine_world::WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    r.enqueue_transfer_completion(TransferCompletion {
        transfer_id: 1,
        bytes: 4,
    })
    .unwrap();
    r.enqueue_presentable_frame(PresentableFrame {
        frame_id: 1,
        visibility_freshness_frames: 1,
    })
    .unwrap();
    let _ = r.run_tick().unwrap();
    assert_eq!(r.diagnostics().transfer_completion_queue_depth, 0);
}
#[test]
fn run_tick_applies_segment_budget() {
    let mut r = RuntimeKernel::new(
        engine_world::WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Interactive60,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    r.enqueue_apply_segment(ApplySegment {
        region_key: (0, 0, 0),
        family_tags: vec![1],
    })
    .unwrap();
    r.enqueue_apply_segment(ApplySegment {
        region_key: (1, 0, 0),
        family_tags: vec![1],
    })
    .unwrap();
    r.enqueue_presentable_frame(PresentableFrame {
        frame_id: 1,
        visibility_freshness_frames: 1,
    })
    .unwrap();
    let out = r.run_tick().unwrap();
    assert_eq!(out.applied_segments, 1);
}

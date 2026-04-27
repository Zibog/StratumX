mod common;
use common::*;

#[test]
fn runtime_headless_rejects_present_0() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_1() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_2() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_3() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_4() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_5() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_6() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_7() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_8() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_9() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_10() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_11() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_12() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_13() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_14() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_15() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_16() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_17() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_18() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_19() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_20() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_21() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_22() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_23() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_rejects_present_24() {
    let mut rt = RuntimeKernel::new(
        WorldState::new(),
        RuntimeConfig {
            profile: RuntimeProfile::Headless20,
            max_apply_segments_per_tick: 1,
            publish_passes: 1,
        },
    );
    assert!(rt
        .enqueue_presentable_frame(PresentableFrame {
            frame_id: 1,
            visibility_freshness_frames: 1
        })
        .is_err());
}
#[test]
fn runtime_headless_step_case_0() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_1() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_2() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_3() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_4() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_5() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_6() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_7() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_8() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_9() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_10() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_11() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_12() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_13() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_14() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_15() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_16() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_17() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_18() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_19() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_20() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_21() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_22() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 1,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_23() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 2,
            emit_snapshot_bytes: false,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}
#[test]
fn runtime_headless_step_case_24() {
    let mut p = HeadlessRuntimeProfile::new(
        WorldState::new(),
        HeadlessRuntimeConfig {
            snapshot_segment_count: 0,
            emit_snapshot_bytes: true,
        },
    );
    let r = p.step().unwrap();
    assert_eq!(r.kernel_result.tick.0, 1);
}

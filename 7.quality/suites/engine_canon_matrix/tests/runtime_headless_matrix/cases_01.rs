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

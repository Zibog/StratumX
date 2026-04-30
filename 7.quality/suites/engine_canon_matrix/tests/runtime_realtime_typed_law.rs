use engine_core::EngineCoreError;
use engine_runtime_realtime::{
    RealtimeFrameCadence, RealtimeRuntimeConfig, RealtimeRuntimeFailure,
    RealtimeRuntimeFailureReason, RealtimeRuntimeProfile,
};
use engine_world::WorldState;

#[test]
fn realtime_target_fps_zero_returns_typed_failure() {
    let failure = RealtimeFrameCadence::from_fps(0).unwrap_err();

    assert_eq!(
        failure.reason,
        RealtimeRuntimeFailureReason::InvalidTargetFps
    );
}

#[test]
fn realtime_invalid_cadence_returns_typed_failure() {
    let failure = RealtimeRuntimeProfile::new(
        WorldState::new(),
        RealtimeRuntimeConfig {
            target_fps: 60,
            visibility_freshness_frames: 0,
            enqueue_presentable_frames: true,
        },
    )
    .unwrap_err();

    assert_eq!(
        failure.reason,
        RealtimeRuntimeFailureReason::InvalidFrameCadence
    );
}

#[test]
fn realtime_engine_core_error_bridge_preserves_reason() {
    let bridge: EngineCoreError =
        RealtimeRuntimeFailure::for_reason(RealtimeRuntimeFailureReason::InvalidTargetFps).into();

    assert_eq!(
        bridge,
        EngineCoreError::InvalidDescriptor("realtime target_fps must be non-zero")
    );
}

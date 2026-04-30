use engine_animation::{AnimationClip, AnimationRuntime, AnimationTier, Keyframe};
use engine_core::EngineCoreError;

fn test_runtime() -> AnimationRuntime {
    let mut runtime = AnimationRuntime::new();
    runtime.add_clip(AnimationClip {
        name: "walk".to_string(),
        duration_sec: 2.0,
        keyframes: vec![
            Keyframe {
                time_sec: 0.0,
                joint_index: 0,
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
            },
            Keyframe {
                time_sec: 1.0,
                joint_index: 0,
                position: [1.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
            },
            Keyframe {
                time_sec: 2.0,
                joint_index: 0,
                position: [2.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
            },
        ],
        looping: true,
    });
    runtime
}

#[test]
fn animation_missing_clip_fails_with_exact_reason() {
    let runtime = test_runtime();

    let result = runtime.sample_with_receipt("run", 0.5, 20);

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "animation sample requires registered clip",
        ))
    );
}

#[test]
fn animation_invalid_sample_time_fails_with_exact_reason() {
    let runtime = test_runtime();

    let result = runtime.sample_with_receipt("walk", 3.0, 20);
    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "animation sample time lies outside clip duration",
        ))
    );

    let result = runtime.sample_with_receipt("walk", -1.0, 20);
    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "animation sample time lies outside clip duration",
        ))
    );
}

#[test]
fn animation_invalid_joint_count_fails_with_exact_reason() {
    let runtime = test_runtime();

    let result = runtime.sample_with_receipt("walk", 0.5, 0);

    assert_eq!(
        result,
        Err(EngineCoreError::InvalidDescriptor(
            "animation sample requires non-zero joint count",
        ))
    );
}

#[test]
fn animation_budget_pressure_selects_expected_tiers() {
    let runtime = test_runtime();

    let low = runtime.sample_with_receipt("walk", 0.5, 20).unwrap();
    let high = runtime.sample_with_receipt("walk", 0.5, 60).unwrap();
    let very_high = runtime.sample_with_receipt("walk", 0.5, 120).unwrap();

    assert_eq!(low.selected_tier, AnimationTier::FullPose);
    assert_eq!(high.selected_tier, AnimationTier::ReducedPose);
    assert_eq!(very_high.selected_tier, AnimationTier::RootMotionOnly);
}

#[test]
fn same_animation_input_same_digest() {
    let runtime = test_runtime();

    let receipt1 = runtime.sample_with_receipt("walk", 0.5, 20).unwrap();
    let receipt2 = runtime.sample_with_receipt("walk", 0.5, 20).unwrap();

    assert_eq!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn different_sample_time_changes_digest() {
    let runtime = test_runtime();

    let receipt1 = runtime.sample_with_receipt("walk", 0.5, 20).unwrap();
    let receipt2 = runtime.sample_with_receipt("walk", 1.0, 20).unwrap();

    assert_ne!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

#[test]
fn different_joint_count_changes_digest() {
    let runtime = test_runtime();

    let receipt1 = runtime.sample_with_receipt("walk", 0.5, 20).unwrap();
    let receipt2 = runtime.sample_with_receipt("walk", 0.5, 30).unwrap();

    assert_ne!(receipt1.deterministic_digest, receipt2.deterministic_digest);
}

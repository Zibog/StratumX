//! Animation: проверка системы анимации

use engine_animation::{AnimationClip, AnimationRuntime, IkChain, Keyframe};

#[test]
fn animation_playback() {
    let mut runtime = AnimationRuntime::new();
    runtime.add_clip(AnimationClip {
        name: "reload".to_string(),
        duration_sec: 0.5,
        keyframes: vec![Keyframe {
            time_sec: 0.0,
            joint_index: 0,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
        }],
        looping: false,
    });

    assert!(runtime.play_clip("reload"));
    runtime.update(1.0);
    assert!(!runtime.states[0].playing);
}

#[test]
fn animation_blending() {
    let mut chain = IkChain::new();
    let root = chain.add_joint([0.0, 0.0, 0.0], None, 0.0);
    let tip = chain.add_joint([1.0, 0.0, 0.0], Some(root), 1.0);
    chain.set_end_effector(tip);

    let target = chain.solve_reach([1.0, 0.0, 0.0], 10, 0.01);
    assert!(target.reached);
    assert!(target.distance_to_target < 0.01);
}

#[test]
fn animation_state_machine() {
    let mut runtime = AnimationRuntime::new();
    runtime.add_clip(AnimationClip {
        name: "idle".to_string(),
        duration_sec: 2.0,
        keyframes: Vec::new(),
        looping: true,
    });

    assert!(runtime.play_clip("idle"));
    assert!(!runtime.play_clip("missing"));
    assert_eq!(runtime.states.len(), 1);
}

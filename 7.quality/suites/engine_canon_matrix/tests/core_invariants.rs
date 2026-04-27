use engine_core::{Aabb3f, EngineCoreError, FeatureProfile, Generation, Invariant, ACTIVE_PROFILE};
use glam::vec3;

#[test]
fn generation_advances_without_zero_aliasing() {
    assert_eq!(Generation(41).next(), Generation(42));
    assert_eq!(Generation(u32::MAX).next(), Generation::INITIAL);
}

#[test]
fn aabb_invariant_accepts_ordered_bounds() {
    let bounds = Aabb3f {
        min: vec3(-1.0, 0.0, 2.0),
        max: vec3(4.0, 8.0, 9.0),
    };

    assert!(bounds.validate());
    bounds.check_invariants().unwrap();
}

#[test]
fn aabb_invariant_rejects_inverted_bounds() {
    let bounds = Aabb3f {
        min: vec3(4.0, 1.0, 2.0),
        max: vec3(3.0, 8.0, 9.0),
    };

    assert_eq!(
        bounds.check_invariants(),
        Err(EngineCoreError::InvariantViolation(
            "aabb min must not exceed max",
        ))
    );
}

#[test]
fn active_profile_stays_within_legal_feature_set() {
    assert!(matches!(
        ACTIVE_PROFILE,
        FeatureProfile::Minimal | FeatureProfile::Headless | FeatureProfile::Realtime
    ));
}

#![allow(unused_imports)]
use super::*;

#[test]
fn generation_next_preserves_non_zero() {
    assert_ne!(Generation(0).next(), Generation(0));
}
#[test]
fn generation_initial_advances_to_two() {
    assert_eq!(Generation::INITIAL.next(), Generation(2));
}
#[test]
fn ordered_aabb_is_valid() {
    let a = Aabb3f {
        min: Vec3f::new(0.0, 0.0, 0.0),
        max: Vec3f::new(1.0, 1.0, 1.0),
    };
    assert!(a.validate());
}
#[test]
fn inverted_aabb_is_invalid() {
    let a = Aabb3f {
        min: Vec3f::new(2.0, 0.0, 0.0),
        max: Vec3f::new(1.0, 1.0, 1.0),
    };
    assert!(!a.validate());
}
#[test]
fn active_profile_is_legal_variant() {
    assert!(matches!(
        ACTIVE_PROFILE,
        FeatureProfile::Minimal | FeatureProfile::Headless | FeatureProfile::Realtime
    ));
}

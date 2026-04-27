use engine_core::{Aabb3f, EngineCoreError, FeatureProfile, Generation, Tick};

#[test]
fn invalid_aabb_is_rejected_by_validator() {
    let aabb = Aabb3f {
        min: glam::vec3(5.0, 0.0, 0.0),
        max: glam::vec3(4.0, 1.0, 1.0),
    };
    assert!(!aabb.validate());
}

#[test]
fn generation_wraps_without_reaching_zero() {
    let wrapped = Generation(u32::MAX).next();
    assert_eq!(wrapped.0, 1);
}

#[test]
fn error_variants_preserve_contract_text() {
    assert_eq!(
        EngineCoreError::FeatureConflict("x").to_string(),
        "feature profile conflict: x"
    );
    assert_eq!(Tick(9).0, 9);
    assert!(matches!(
        engine_core::ACTIVE_PROFILE,
        FeatureProfile::Minimal | FeatureProfile::Headless | FeatureProfile::Realtime
    ));
}

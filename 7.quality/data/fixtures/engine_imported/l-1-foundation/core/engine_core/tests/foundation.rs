use engine_core::{
    contracts::Invariant, Aabb3f, ComponentTypeId, EngineCoreError, FeatureProfile, Generation,
    Tick, ACTIVE_PROFILE,
};

#[test]
fn foundational_types_round_trip_identity_values() {
    let generation = Generation::INITIAL;
    let tick = Tick(7);
    let component = ComponentTypeId(42);

    assert_eq!(generation.0, 1);
    assert_eq!(generation.next().0, 2);
    assert_eq!(tick.0, 7);
    assert_eq!(component.0, 42);
}

#[test]
fn active_profile_is_a_legal_value() {
    assert!(matches!(
        ACTIVE_PROFILE,
        FeatureProfile::Minimal | FeatureProfile::Headless | FeatureProfile::Realtime
    ));
}

#[test]
fn aabb_invariant_validation_is_deterministic() {
    let aabb = Aabb3f {
        min: glam::vec3(0.0, 1.0, 2.0),
        max: glam::vec3(3.0, 4.0, 5.0),
    };
    assert!(aabb.validate());

    struct Bad;
    impl Invariant for Bad {
        fn check_invariants(&self) -> Result<(), EngineCoreError> {
            Err(EngineCoreError::InvariantViolation("test"))
        }
    }

    assert_eq!(
        Bad.check_invariants(),
        Err(EngineCoreError::InvariantViolation("test"))
    );
}

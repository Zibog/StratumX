use engine_core::Generation;
use engine_handle::{
    StableComponentHandle, StableEntityHandle, ValidationContext, ValidationResult,
};
use engine_identity::{ComponentId, EntityId};

#[test]
fn stable_entity_handle_detects_valid_stale_and_invalidated_states() {
    let current = EntityId {
        slot: 7,
        generation: Generation(2),
    };
    let handle = StableEntityHandle::new(current);

    assert_eq!(
        handle.validate(current, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
    assert_eq!(
        handle.validate(
            EntityId {
                slot: current.slot,
                generation: Generation(3),
            },
            ValidationContext::Diagnostics,
        ),
        ValidationResult::Stale
    );
    assert_eq!(
        handle.validate(current, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );

    let mut invalidated = handle;
    invalidated.invalidate();
    assert_eq!(
        invalidated.validate(current, ValidationContext::PlanBuild),
        ValidationResult::Invalidated
    );
}

#[test]
fn stable_component_handle_roundtrips_through_json() {
    let component = ComponentId {
        slot: 3,
        generation: Generation(4),
    };
    let handle = StableComponentHandle::new(component);

    let json = serde_json::to_string(&handle).unwrap();
    let restored: StableComponentHandle = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, handle);
    assert_eq!(
        restored.validate(component, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}

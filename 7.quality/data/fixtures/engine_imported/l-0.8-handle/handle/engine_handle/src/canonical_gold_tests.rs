#![allow(unused_imports)]
use super::*;
use engine_core::Generation;
use engine_identity::{ComponentId, EntityId};

#[test]
fn entity_handle_validates_at_boundary() {
    let id = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn entity_handle_reports_stale_generation() {
    let id = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(
            EntityId {
                slot: 1,
                generation: id.generation.next()
            },
            ValidationContext::Diagnostics
        ),
        ValidationResult::Stale
    );
}
#[test]
fn entity_handle_reports_invalidated() {
    let id = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let mut h = StableEntityHandle::new(id);
    h.invalidate();
    assert_eq!(
        h.validate(id, ValidationContext::PlanBuild),
        ValidationResult::Invalidated
    );
}
#[test]
fn component_handle_validates_in_diagnostics() {
    let id = ComponentId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let h = StableComponentHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::Diagnostics),
        ValidationResult::Valid
    );
}
#[test]
fn steady_traversal_is_illegal() {
    let id = ComponentId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let h = StableComponentHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}

#[test]
fn handle_illegal_steady_traversal_7() {
    let id = EntityId {
        slot: 8,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_8() {
    let id = EntityId {
        slot: 9,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_9() {
    let id = EntityId {
        slot: 10,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_10() {
    let id = EntityId {
        slot: 11,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_11() {
    let id = EntityId {
        slot: 12,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_12() {
    let id = EntityId {
        slot: 13,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_13() {
    let id = EntityId {
        slot: 14,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_14() {
    let id = EntityId {
        slot: 15,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}

#[test]
fn handle_component_stale_10() {
    let id = ComponentId {
        slot: 11,
        generation: Generation::INITIAL,
    };
    let h = StableComponentHandle::new(id);
    assert_eq!(
        h.validate(
            ComponentId {
                slot: id.slot,
                generation: id.generation.next()
            },
            ValidationContext::Diagnostics
        ),
        ValidationResult::Stale
    );
}
#[test]
fn handle_component_stale_11() {
    let id = ComponentId {
        slot: 12,
        generation: Generation::INITIAL,
    };
    let h = StableComponentHandle::new(id);
    assert_eq!(
        h.validate(
            ComponentId {
                slot: id.slot,
                generation: id.generation.next()
            },
            ValidationContext::Diagnostics
        ),
        ValidationResult::Stale
    );
}
#[test]
fn handle_component_stale_12() {
    let id = ComponentId {
        slot: 13,
        generation: Generation::INITIAL,
    };
    let h = StableComponentHandle::new(id);
    assert_eq!(
        h.validate(
            ComponentId {
                slot: id.slot,
                generation: id.generation.next()
            },
            ValidationContext::Diagnostics
        ),
        ValidationResult::Stale
    );
}
#[test]
fn handle_component_stale_13() {
    let id = ComponentId {
        slot: 14,
        generation: Generation::INITIAL,
    };
    let h = StableComponentHandle::new(id);
    assert_eq!(
        h.validate(
            ComponentId {
                slot: id.slot,
                generation: id.generation.next()
            },
            ValidationContext::Diagnostics
        ),
        ValidationResult::Stale
    );
}
#[test]
fn handle_component_stale_14() {
    let id = ComponentId {
        slot: 15,
        generation: Generation::INITIAL,
    };
    let h = StableComponentHandle::new(id);
    assert_eq!(
        h.validate(
            ComponentId {
                slot: id.slot,
                generation: id.generation.next()
            },
            ValidationContext::Diagnostics
        ),
        ValidationResult::Stale
    );
}
#[test]
fn handle_illegal_steady_traversal_0() {
    let id = EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_1() {
    let id = EntityId {
        slot: 2,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_2() {
    let id = EntityId {
        slot: 3,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_3() {
    let id = EntityId {
        slot: 4,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_4() {
    let id = EntityId {
        slot: 5,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_5() {
    let id = EntityId {
        slot: 6,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}
#[test]
fn handle_illegal_steady_traversal_6() {
    let id = EntityId {
        slot: 7,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}

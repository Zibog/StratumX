mod common;
use common::*;

#[test]
fn handle_entity_valid_0() {
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
fn handle_entity_valid_1() {
    let id = EntityId {
        slot: 2,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_2() {
    let id = EntityId {
        slot: 3,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_3() {
    let id = EntityId {
        slot: 4,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_4() {
    let id = EntityId {
        slot: 5,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_5() {
    let id = EntityId {
        slot: 6,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_6() {
    let id = EntityId {
        slot: 7,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_7() {
    let id = EntityId {
        slot: 8,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_8() {
    let id = EntityId {
        slot: 9,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_9() {
    let id = EntityId {
        slot: 10,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_10() {
    let id = EntityId {
        slot: 11,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_11() {
    let id = EntityId {
        slot: 12,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_12() {
    let id = EntityId {
        slot: 13,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_13() {
    let id = EntityId {
        slot: 14,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_entity_valid_14() {
    let id = EntityId {
        slot: 15,
        generation: Generation::INITIAL,
    };
    let h = StableEntityHandle::new(id);
    assert_eq!(
        h.validate(id, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}
#[test]
fn handle_component_stale_0() {
    let id = ComponentId {
        slot: 1,
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
fn handle_component_stale_1() {
    let id = ComponentId {
        slot: 2,
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
fn handle_component_stale_2() {
    let id = ComponentId {
        slot: 3,
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
fn handle_component_stale_3() {
    let id = ComponentId {
        slot: 4,
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
fn handle_component_stale_4() {
    let id = ComponentId {
        slot: 5,
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
fn handle_component_stale_5() {
    let id = ComponentId {
        slot: 6,
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
fn handle_component_stale_6() {
    let id = ComponentId {
        slot: 7,
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
fn handle_component_stale_7() {
    let id = ComponentId {
        slot: 8,
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
fn handle_component_stale_8() {
    let id = ComponentId {
        slot: 9,
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
fn handle_component_stale_9() {
    let id = ComponentId {
        slot: 10,
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

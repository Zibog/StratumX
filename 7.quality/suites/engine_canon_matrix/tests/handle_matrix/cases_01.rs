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

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

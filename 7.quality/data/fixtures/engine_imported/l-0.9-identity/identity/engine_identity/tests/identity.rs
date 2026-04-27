use engine_identity::{IdentityAllocator, IdentityDomain};

#[test]
fn entity_reuse_is_generation_protected_and_delayed() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let first = allocator.issue_entity().unwrap();
    assert!(allocator.is_live_entity(first));
    assert!(allocator.retire_entity(first));

    let second = allocator.issue_entity().unwrap();
    assert_ne!(first.slot, second.slot, "same epoch must not recycle slot");

    allocator.advance_epoch();
    let third = allocator.issue_entity().unwrap();
    assert_eq!(
        first.slot, third.slot,
        "slot can recycle after epoch advance"
    );
    assert_ne!(
        first.generation, third.generation,
        "generation must increase on reuse"
    );
}

#[test]
fn stale_identity_is_rejected() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let id = allocator.issue_entity().unwrap();
    assert!(allocator.retire_entity(id));
    allocator.advance_epoch();
    let _new = allocator.issue_entity().unwrap();
    assert!(!allocator.is_live_entity(id));
}

#[test]
fn domain_separation_is_typed() {
    let mut entities = IdentityAllocator::new(IdentityDomain::Entity);
    let mut components = IdentityAllocator::new(IdentityDomain::Component);
    assert!(entities.issue_component().is_none());
    assert!(components.issue_entity().is_none());
}

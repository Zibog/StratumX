use engine_identity::{IdentityAllocator, IdentityDomain};

#[test]
fn component_domain_issues_component_ids_only() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Component);
    let component = allocator.issue_component().expect("component id");
    assert!(allocator.is_live_component(component));
    assert!(allocator.retire_component(component));
    assert!(!allocator.is_live_component(component));
}

#[test]
fn recycled_slot_requires_epoch_advance() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let first = allocator.issue_entity().unwrap();
    assert!(allocator.retire_entity(first));
    let second = allocator.issue_entity().unwrap();
    assert_ne!(first.slot, second.slot);
    allocator.advance_epoch();
    let third = allocator.issue_entity().unwrap();
    assert_eq!(first.slot, third.slot);
}

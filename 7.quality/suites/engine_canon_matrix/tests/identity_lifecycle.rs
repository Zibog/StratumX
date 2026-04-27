use engine_identity::{IdentityAllocator, IdentityDomain};

#[test]
fn allocator_reuses_slots_only_after_epoch_advance() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);

    let first = allocator.issue_entity().unwrap();
    assert!(allocator.is_live_entity(first));
    assert!(allocator.retire_entity(first));
    assert!(!allocator.is_live_entity(first));

    let second = allocator.issue_entity().unwrap();
    assert_ne!(second.slot, first.slot);

    allocator.retire_entity(second);
    allocator.advance_epoch();
    let recycled = allocator.issue_entity().unwrap();

    assert_eq!(recycled.slot, first.slot);
    assert_eq!(recycled.generation, first.generation.next());
    assert!(allocator.is_live_entity(recycled));
}

#[test]
fn allocators_only_issue_matching_identity_domain() {
    let mut entity_allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let mut component_allocator = IdentityAllocator::new(IdentityDomain::Component);

    assert!(entity_allocator.issue_entity().is_some());
    assert!(entity_allocator.issue_component().is_none());
    assert!(component_allocator.issue_component().is_some());
    assert!(component_allocator.issue_entity().is_none());
}

#[test]
fn entity_and_component_ids_roundtrip_through_json() {
    let mut entity_allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let mut component_allocator = IdentityAllocator::new(IdentityDomain::Component);
    let entity = entity_allocator.issue_entity().unwrap();
    let component = component_allocator.issue_component().unwrap();

    let entity_json = serde_json::to_string(&entity).unwrap();
    let component_json = serde_json::to_string(&component).unwrap();

    assert_eq!(
        serde_json::from_str::<engine_identity::EntityId>(&entity_json).unwrap(),
        entity
    );
    assert_eq!(
        serde_json::from_str::<engine_identity::ComponentId>(&component_json).unwrap(),
        component
    );
}

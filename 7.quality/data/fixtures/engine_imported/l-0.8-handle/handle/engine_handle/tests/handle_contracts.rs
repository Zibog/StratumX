use engine_handle::{
    StableComponentHandle, StableEntityHandle, ValidationContext, ValidationResult,
};
use engine_identity::{IdentityAllocator, IdentityDomain};

#[test]
fn component_handle_detects_generation_turnover() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Component);
    let issued = allocator.issue_component().unwrap();
    let handle = StableComponentHandle::new(issued);
    allocator.retire_component(issued);
    allocator.advance_epoch();
    let recycled = allocator.issue_component().unwrap();
    assert_eq!(
        handle.validate(recycled, ValidationContext::BoundaryEntry),
        ValidationResult::Stale
    );
}

#[test]
fn entity_handle_validates_in_boundary_context() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let issued = allocator.issue_entity().unwrap();
    let handle = StableEntityHandle::new(issued);
    assert_eq!(
        handle.validate(issued, ValidationContext::BoundaryEntry),
        ValidationResult::Valid
    );
}

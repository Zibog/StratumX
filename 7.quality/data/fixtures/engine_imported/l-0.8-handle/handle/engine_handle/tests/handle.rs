use engine_handle::{StableEntityHandle, ValidationContext, ValidationResult};
use engine_identity::{IdentityAllocator, IdentityDomain};

#[test]
fn handle_detects_stale_identity_turnover() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let issued = allocator.issue_entity().unwrap();
    let handle = StableEntityHandle::new(issued);

    allocator.retire_entity(issued);
    allocator.advance_epoch();
    let recycled = allocator.issue_entity().unwrap();
    assert_eq!(issued.slot, recycled.slot);

    assert_eq!(
        handle.validate(recycled, ValidationContext::BoundaryEntry),
        ValidationResult::Stale
    );
}

#[test]
fn handle_invalidation_transitions_to_non_authoritative() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let issued = allocator.issue_entity().unwrap();
    let mut handle = StableEntityHandle::new(issued);
    handle.invalidate();

    assert_eq!(
        handle.validate(issued, ValidationContext::Diagnostics),
        ValidationResult::Invalidated
    );
}

#[test]
fn repeated_validation_in_steady_traversal_is_illegal() {
    let mut allocator = IdentityAllocator::new(IdentityDomain::Entity);
    let issued = allocator.issue_entity().unwrap();
    let handle = StableEntityHandle::new(issued);

    assert_eq!(
        handle.validate(issued, ValidationContext::SteadyTraversal),
        ValidationResult::IllegalContext
    );
}

use engine_handle::StableEntityHandle;
use engine_identity::{IdentityAllocator, IdentityDomain};
use engine_storage_access::{
    make_read_view, make_write_window, traversal_entry_bind, AccessDescriptor, AccessMode,
    ScratchClass, TraversalPlanId,
};
use engine_storage_layout::LocalityClass;

#[test]
fn write_window_rejects_direct_write_path() {
    let mut alloc = IdentityAllocator::new(IdentityDomain::Entity);
    let handle = StableEntityHandle::new(alloc.issue_entity().unwrap());
    let descriptor = AccessDescriptor {
        mode: AccessMode::WRITE,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Partition,
        scratch: ScratchClass::Borrowed,
        staged_mutation_handoff: false,
    };

    assert!(make_write_window(descriptor, handle).is_err());
}

#[test]
fn read_view_is_legal_only_with_read_mode() {
    let mut alloc = IdentityAllocator::new(IdentityDomain::Entity);
    let handle = StableEntityHandle::new(alloc.issue_entity().unwrap());
    let descriptor = AccessDescriptor {
        mode: AccessMode::STAGED,
        plan_id: TraversalPlanId(2),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: true,
    };

    assert!(make_read_view(descriptor, handle).is_err());
}

#[test]
fn traversal_bind_rejects_cache_hit_recompile() {
    let descriptor = AccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(3),
        locality: LocalityClass::TraversalLane,
        scratch: ScratchClass::Borrowed,
        staged_mutation_handoff: true,
    };

    assert!(traversal_entry_bind(&descriptor, true, true, LocalityClass::TraversalLane).is_err());
    assert!(traversal_entry_bind(&descriptor, true, false, LocalityClass::TraversalLane).is_ok());
}

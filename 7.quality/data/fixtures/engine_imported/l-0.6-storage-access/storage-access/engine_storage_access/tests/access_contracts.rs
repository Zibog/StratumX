use engine_handle::StableEntityHandle;
use engine_identity::{IdentityAllocator, IdentityDomain};
use engine_storage_access::{
    make_read_view, make_write_window, traversal_entry_bind, AccessDescriptor, AccessMode,
    ScratchClass, TraversalPlanId,
};
use engine_storage_layout::LocalityClass;

fn handle() -> StableEntityHandle {
    let mut alloc = IdentityAllocator::new(IdentityDomain::Entity);
    StableEntityHandle::new(alloc.issue_entity().unwrap())
}

#[test]
fn staged_write_window_is_legal_with_handoff() {
    let descriptor = AccessDescriptor {
        mode: AccessMode::STAGED,
        plan_id: TraversalPlanId(11),
        locality: LocalityClass::Partition,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: true,
    };
    assert!(make_write_window(descriptor, handle()).is_ok());
}

#[test]
fn locality_widen_after_bind_is_rejected() {
    let descriptor = AccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(12),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Borrowed,
        staged_mutation_handoff: false,
    };
    assert!(traversal_entry_bind(&descriptor, false, false, LocalityClass::Spatial).is_err());
}

#[test]
fn mixed_mode_supports_read_view() {
    let descriptor = AccessDescriptor {
        mode: AccessMode::MIXED,
        plan_id: TraversalPlanId(13),
        locality: LocalityClass::TraversalLane,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: true,
    };
    assert!(make_read_view(descriptor, handle()).is_ok());
}

use engine_core::{EngineCoreError, Generation};
use engine_handle::StableEntityHandle;
use engine_identity::EntityId;
use engine_storage_access::{
    make_read_view, make_write_window, traversal_entry_bind, AccessDescriptor, AccessMode,
    ScratchClass, TraversalPlanId,
};
use engine_storage_layout::LocalityClass;

fn anchor() -> StableEntityHandle {
    StableEntityHandle::new(EntityId {
        slot: 5,
        generation: Generation(1),
    })
}

#[test]
fn read_view_requires_read_mode() {
    let descriptor = AccessDescriptor {
        mode: AccessMode::WRITE,
        plan_id: TraversalPlanId(7),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Borrowed,
        staged_mutation_handoff: true,
    };

    assert_eq!(
        make_read_view(descriptor, anchor()).map(|_| ()),
        Err(EngineCoreError::InvalidDescriptor(
            "read view requires READ mode",
        ))
    );
}

#[test]
fn write_window_requires_staged_handoff_for_write_mode() {
    let direct_write = AccessDescriptor {
        mode: AccessMode::WRITE,
        plan_id: TraversalPlanId(8),
        locality: LocalityClass::Spatial,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: false,
    };
    assert_eq!(
        make_write_window(direct_write, anchor()).map(|_| ()),
        Err(EngineCoreError::InvalidDescriptor(
            "direct write entry is illegal; staged handoff required",
        ))
    );

    let staged = AccessDescriptor {
        mode: AccessMode::STAGED,
        plan_id: TraversalPlanId(9),
        locality: LocalityClass::Spatial,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: true,
    };
    make_write_window(staged, anchor()).unwrap();
}

#[test]
fn traversal_bind_rejects_recompile_on_cache_hit_and_locality_widening() {
    let descriptor = AccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(11),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Borrowed,
        staged_mutation_handoff: false,
    };

    assert_eq!(
        traversal_entry_bind(&descriptor, true, true, LocalityClass::Cache),
        Err(EngineCoreError::InvalidDescriptor(
            "cache hit must not trigger ad hoc compile",
        ))
    );
    assert_eq!(
        traversal_entry_bind(&descriptor, false, false, LocalityClass::Spatial),
        Err(EngineCoreError::InvalidDescriptor(
            "locality class may not widen after bind",
        ))
    );
    traversal_entry_bind(&descriptor, false, false, LocalityClass::Cache).unwrap();
}

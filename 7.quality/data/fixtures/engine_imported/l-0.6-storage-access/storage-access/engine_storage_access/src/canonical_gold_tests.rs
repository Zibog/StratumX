#![allow(unused_imports)]
use super::*;
use engine_core::Generation;
use engine_handle::StableEntityHandle;
use engine_identity::EntityId;
use engine_storage_layout::LocalityClass;

fn anchor() -> StableEntityHandle {
    StableEntityHandle::new(EntityId {
        slot: 1,
        generation: Generation::INITIAL,
    })
}
#[test]
fn read_view_requires_read() {
    let d = AccessDescriptor {
        mode: AccessMode::WRITE,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: true,
    };
    assert!(make_read_view(d, anchor()).is_err());
}
#[test]
fn read_view_accepts_read() {
    let d = AccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: false,
    };
    assert!(make_read_view(d, anchor()).is_ok());
}
#[test]
fn write_window_requires_write_or_staged() {
    let d = AccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: false,
    };
    assert!(make_write_window(d, anchor()).is_err());
}
#[test]
fn direct_write_requires_handoff() {
    let d = AccessDescriptor {
        mode: AccessMode::WRITE,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: false,
    };
    assert!(make_write_window(d, anchor()).is_err());
}
#[test]
fn locality_cannot_widen_after_bind() {
    let d = AccessDescriptor {
        mode: AccessMode::READ,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Owned,
        staged_mutation_handoff: false,
    };
    assert!(traversal_entry_bind(&d, false, false, LocalityClass::Spatial).is_err());
}

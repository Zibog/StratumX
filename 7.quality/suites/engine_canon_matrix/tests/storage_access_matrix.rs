#![allow(unused_imports, unused_mut, unused_variables)]
mod common;
use common::*;
use proptest::prelude::*;

fn storage_access_read_view_strategy() -> impl Strategy<Value = usize> {
    0usize..20
}

fn storage_access_write_window_strategy() -> impl Strategy<Value = usize> {
    0usize..20
}

fn storage_access_bind_locality_strategy() -> impl Strategy<Value = usize> {
    0usize..20
}

proptest! {
    #[test]
    fn storage_access_all_read_view_cases(|case in storage_access_read_view_strategy()) {
        let h = StableEntityHandle::new(EntityId {
            slot: 1,
            generation: Generation::INITIAL,
        });
        let mode = if case % 2 == 0 {
            AccessMode::READ
        } else {
            AccessMode::MIXED
        };
        let d = StorageAccessDescriptor {
            mode,
            plan_id: TraversalPlanId(1),
            locality: LocalityClass::Cache,
            scratch: ScratchClass::Owned,
            staged_mutation_handoff: true,
        };
        prop_assert!(make_read_view(d, h).is_ok());
    }

    #[test]
    fn storage_access_all_write_window_cases(|case in storage_access_write_window_strategy()) {
        let h = StableEntityHandle::new(EntityId {
            slot: 1,
            generation: Generation::INITIAL,
        });
        let handoff = case % 2 == 0;
        let d = StorageAccessDescriptor {
            mode: AccessMode::WRITE,
            plan_id: TraversalPlanId(1),
            locality: LocalityClass::Cache,
            scratch: ScratchClass::Owned,
            staged_mutation_handoff: handoff,
        };
        if handoff {
            prop_assert!(make_write_window(d, h).is_ok());
        } else {
            prop_assert!(make_write_window(d, h).is_err());
        }
    }

    #[test]
    fn storage_access_all_bind_locality_cases(|case in storage_access_bind_locality_strategy()) {
        let d = StorageAccessDescriptor {
            mode: AccessMode::READ,
            plan_id: TraversalPlanId(1),
            locality: LocalityClass::Cache,
            scratch: ScratchClass::Owned,
            staged_mutation_handoff: false,
        };
        let target = if case % 2 == 0 {
            LocalityClass::Cache
        } else {
            LocalityClass::Spatial
        };
        if target == LocalityClass::Cache {
            prop_assert!(traversal_entry_bind(&d, false, false, target).is_ok());
        } else {
            prop_assert!(traversal_entry_bind(&d, false, false, target).is_err());
        }
    }
}

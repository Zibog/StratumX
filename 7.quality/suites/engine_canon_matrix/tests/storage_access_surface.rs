use engine_core::Generation;
use engine_handle::{InvalidationState, StableEntityHandle};
use engine_storage_access::*;
use engine_storage_layout::LocalityClass;

fn make_descriptor(mode: AccessMode) -> AccessDescriptor {
    AccessDescriptor {
        mode,
        plan_id: TraversalPlanId(1),
        locality: LocalityClass::Cache,
        scratch: ScratchClass::Borrowed,
        staged_mutation_handoff: true,
    }
}

fn make_handle() -> StableEntityHandle {
    StableEntityHandle {
        id: engine_identity::EntityId {
            slot: 1,
            generation: Generation(0),
        },
        observed_generation: Generation(0),
        state: InvalidationState::Active,
    }
}

// === AccessMode Tests ===

include!("storage_access_surface/cases_01.rs");
include!("storage_access_surface/cases_02.rs");

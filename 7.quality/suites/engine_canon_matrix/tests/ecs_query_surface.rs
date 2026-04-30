use engine_core::ComponentTypeId;
use engine_ecs_query::*;
use engine_storage_access::ScratchClass;
use smallvec::smallvec;

// === QueryAccessMode Tests ===

include!("ecs_query_surface/cases_01.rs");
include!("ecs_query_surface/cases_02.rs");

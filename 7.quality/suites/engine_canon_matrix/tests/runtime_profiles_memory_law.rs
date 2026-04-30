// Runtime Profiles and Memory Law Tests

use engine_memory_control::{
    AllocationDescriptor, MemoryAllocationPool, MemoryConfig, MemoryControlService, PressureClass,
};
use engine_runtime_headless::{
    HeadlessRuntimeConfig, HeadlessRuntimeProfile, HEADLESS_TICK_BUDGET_MS,
};
use engine_runtime_realtime::{
    PresentationBudgetDecision, RealtimeFrameCadence, RealtimeRuntimeConfig,
    RealtimeRuntimeFailureReason, RealtimeRuntimeProfile,
};
use engine_world::WorldState;

include!("runtime_profiles_memory_law/cases_01.rs");
include!("runtime_profiles_memory_law/cases_02.rs");

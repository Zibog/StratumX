// Runtime Profile Law Tests
//
// Tests for BLKR-RUNTIME-PROFILES-01:
// - Headless runtime determinism
// - Realtime budget/degrade behavior
// - Memory pressure classification
// - Invalid runtime state denial
// - Profile validation

use engine_core::EngineCoreError;
use engine_memory_control::{
    AllocationDescriptor, MemoryConfig, MemoryControlService, PressureClass,
};
use engine_runtime::{
    BudgetEnvelope, DomainBudgetUsage, PressureBucket, RuntimeKernel, RuntimeMode,
};
use engine_runtime_headless::{HeadlessRuntimeConfig, HeadlessRuntimeProfile};
use engine_runtime_realtime::{RealtimeRuntimeConfig, RealtimeRuntimeProfile};
use engine_world::WorldState;

include!("runtime_profile_law/cases_01.rs");
include!("runtime_profile_law/cases_02.rs");
include!("runtime_profile_law/cases_03.rs");

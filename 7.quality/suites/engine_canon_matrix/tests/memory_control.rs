// Memory Control Service Tests

use engine_core::EngineCoreError;
use engine_memory_control::{
    AllocationDescriptor, MemoryAllocationId, MemoryAllocationPool, MemoryConfig,
    MemoryControlService, MemoryFailureReason, PressureClass,
};
use engine_storage_layout::LayoutClass;

include!("memory_control/cases_01.rs");
include!("memory_control/cases_02.rs");

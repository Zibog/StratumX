use crate::pressure::PressureClass;
use engine_storage_layout::LayoutClass;
use serde::{Deserialize, Serialize};

/// Unique identifier for a memory allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MemoryAllocationId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryAllocationPool {
    Heap,
    Staging,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryAllocationRecord {
    pub allocation_id: MemoryAllocationId,
    pub pool: MemoryAllocationPool,
    pub bytes_reserved: usize,
    pub layout_class: Option<LayoutClass>,
}

/// Receipt confirming a memory reservation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryReservationReceipt {
    pub allocation_id: MemoryAllocationId,
    pub pool: MemoryAllocationPool,
    pub bytes_reserved: usize,
    pub pressure_after: PressureClass,
}

/// Receipt confirming a memory release.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryReleaseReceipt {
    pub allocation_id: MemoryAllocationId,
    pub pool: MemoryAllocationPool,
    pub bytes_released: usize,
    pub pressure_after: PressureClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub heap_budget_bytes: usize,
    pub staging_budget_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationDescriptor {
    pub allocation_id: u64,
    pub bytes: usize,
    pub layout_class: Option<LayoutClass>,
}

impl AllocationDescriptor {
    pub fn memory_allocation_id(&self) -> MemoryAllocationId {
        MemoryAllocationId(self.allocation_id)
    }
}

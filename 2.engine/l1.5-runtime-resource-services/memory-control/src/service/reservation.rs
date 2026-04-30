use super::{budget_message, memory_failure, MemoryControlService};
use crate::error::{MemoryControlResult, MemoryFailureReason};
use crate::model::{AllocationDescriptor, MemoryAllocationPool, MemoryAllocationRecord};

impl MemoryControlService {
    pub(crate) fn reserve(
        &mut self,
        descriptor: &AllocationDescriptor,
        pool: MemoryAllocationPool,
    ) -> MemoryControlResult<()> {
        let allocation_id = descriptor.memory_allocation_id();
        if allocation_id.0 == 0 {
            return Err(memory_failure(
                MemoryFailureReason::ZeroAllocation,
                "memory allocation id must be non-zero",
            ));
        }
        if descriptor.bytes == 0 {
            return Err(memory_failure(
                MemoryFailureReason::ZeroAllocation,
                "memory allocation size must be non-zero",
            ));
        }
        if self.allocations.contains_key(&allocation_id) {
            return Err(memory_failure(
                MemoryFailureReason::DuplicateAllocationId,
                "memory allocation id already reserved",
            ));
        }
        let next = self.bytes_for_pool(pool).saturating_add(descriptor.bytes);
        if next > self.pool_budget(pool) {
            return Err(memory_failure(
                MemoryFailureReason::OverBudget,
                budget_message(pool),
            ));
        }
        self.set_bytes_for_pool(pool, next);
        self.allocations.insert(
            allocation_id,
            MemoryAllocationRecord {
                allocation_id,
                pool,
                bytes_reserved: descriptor.bytes,
                layout_class: descriptor.layout_class,
            },
        );
        self.released_allocations.remove(&allocation_id);
        Ok(())
    }
}

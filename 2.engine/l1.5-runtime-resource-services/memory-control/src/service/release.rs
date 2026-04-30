use super::{memory_failure, MemoryControlService};
use crate::error::{MemoryControlResult, MemoryFailure, MemoryFailureReason};
use crate::model::{MemoryAllocationId, MemoryAllocationPool};

impl MemoryControlService {
    pub(crate) fn release(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
        pool: MemoryAllocationPool,
    ) -> MemoryControlResult<()> {
        if bytes == 0 {
            return Err(memory_failure(
                MemoryFailureReason::ZeroAllocation,
                "memory release must be non-zero",
            ));
        }
        let Some(record) = self.allocations.get(&allocation_id) else {
            return Err(self.missing_release_failure(allocation_id));
        };
        if record.pool != pool {
            return Err(memory_failure(
                MemoryFailureReason::PoolMismatch,
                "memory release pool does not match reservation",
            ));
        }
        if record.bytes_reserved != bytes {
            return Err(memory_failure(
                MemoryFailureReason::SizeMismatch,
                "memory release size does not match reservation",
            ));
        }
        self.allocations.remove(&allocation_id);
        let next = self.bytes_for_pool(pool).saturating_sub(bytes);
        self.set_bytes_for_pool(pool, next);
        self.released_allocations.insert(allocation_id);
        Ok(())
    }

    fn missing_release_failure(&self, allocation_id: MemoryAllocationId) -> MemoryFailure {
        if self.released_allocations.contains(&allocation_id) {
            memory_failure(
                MemoryFailureReason::AlreadyReleased,
                "memory allocation already released",
            )
        } else {
            memory_failure(
                MemoryFailureReason::UnknownAllocation,
                "memory release references unknown allocation",
            )
        }
    }
}

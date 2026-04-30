use super::MemoryControlService;
use crate::error::MemoryControlResult;
use crate::model::{
    AllocationDescriptor, MemoryAllocationId, MemoryAllocationPool, MemoryReleaseReceipt,
    MemoryReservationReceipt,
};

impl MemoryControlService {
    pub fn try_reserve_heap(
        &mut self,
        descriptor: &AllocationDescriptor,
    ) -> MemoryControlResult<()> {
        self.reserve(descriptor, MemoryAllocationPool::Heap)
    }

    pub fn try_reserve_staging(
        &mut self,
        descriptor: &AllocationDescriptor,
    ) -> MemoryControlResult<()> {
        self.reserve(descriptor, MemoryAllocationPool::Staging)
    }

    pub fn try_reserve_with_receipt(
        &mut self,
        descriptor: &AllocationDescriptor,
    ) -> MemoryControlResult<MemoryReservationReceipt> {
        self.reserve_with_pool_receipt(descriptor, MemoryAllocationPool::Heap)
    }

    pub fn try_reserve_staging_with_receipt(
        &mut self,
        descriptor: &AllocationDescriptor,
    ) -> MemoryControlResult<MemoryReservationReceipt> {
        self.reserve_with_pool_receipt(descriptor, MemoryAllocationPool::Staging)
    }

    pub fn try_release_heap(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> MemoryControlResult<()> {
        self.release(allocation_id, bytes, MemoryAllocationPool::Heap)
    }

    pub fn try_release_staging(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> MemoryControlResult<()> {
        self.release(allocation_id, bytes, MemoryAllocationPool::Staging)
    }

    pub fn try_release_with_receipt(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> MemoryControlResult<MemoryReleaseReceipt> {
        self.release_with_pool_receipt(allocation_id, bytes, MemoryAllocationPool::Heap)
    }

    pub fn try_release_staging_with_receipt(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> MemoryControlResult<MemoryReleaseReceipt> {
        self.release_with_pool_receipt(allocation_id, bytes, MemoryAllocationPool::Staging)
    }

    fn reserve_with_pool_receipt(
        &mut self,
        descriptor: &AllocationDescriptor,
        pool: MemoryAllocationPool,
    ) -> MemoryControlResult<MemoryReservationReceipt> {
        self.reserve(descriptor, pool)?;
        Ok(MemoryReservationReceipt {
            allocation_id: descriptor.memory_allocation_id(),
            pool,
            bytes_reserved: descriptor.bytes,
            pressure_after: self.metrics().pressure,
        })
    }

    fn release_with_pool_receipt(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
        pool: MemoryAllocationPool,
    ) -> MemoryControlResult<MemoryReleaseReceipt> {
        self.release(allocation_id, bytes, pool)?;
        Ok(MemoryReleaseReceipt {
            allocation_id,
            pool,
            bytes_released: bytes,
            pressure_after: self.metrics().pressure,
        })
    }
}

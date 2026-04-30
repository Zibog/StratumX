use super::MemoryControlService;
use crate::model::{
    AllocationDescriptor, MemoryAllocationId, MemoryReleaseReceipt, MemoryReservationReceipt,
};
use engine_core::{EngineCoreError, EngineCoreResult};

impl MemoryControlService {
    pub fn reserve_heap(&mut self, descriptor: &AllocationDescriptor) -> EngineCoreResult<()> {
        self.try_reserve_heap(descriptor)
            .map_err(EngineCoreError::from)
    }

    pub fn reserve_staging(&mut self, descriptor: &AllocationDescriptor) -> EngineCoreResult<()> {
        self.try_reserve_staging(descriptor)
            .map_err(EngineCoreError::from)
    }

    pub fn reserve_with_receipt(
        &mut self,
        descriptor: &AllocationDescriptor,
    ) -> EngineCoreResult<MemoryReservationReceipt> {
        self.try_reserve_with_receipt(descriptor)
            .map_err(EngineCoreError::from)
    }

    pub fn reserve_staging_with_receipt(
        &mut self,
        descriptor: &AllocationDescriptor,
    ) -> EngineCoreResult<MemoryReservationReceipt> {
        self.try_reserve_staging_with_receipt(descriptor)
            .map_err(EngineCoreError::from)
    }

    pub fn release_heap(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> EngineCoreResult<()> {
        self.try_release_heap(allocation_id, bytes)
            .map_err(EngineCoreError::from)
    }

    pub fn release_staging(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> EngineCoreResult<()> {
        self.try_release_staging(allocation_id, bytes)
            .map_err(EngineCoreError::from)
    }

    pub fn release_with_receipt(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> EngineCoreResult<MemoryReleaseReceipt> {
        self.try_release_with_receipt(allocation_id, bytes)
            .map_err(EngineCoreError::from)
    }

    pub fn release_staging_with_receipt(
        &mut self,
        allocation_id: MemoryAllocationId,
        bytes: usize,
    ) -> EngineCoreResult<MemoryReleaseReceipt> {
        self.try_release_staging_with_receipt(allocation_id, bytes)
            .map_err(EngineCoreError::from)
    }
}

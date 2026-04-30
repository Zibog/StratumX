mod compat;
mod metrics;
mod release;
mod reservation;
mod typed;

use crate::error::{MemoryFailure, MemoryFailureReason};
use crate::model::{
    MemoryAllocationId, MemoryAllocationPool, MemoryAllocationRecord, MemoryConfig,
};
use crate::pressure::PressureClass;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct MemoryControlService {
    pub(crate) config: MemoryConfig,
    pub(crate) heap_bytes: usize,
    pub(crate) staging_bytes: usize,
    pub(crate) allocations: BTreeMap<MemoryAllocationId, MemoryAllocationRecord>,
    pub(crate) released_allocations: BTreeSet<MemoryAllocationId>,
    pub(crate) previous_pressure: PressureClass,
}

impl MemoryControlService {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            config,
            heap_bytes: 0,
            staging_bytes: 0,
            allocations: BTreeMap::new(),
            released_allocations: BTreeSet::new(),
            previous_pressure: PressureClass::Healthy,
        }
    }

    pub fn allocation(&self, allocation_id: MemoryAllocationId) -> Option<&MemoryAllocationRecord> {
        self.allocations.get(&allocation_id)
    }

    pub fn allocation_count(&self) -> usize {
        self.allocations.len()
    }

    pub(crate) fn bytes_for_pool(&self, pool: MemoryAllocationPool) -> usize {
        match pool {
            MemoryAllocationPool::Heap => self.heap_bytes,
            MemoryAllocationPool::Staging => self.staging_bytes,
        }
    }

    pub(crate) fn set_bytes_for_pool(&mut self, pool: MemoryAllocationPool, bytes: usize) {
        match pool {
            MemoryAllocationPool::Heap => self.heap_bytes = bytes,
            MemoryAllocationPool::Staging => self.staging_bytes = bytes,
        }
    }

    pub(crate) fn pool_budget(&self, pool: MemoryAllocationPool) -> usize {
        match pool {
            MemoryAllocationPool::Heap => self.config.heap_budget_bytes,
            MemoryAllocationPool::Staging => self.config.staging_budget_bytes,
        }
    }
}

pub(crate) fn memory_failure(reason: MemoryFailureReason, message: &'static str) -> MemoryFailure {
    MemoryFailure::new(reason, message)
}

pub(crate) fn budget_message(pool: MemoryAllocationPool) -> &'static str {
    match pool {
        MemoryAllocationPool::Heap => "heap reservation exceeds configured budget",
        MemoryAllocationPool::Staging => "staging reservation exceeds configured budget",
    }
}

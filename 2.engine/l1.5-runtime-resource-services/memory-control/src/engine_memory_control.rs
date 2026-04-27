use engine_core::{EngineCoreError, EngineCoreResult};
use engine_storage_layout::LayoutClass;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureClass {
    Healthy,
    Elevated,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub heap_bytes: usize,
    pub staging_bytes: usize,
    pub pressure: PressureClass,
}

#[derive(Debug)]
pub struct MemoryControlService {
    config: MemoryConfig,
    heap_bytes: usize,
    staging_bytes: usize,
}

impl MemoryControlService {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            config,
            heap_bytes: 0,
            staging_bytes: 0,
        }
    }
    pub fn reserve_heap(&mut self, descriptor: &AllocationDescriptor) -> EngineCoreResult<()> {
        let next = self.heap_bytes.saturating_add(descriptor.bytes);
        if next > self.config.heap_budget_bytes {
            return Err(EngineCoreError::InvalidDescriptor(
                "heap reservation exceeds configured budget",
            ));
        }
        self.heap_bytes = next;
        Ok(())
    }
    pub fn reserve_staging(&mut self, descriptor: &AllocationDescriptor) -> EngineCoreResult<()> {
        let next = self.staging_bytes.saturating_add(descriptor.bytes);
        if next > self.config.staging_budget_bytes {
            return Err(EngineCoreError::InvalidDescriptor(
                "staging reservation exceeds configured budget",
            ));
        }
        self.staging_bytes = next;
        Ok(())
    }
    pub fn release_heap(&mut self, bytes: usize) {
        self.heap_bytes = self.heap_bytes.saturating_sub(bytes);
    }
    pub fn release_staging(&mut self, bytes: usize) {
        self.staging_bytes = self.staging_bytes.saturating_sub(bytes);
    }
    pub fn metrics(&self) -> MemoryMetrics {
        let total = self.heap_bytes.saturating_add(self.staging_bytes);
        let budget = self
            .config
            .heap_budget_bytes
            .saturating_add(self.config.staging_budget_bytes)
            .max(1);
        let usage = total.saturating_mul(100) / budget;
        let pressure = if usage >= 90 {
            PressureClass::Critical
        } else if usage >= 70 {
            PressureClass::Elevated
        } else {
            PressureClass::Healthy
        };
        MemoryMetrics {
            heap_bytes: self.heap_bytes,
            staging_bytes: self.staging_bytes,
            pressure,
        }
    }
}

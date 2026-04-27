use engine_memory_control::{MemoryControlService, PressureClass};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyConfig {
    pub resident_item_budget: usize,
    pub streaming_item_budget: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResidencySet {
    Hot,
    StreamingResident,
    StagingBacked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyDescriptor {
    pub asset_key: u64,
    pub residency_set: ResidencySet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyMetrics {
    pub resident_items: usize,
    pub streaming_items: usize,
    pub pressure: PressureClass,
}

#[derive(Debug)]
pub struct ResidencyControlService {
    config: ResidencyConfig,
    descriptors: BTreeMap<u64, ResidencyDescriptor>,
}

impl ResidencyControlService {
    pub fn new(config: ResidencyConfig) -> Self {
        Self {
            config,
            descriptors: BTreeMap::new(),
        }
    }
    pub fn pin(&mut self, descriptor: ResidencyDescriptor) {
        self.descriptors.insert(descriptor.asset_key, descriptor);
    }
    pub fn unpin(&mut self, asset_key: u64) {
        self.descriptors.remove(&asset_key);
    }
    pub fn metrics(&self, memory: &MemoryControlService) -> ResidencyMetrics {
        let resident_items = self
            .descriptors
            .values()
            .filter(|d| d.residency_set == ResidencySet::Hot)
            .count();
        let streaming_items = self
            .descriptors
            .values()
            .filter(|d| d.residency_set == ResidencySet::StreamingResident)
            .count();
        let pressure = if resident_items > self.config.resident_item_budget
            || streaming_items > self.config.streaming_item_budget
        {
            PressureClass::Critical
        } else {
            memory.metrics().pressure
        };
        ResidencyMetrics {
            resident_items,
            streaming_items,
            pressure,
        }
    }
}

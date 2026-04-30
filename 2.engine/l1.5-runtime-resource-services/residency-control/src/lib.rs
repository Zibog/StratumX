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

    pub fn transition(
        &mut self,
        asset_key: u64,
        next_set: ResidencySet,
    ) -> engine_core::EngineCoreResult<()> {
        match self.descriptors.get_mut(&asset_key) {
            Some(current) => {
                if current.residency_set == next_set {
                    return Ok(());
                }
                let legal = matches!(
                    (current.residency_set, next_set),
                    (ResidencySet::StagingBacked, ResidencySet::StreamingResident)
                        | (ResidencySet::StreamingResident, ResidencySet::Hot)
                        | (ResidencySet::Hot, ResidencySet::StreamingResident)
                        | (ResidencySet::StreamingResident, ResidencySet::StagingBacked)
                );
                if !legal {
                    return Err(engine_core::EngineCoreError::InvalidDescriptor(
                        "residency transition violates canonical promotion ladder",
                    ));
                }
                current.residency_set = next_set;
                Ok(())
            }
            None if next_set == ResidencySet::StagingBacked => {
                self.descriptors.insert(
                    asset_key,
                    ResidencyDescriptor {
                        asset_key,
                        residency_set: ResidencySet::StagingBacked,
                    },
                );
                Ok(())
            }
            None => Err(engine_core::EngineCoreError::InvalidDescriptor(
                "residency transition requires staged descriptor",
            )),
        }
    }

    pub fn descriptor(&self, asset_key: u64) -> Option<&ResidencyDescriptor> {
        self.descriptors.get(&asset_key)
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

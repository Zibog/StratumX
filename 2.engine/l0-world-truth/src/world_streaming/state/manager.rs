use super::metadata::{EvictionPolicy, RegionMetadata};
use crate::world_streaming::mapping::RegionKey;
use crate::world_streaming::residency::{MemoryPressure, ResidencyState};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamingManager {
    pub regions: HashMap<RegionKey, RegionMetadata>,
    pub memory_budget_bytes: usize,
    pub current_memory_bytes: usize,
    pub eviction_policy: EvictionPolicy,
}

impl StreamingManager {
    pub fn new(memory_budget_mb: usize) -> Self {
        Self {
            regions: HashMap::new(),
            memory_budget_bytes: memory_budget_mb * 1024 * 1024,
            current_memory_bytes: 0,
            eviction_policy: EvictionPolicy::LeastRecentlyUsed,
        }
    }

    pub fn memory_pressure(&self) -> MemoryPressure {
        let usage_ratio = self.current_memory_bytes as f32 / self.memory_budget_bytes as f32;
        if usage_ratio > 0.9 {
            MemoryPressure::Critical
        } else if usage_ratio > 0.7 {
            MemoryPressure::Elevated
        } else {
            MemoryPressure::Healthy
        }
    }

    pub fn resident_count(&self) -> usize {
        self.regions
            .values()
            .filter(|m| m.residency_state == ResidencyState::Resident)
            .count()
    }

    pub fn resident_regions(&self) -> Vec<RegionKey> {
        self.regions
            .iter()
            .filter(|(_, m)| m.residency_state == ResidencyState::Resident)
            .map(|(k, _)| *k)
            .collect()
    }

    pub fn request_load(&mut self, key: RegionKey, current_time: f32) -> Result<(), String> {
        if let Some(metadata) = self.regions.get_mut(&key) {
            if metadata.residency_state == ResidencyState::Resident {
                metadata.last_access_time = current_time;
                return Ok(());
            }
        }

        let estimated_size = 10 * 1024 * 1024;
        if self.current_memory_bytes + estimated_size > self.memory_budget_bytes {
            self.evict_one_region()?;
        }

        self.regions.insert(
            key,
            RegionMetadata {
                key,
                residency_state: ResidencyState::Loading,
                memory_bytes: estimated_size,
                last_access_time: current_time,
                priority: 5,
            },
        );

        Ok(())
    }

    pub fn complete_load(&mut self, key: RegionKey, actual_size: usize) -> Result<(), String> {
        if let Some(metadata) = self.regions.get_mut(&key) {
            metadata.residency_state = ResidencyState::Resident;
            metadata.memory_bytes = actual_size;
            self.current_memory_bytes += actual_size;
            Ok(())
        } else {
            Err("Region not found in loading state".to_string())
        }
    }

    pub fn request_unload(&mut self, key: RegionKey) -> Result<(), String> {
        if let Some(metadata) = self.regions.get_mut(&key) {
            if metadata.residency_state == ResidencyState::Resident {
                metadata.residency_state = ResidencyState::Evicting;
                Ok(())
            } else {
                Err("Region not resident".to_string())
            }
        } else {
            Err("Region not found".to_string())
        }
    }

    pub fn complete_unload(&mut self, key: RegionKey) -> Result<(), String> {
        if let Some(metadata) = self.regions.remove(&key) {
            self.current_memory_bytes = self
                .current_memory_bytes
                .saturating_sub(metadata.memory_bytes);
            Ok(())
        } else {
            Err("Region not found".to_string())
        }
    }

    pub(crate) fn evict_one_region(&mut self) -> Result<(), String> {
        use crate::world_streaming::eviction::select_eviction_candidate;

        let candidate = select_eviction_candidate(&self.regions, self.eviction_policy);

        if let Some(key) = candidate {
            self.request_unload(key)?;
            self.complete_unload(key)?;
            Ok(())
        } else {
            Err("No region available for eviction".to_string())
        }
    }
}

use super::state::{TextureResidencyInfo, TextureResidencyState};
use super::types::TextureDescriptor;
use engine_memory_control::PressureClass;
use engine_residency_control::{ResidencyControlService, ResidencyDescriptor, ResidencySet};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextureResidencyMetrics {
    pub total_textures: usize,
    pub resident_textures: usize,
    pub streaming_textures: usize,
    pub resident_bytes: usize,
    pub budget_bytes: usize,
    pub pressure: PressureClass,
    pub evictions_this_frame: usize,
}

#[derive(Debug)]
pub struct TextureResidencyRuntime {
    pub textures: BTreeMap<u64, TextureResidencyInfo>,
    pub budget_bytes: usize,
    pub current_resident_bytes: usize,
    pub current_frame: u64,
    pub evictions_this_frame: usize,
}

impl TextureResidencyRuntime {
    pub fn new(budget_bytes: usize) -> Self {
        Self {
            textures: BTreeMap::new(),
            budget_bytes,
            current_resident_bytes: 0,
            current_frame: 0,
            evictions_this_frame: 0,
        }
    }

    pub fn register_texture(&mut self, descriptor: TextureDescriptor, priority: u32) {
        let texture_id = descriptor.texture_id;
        let info = TextureResidencyInfo {
            descriptor,
            state: TextureResidencyState::NotLoaded,
            last_access_frame: self.current_frame,
            priority,
        };
        self.textures.insert(texture_id, info);
    }

    pub fn request_texture(&mut self, texture_id: u64) -> bool {
        let needs_eviction = if let Some(info) = self.textures.get(&texture_id) {
            match info.state {
                TextureResidencyState::NotLoaded => {
                    self.current_resident_bytes + info.descriptor.size_bytes > self.budget_bytes
                }
                TextureResidencyState::Streaming => {
                    return true;
                }
                TextureResidencyState::Resident => {
                    return true;
                }
                TextureResidencyState::Evicted => false,
            }
        } else {
            return false;
        };

        if needs_eviction {
            self.evict_lru();
        }

        if let Some(info) = self.textures.get_mut(&texture_id) {
            info.last_access_frame = self.current_frame;

            if info.state == TextureResidencyState::NotLoaded
                || info.state == TextureResidencyState::Evicted
            {
                if self.current_resident_bytes + info.descriptor.size_bytes <= self.budget_bytes {
                    info.state = TextureResidencyState::Streaming;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn mark_resident(&mut self, texture_id: u64) {
        if let Some(info) = self.textures.get_mut(&texture_id) {
            if info.state == TextureResidencyState::Streaming {
                info.state = TextureResidencyState::Resident;
                self.current_resident_bytes += info.descriptor.size_bytes;
            }
        }
    }

    pub fn evict_lru(&mut self) {
        let mut lru_id: Option<u64> = None;
        let mut lru_frame = u64::MAX;

        for (id, info) in &self.textures {
            if info.state == TextureResidencyState::Resident && info.last_access_frame < lru_frame {
                lru_frame = info.last_access_frame;
                lru_id = Some(*id);
            }
        }

        if let Some(id) = lru_id {
            self.evict_texture(id);
        }
    }

    pub fn evict_texture(&mut self, texture_id: u64) {
        if let Some(info) = self.textures.get_mut(&texture_id) {
            if info.state == TextureResidencyState::Resident {
                self.current_resident_bytes -= info.descriptor.size_bytes;
                info.state = TextureResidencyState::Evicted;
                self.evictions_this_frame += 1;
            }
        }
    }

    pub fn advance_frame(&mut self) {
        self.current_frame += 1;
        self.evictions_this_frame = 0;
    }

    pub fn metrics(&self) -> TextureResidencyMetrics {
        let total_textures = self.textures.len();
        let resident_textures = self
            .textures
            .values()
            .filter(|t| t.state == TextureResidencyState::Resident)
            .count();
        let streaming_textures = self
            .textures
            .values()
            .filter(|t| t.state == TextureResidencyState::Streaming)
            .count();

        let pressure = if self.current_resident_bytes > self.budget_bytes {
            PressureClass::Critical
        } else if self.current_resident_bytes > (self.budget_bytes * 8 / 10) {
            PressureClass::Elevated
        } else {
            PressureClass::Healthy
        };

        TextureResidencyMetrics {
            total_textures,
            resident_textures,
            streaming_textures,
            resident_bytes: self.current_resident_bytes,
            budget_bytes: self.budget_bytes,
            pressure,
            evictions_this_frame: self.evictions_this_frame,
        }
    }

    pub fn integrate_with_residency_control(&self, residency: &mut ResidencyControlService) {
        for (id, info) in &self.textures {
            let residency_set = match info.state {
                TextureResidencyState::Resident => ResidencySet::Hot,
                TextureResidencyState::Streaming => ResidencySet::StreamingResident,
                _ => ResidencySet::StagingBacked,
            };

            residency.pin(ResidencyDescriptor {
                asset_key: *id,
                residency_set,
            });
        }
    }
}

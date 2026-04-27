use engine_core::{EngineCoreError, EngineCoreResult};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamControlConfig {
    pub max_inflight_requests: usize,
    pub prefetch_radius_regions: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamReason {
    Visibility,
    Prefetch,
    Recovery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamRequest {
    pub region_key: (i32, i32, i32),
    pub priority: u8,
    pub reason: StreamReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamResult {
    pub accepted: bool,
    pub activated_regions: Vec<(i32, i32, i32)>,
    pub deferred: usize,
}

#[derive(Debug)]
pub struct StreamControlService {
    config: StreamControlConfig,
    pending: VecDeque<StreamRequest>,
    active_regions: BTreeSet<(i32, i32, i32)>,
}

impl StreamControlService {
    pub fn new(config: StreamControlConfig) -> Self {
        Self {
            config,
            pending: VecDeque::new(),
            active_regions: BTreeSet::new(),
        }
    }
    pub fn queue_request(&mut self, request: StreamRequest) -> EngineCoreResult<()> {
        if self.pending.len() >= self.config.max_inflight_requests {
            return Err(EngineCoreError::InvalidDescriptor(
                "stream request queue exceeds canonical ceiling",
            ));
        }
        self.pending.push_back(request);
        Ok(())
    }
    pub fn tick(&mut self) -> StreamResult {
        let mut activated = Vec::new();
        while let Some(request) = self.pending.pop_front() {
            self.active_regions.insert(request.region_key);
            activated.push(request.region_key);
        }
        StreamResult {
            accepted: !activated.is_empty(),
            activated_regions: activated,
            deferred: self.pending.len(),
        }
    }
    pub fn active_regions(&self) -> &BTreeSet<(i32, i32, i32)> {
        &self.active_regions
    }
}

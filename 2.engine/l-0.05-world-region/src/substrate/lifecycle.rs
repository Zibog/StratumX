use super::RegionSubstrate;
use crate::validation::validate_state_transition;
use crate::{RegionDescriptor, RegionPriority, RegionState, RegionVersion};
use engine_core::Tick;
use engine_world_spatial::RegionAddress;

impl RegionSubstrate {
    /// Register a new region with default lifecycle state (Inactive).
    pub fn register_region(&mut self, address: RegionAddress, tick: Tick) {
        self.regions
            .entry(address)
            .or_insert_with(|| RegionDescriptor {
                address,
                version: RegionVersion {
                    epoch: 0,
                    last_dirty_tick: tick,
                },
                state: RegionState::Inactive,
                priority: RegionPriority::LOWEST,
                bounds: [0, 0, 0, 0],
            });
    }

    /// Register a region with explicit bounds.
    pub fn register_region_with_bounds(
        &mut self,
        address: RegionAddress,
        tick: Tick,
        bounds: [i32; 4],
    ) {
        self.regions
            .entry(address)
            .or_insert_with(|| RegionDescriptor {
                address,
                version: RegionVersion {
                    epoch: 0,
                    last_dirty_tick: tick,
                },
                state: RegionState::Inactive,
                priority: RegionPriority::LOWEST,
                bounds,
            });
    }

    /// Activate a region for simulation.
    /// **Canon rule**: only active regions participate in tick processing.
    pub fn activate_region(&mut self, region: RegionAddress, priority: RegionPriority) -> bool {
        self.transition_region(region, RegionState::Active, Some(priority))
    }

    /// Deactivate a region — it stops receiving tick updates.
    pub fn deactivate_region(&mut self, region: RegionAddress) -> bool {
        self.transition_region(region, RegionState::Inactive, None)
    }

    /// Set region to streaming-in state — data is being loaded.
    pub fn start_streaming_in(&mut self, region: RegionAddress) -> bool {
        self.transition_region(region, RegionState::StreamingIn, None)
    }

    /// Complete streaming — transition to Active if previously streaming.
    pub fn complete_stream_in(&mut self, region: RegionAddress) -> bool {
        if self.region_state(region) != Some(RegionState::StreamingIn) {
            return false;
        }
        self.transition_region(region, RegionState::Active, None)
    }

    /// Start streaming out — region scheduled for unload.
    pub fn start_streaming_out(&mut self, region: RegionAddress) -> bool {
        self.transition_region(region, RegionState::StreamingOut, None)
    }

    /// Freeze a region — preserved but not simulated.
    pub fn freeze_region(&mut self, region: RegionAddress) -> bool {
        self.transition_region(region, RegionState::Frozen, None)
    }

    /// Advance substrate tick — processes all active regions.
    /// Returns list of regions that had dirty updates.
    pub fn advance_tick(&mut self, _tick: Tick) -> Vec<RegionAddress> {
        let mut dirty_regions = Vec::new();
        for (addr, region) in &self.regions {
            if region.state == RegionState::Active {
                if let Some(snapshot) = self.dirty_snapshot(*addr) {
                    if !snapshot.chunks.is_empty() {
                        dirty_regions.push(*addr);
                    }
                }
            }
        }
        dirty_regions
    }

    fn transition_region(
        &mut self,
        region: RegionAddress,
        target: RegionState,
        priority: Option<RegionPriority>,
    ) -> bool {
        let Some(region_descriptor) = self.regions.get_mut(&region) else {
            return false;
        };
        if validate_state_transition(region_descriptor.state, target).is_err() {
            return false;
        }
        region_descriptor.state = target;
        if let Some(priority) = priority {
            region_descriptor.priority = priority;
        }
        true
    }
}

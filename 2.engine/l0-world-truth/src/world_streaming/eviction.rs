// Eviction policy - select candidate for eviction

use super::mapping::RegionKey;
use super::residency::ResidencyState;
use super::state::{EvictionPolicy, RegionMetadata};
use std::collections::HashMap;

pub fn select_eviction_candidate(
    regions: &HashMap<RegionKey, RegionMetadata>,
    policy: EvictionPolicy,
) -> Option<RegionKey> {
    match policy {
        EvictionPolicy::LeastRecentlyUsed => regions
            .iter()
            .filter(|(_, m)| m.residency_state == ResidencyState::Resident)
            .min_by_key(|(_, m)| (m.last_access_time * 1000.0) as i64)
            .map(|(k, _)| *k),
        EvictionPolicy::LeastPriority => regions
            .iter()
            .filter(|(_, m)| m.residency_state == ResidencyState::Resident)
            .min_by_key(|(_, m)| m.priority)
            .map(|(k, _)| *k),
        EvictionPolicy::FarthestFromPlayer => regions
            .iter()
            .filter(|(_, m)| m.residency_state == ResidencyState::Resident)
            .max_by_key(|(k, _)| k.0.abs() + k.1.abs() + k.2.abs())
            .map(|(k, _)| *k),
    }
}

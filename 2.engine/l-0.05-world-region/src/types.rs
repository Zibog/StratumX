//! Type definitions for regions and chunks.

use bitflags::bitflags;
use engine_core::Tick;
use engine_world_spatial::{ChunkAddress, RegionAddress};
use serde::{Deserialize, Serialize};

pub const CHUNK_EDGE_METERS: u32 = 32;
pub const VERTICAL_SLAB_METERS: u32 = 16;
pub const SAME_TICK_HALO_WIDTH: u8 = 1;
pub const MAX_FIELD_SOLVE_HALO_WIDTH: u8 = 2;

/// Canonical region lifecycle states.
/// **Owner**: engine_world_region — no other layer may define region states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RegionState {
    /// Region is registered but not yet active for simulation.
    #[default]
    Inactive,
    /// Region is active and participates in tick processing.
    Active,
    /// Region is streaming in — partial data available.
    StreamingIn,
    /// Region is streaming out — scheduled for unload.
    StreamingOut,
    /// Region is frozen — no simulation updates, data preserved.
    Frozen,
}

// Explicit Default removed: now derived via Default derive macro

/// Canonical region priority for tick ordering and field solve scheduling.
/// **Owner**: engine_world_region — defines priority semantics for all consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RegionPriority(pub u8);

impl RegionPriority {
    pub const HIGHEST: Self = Self(0);
    pub const LOWEST: Self = Self(u8::MAX);

    pub fn is_high_priority(self) -> bool {
        self.0 < 64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionVersion {
    pub epoch: u64,
    pub last_dirty_tick: Tick,
}

/// Extended region metadata with lifecycle state and priority.
/// **Canon anchor**: region_lifecycle, region_priority, region_bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionDescriptor {
    pub address: RegionAddress,
    pub version: RegionVersion,
    pub state: RegionState,
    pub priority: RegionPriority,
    /// World-space bounds [min_x, min_z, max_x, max_z] in meters.
    pub bounds: [i32; 4],
}

impl RegionDescriptor {
    /// Check if this region is currently active for simulation.
    pub fn is_simulation_active(&self) -> bool {
        matches!(self.state, RegionState::Active)
    }

    /// Check if this region has any data available (including streaming).
    pub fn has_data(&self) -> bool {
        !matches!(self.state, RegionState::Inactive)
    }

    /// Calculate center point of region bounds.
    pub fn center(&self) -> [i32; 2] {
        [
            (self.bounds[0] + self.bounds[2]) / 2,
            (self.bounds[1] + self.bounds[3]) / 2,
        ]
    }

    /// Calculate region size from bounds.
    pub fn size(&self) -> [u32; 2] {
        [
            (self.bounds[2] - self.bounds[0]) as u32,
            (self.bounds[3] - self.bounds[1]) as u32,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkDescriptor {
    pub address: ChunkAddress,
    pub region: RegionAddress,
}

impl ChunkDescriptor {
    /// Check if this chunk belongs to the given region.
    pub fn belongs_to(&self, region: RegionAddress) -> bool {
        self.region == region
    }

    /// Get chunk grid position within its region.
    pub fn grid_position(&self) -> [i32; 2] {
        [self.address.chunk_x, self.address.chunk_y]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirtyChunkEntry {
    pub chunk: ChunkAddress,
    pub flags: DirtyFlags,
    pub dirty_tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirtyRegionSnapshot {
    pub region: RegionAddress,
    pub version: RegionVersion,
    pub chunks: Vec<DirtyChunkEntry>,
    pub total_dirty_count: u32,
}

impl DirtyRegionSnapshot {
    /// Check if any chunk has a specific dirty flag.
    pub fn has_flag(&self, flag: DirtyFlags) -> bool {
        self.chunks.iter().any(|c| c.flags.contains(flag))
    }

    /// Count chunks with a specific flag.
    pub fn count_with_flag(&self, flag: DirtyFlags) -> u32 {
        self.chunks
            .iter()
            .filter(|c| c.flags.contains(flag))
            .count() as u32
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DirtyFlags: u8 {
        const GEOMETRY = 0b0001;
        const ECS = 0b0010;
        const MATERIAL = 0b0100;
        const HYDROLOGY = 0b1000;
    }
}

use engine_core::Tick;
use engine_world_region::{DirtyFlags, RegionPriority, RegionState, RegionSubstrate};
use engine_world_spatial::{ChunkAddress, RegionAddress};

fn make_region(id: i32) -> RegionAddress {
    RegionAddress {
        x: id,
        y: 0,
        slab_z: 0,
    }
}

fn make_chunk(region: RegionAddress, x: i32, y: i32) -> ChunkAddress {
    ChunkAddress {
        region,
        chunk_x: x,
        chunk_y: y,
    }
}

// === Region Lifecycle Tests ===

include!("world_region_surface/cases_01.rs");
include!("world_region_surface/cases_02.rs");
include!("world_region_surface/cases_03.rs");

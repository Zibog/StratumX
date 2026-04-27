use engine_core::Tick;
use engine_world_region::{
    DirtyFlags, RegionSubstrate, CHUNK_EDGE_METERS, MAX_FIELD_SOLVE_HALO_WIDTH,
};
use engine_world_spatial::{ChunkAddress, RegionAddress};

#[test]
fn region_constants_match_canon() {
    assert_eq!(CHUNK_EDGE_METERS, 32);
    assert_eq!(MAX_FIELD_SOLVE_HALO_WIDTH, 2);
}

#[test]
fn dirty_tracking_updates_region_version() {
    let mut regions = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 0,
        chunk_y: 0,
    };

    regions.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    regions.mark_dirty(chunk, DirtyFlags::GEOMETRY, Tick(2));

    let dirty = regions.dirty_chunks(region);
    assert_eq!(dirty.len(), 1);
    let version = regions.region_version(region).unwrap();
    assert_eq!(version.epoch, 2);
    assert_eq!(version.last_dirty_tick, Tick(2));
}

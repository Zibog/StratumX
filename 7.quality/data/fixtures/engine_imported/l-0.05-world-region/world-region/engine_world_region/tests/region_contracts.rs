use engine_core::Tick;
use engine_world_region::{
    DirtyFlags, RegionSubstrate, SAME_TICK_HALO_WIDTH, VERTICAL_SLAB_METERS,
};
use engine_world_spatial::{ChunkAddress, RegionAddress};

#[test]
fn register_region_is_idempotent_before_dirtied() {
    let mut substrate = RegionSubstrate::default();
    let region = RegionAddress {
        x: 1,
        y: 2,
        slab_z: 3,
    };
    substrate.register_region(region, Tick(5));
    substrate.register_region(region, Tick(9));
    let version = substrate.region_version(region).unwrap();
    assert_eq!(version.epoch, 0);
    assert_eq!(version.last_dirty_tick, Tick(5));
}

#[test]
fn chunk_descriptor_reflects_region_identity() {
    let substrate = RegionSubstrate::default();
    let address = ChunkAddress {
        region: RegionAddress {
            x: 9,
            y: 8,
            slab_z: 7,
        },
        chunk_x: 6,
        chunk_y: 5,
    };
    let descriptor = substrate.chunk_descriptor(address);
    assert_eq!(descriptor.region, address.region);
    assert_eq!(descriptor.address, address);
    assert_eq!(SAME_TICK_HALO_WIDTH, 1);
    assert_eq!(VERTICAL_SLAB_METERS, 16);
    assert!(DirtyFlags::GEOMETRY.bits() != 0);
}

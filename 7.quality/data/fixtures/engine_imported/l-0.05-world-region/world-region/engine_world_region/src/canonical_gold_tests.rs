#![allow(unused_imports)]
use super::*;
use engine_world_spatial::{ChunkAddress, RegionAddress};

#[test]
fn register_region_sets_initial_epoch() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    r.register_region(region, Tick(0));
    assert_eq!(r.region_version(region).unwrap().epoch, 0);
}
#[test]
fn chunk_descriptor_refers_to_region() {
    let r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 1,
        chunk_y: 2,
    };
    assert_eq!(r.chunk_descriptor(chunk).region, region);
}
#[test]
fn mark_dirty_increments_epoch() {
    let mut r = RegionSubstrate::default();
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
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert_eq!(r.region_version(region).unwrap().epoch, 1);
}
#[test]
fn dirty_chunks_returns_marked_chunk() {
    let mut r = RegionSubstrate::default();
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
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn dirty_flags_union_across_marks() {
    let mut r = RegionSubstrate::default();
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
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    r.mark_dirty(chunk, DirtyFlags::MATERIAL, Tick(2));
    assert_eq!(r.region_version(region).unwrap().last_dirty_tick, Tick(2));
}

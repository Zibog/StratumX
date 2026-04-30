#[test]
fn world_region_dirty_marks_chunk_22() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 22,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_23() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 23,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_24() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 24,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_25() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 25,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_26() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 26,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_27() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 27,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_28() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 28,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_29() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 29,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}

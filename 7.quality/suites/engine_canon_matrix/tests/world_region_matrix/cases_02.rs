#[test]
fn world_region_dirty_marks_chunk_11() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 11,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_12() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 12,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_13() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 13,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_14() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 14,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_15() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 15,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_16() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 16,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_17() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 17,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_18() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 18,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_19() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 19,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_20() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 20,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_21() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 21,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}

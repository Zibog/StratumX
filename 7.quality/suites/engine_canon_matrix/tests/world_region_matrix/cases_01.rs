#[test]
fn world_region_dirty_marks_chunk_0() {
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
fn world_region_dirty_marks_chunk_1() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 1,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_2() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 2,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_3() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 3,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_4() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 4,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_5() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 5,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_6() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 6,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_7() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 7,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_8() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 8,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_9() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 9,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}
#[test]
fn world_region_dirty_marks_chunk_10() {
    let mut r = RegionSubstrate::default();
    let region = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let chunk = ChunkAddress {
        region,
        chunk_x: 10,
        chunk_y: 0,
    };
    r.mark_dirty(chunk, DirtyFlags::ECS, Tick(1));
    assert!(r.dirty_chunks(region).contains(&chunk));
}

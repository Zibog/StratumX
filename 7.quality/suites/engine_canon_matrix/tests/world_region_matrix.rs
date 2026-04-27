mod common;
use common::*;

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

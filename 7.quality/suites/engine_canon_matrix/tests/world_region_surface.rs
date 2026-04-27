use engine_core::Tick;
use engine_world_region::{
    DirtyFlags, RegionPriority, RegionState, RegionSubstrate,
};
use engine_world_spatial::{ChunkAddress, RegionAddress};

fn make_region(id: i32) -> RegionAddress {
    RegionAddress { x: id, y: 0, slab_z: 0 }
}

fn make_chunk(region: RegionAddress, x: i32, y: i32) -> ChunkAddress {
    ChunkAddress {
        region,
        chunk_x: x,
        chunk_y: y,
    }
}

// === Region Lifecycle Tests ===

#[test]
fn test_register_region_creates_inactive() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    assert_eq!(substrate.region_state(region), Some(RegionState::Inactive));
}

#[test]
fn test_activate_region() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    assert!(substrate.activate_region(region, RegionPriority::HIGHEST));
    assert_eq!(substrate.region_state(region), Some(RegionState::Active));
}

#[test]
fn test_activate_nonexistent_region_fails() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(999);

    assert!(!substrate.activate_region(region, RegionPriority::HIGHEST));
}

#[test]
fn test_deactivate_region() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));
    substrate.activate_region(region, RegionPriority(10));

    assert!(substrate.deactivate_region(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::Inactive));
}

#[test]
fn test_streaming_lifecycle() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    assert!(substrate.start_streaming_in(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::StreamingIn));

    assert!(substrate.complete_stream_in(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::Active));

    assert!(substrate.start_streaming_out(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::StreamingOut));
}

#[test]
fn test_complete_stream_in_only_works_from_streaming() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));
    // Region is Inactive, not StreamingIn

    assert!(!substrate.complete_stream_in(region));
}

#[test]
fn test_freeze_region() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));
    substrate.activate_region(region, RegionPriority(10));

    assert!(substrate.freeze_region(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::Frozen));
}

// === Dirty Tracking Tests ===

#[test]
fn test_mark_dirty_tracks_chunk_and_flags() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let chunk = make_chunk(region, 0, 0);

    substrate.mark_dirty(chunk, DirtyFlags::GEOMETRY, Tick(5));

    assert!(substrate.dirty_flags(chunk).is_some());
    assert_eq!(substrate.dirty_flags(chunk), Some(DirtyFlags::GEOMETRY));
    assert_eq!(substrate.dirty_tick(chunk), Some(Tick(5)));
}

#[test]
fn test_dirty_chunks_returns_correct_set() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let chunk1 = make_chunk(region, 0, 0);
    let chunk2 = make_chunk(region, 1, 0);

    substrate.mark_dirty(chunk1, DirtyFlags::GEOMETRY, Tick(1));
    substrate.mark_dirty(chunk2, DirtyFlags::ECS, Tick(2));

    let dirty = substrate.dirty_chunks(region);
    assert_eq!(dirty.len(), 2);
    assert!(dirty.contains(&chunk1));
    assert!(dirty.contains(&chunk2));
}

#[test]
fn test_dirty_chunks_empty_for_clean_region() {
    let substrate = RegionSubstrate::default();
    let region = make_region(1);

    let dirty = substrate.dirty_chunks(region);
    assert!(dirty.is_empty());
}

#[test]
fn test_clear_chunk_dirty() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let chunk = make_chunk(region, 0, 0);

    substrate.mark_dirty(chunk, DirtyFlags::GEOMETRY, Tick(1));
    assert_eq!(substrate.dirty_chunks(region).len(), 1);

    substrate.clear_chunk_dirty(chunk);
    assert_eq!(substrate.dirty_chunks(region).len(), 0);
}

#[test]
fn test_clear_region_dirty() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let chunk1 = make_chunk(region, 0, 0);
    let chunk2 = make_chunk(region, 1, 0);

    substrate.mark_dirty(chunk1, DirtyFlags::GEOMETRY, Tick(1));
    substrate.mark_dirty(chunk2, DirtyFlags::ECS, Tick(2));

    substrate.clear_region_dirty(region);
    assert!(substrate.dirty_chunks(region).is_empty());
}

// === Snapshot Tests ===

#[test]
fn test_dirty_snapshot_contains_all_dirty_chunks() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let chunk1 = make_chunk(region, 0, 0);
    let chunk2 = make_chunk(region, 1, 0);

    substrate.mark_dirty(chunk1, DirtyFlags::GEOMETRY, Tick(1));
    substrate.mark_dirty(chunk2, DirtyFlags::ECS | DirtyFlags::MATERIAL, Tick(2));

    let snapshot = substrate.dirty_snapshot(region).unwrap();
    assert_eq!(snapshot.total_dirty_count, 2);
    assert_eq!(snapshot.chunks.len(), 2);
}

#[test]
fn test_dirty_snapshot_no_snapshot_for_unknown_region() {
    let substrate = RegionSubstrate::default();
    let region = make_region(999);

    assert!(substrate.dirty_snapshot(region).is_none());
}

#[test]
fn test_snapshot_has_flag() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let chunk = make_chunk(region, 0, 0);

    substrate.mark_dirty(chunk, DirtyFlags::GEOMETRY | DirtyFlags::ECS, Tick(1));

    let snapshot = substrate.dirty_snapshot(region).unwrap();
    assert!(snapshot.has_flag(DirtyFlags::GEOMETRY));
    assert!(snapshot.has_flag(DirtyFlags::ECS));
    assert!(!snapshot.has_flag(DirtyFlags::MATERIAL));
}

#[test]
fn test_snapshot_count_with_flag() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);

    substrate.mark_dirty(
        make_chunk(region, 0, 0),
        DirtyFlags::GEOMETRY,
        Tick(1),
    );
    substrate.mark_dirty(
        make_chunk(region, 1, 0),
        DirtyFlags::GEOMETRY | DirtyFlags::ECS,
        Tick(2),
    );
    substrate.mark_dirty(
        make_chunk(region, 2, 0),
        DirtyFlags::MATERIAL,
        Tick(3),
    );

    let snapshot = substrate.dirty_snapshot(region).unwrap();
    assert_eq!(snapshot.count_with_flag(DirtyFlags::GEOMETRY), 2);
    assert_eq!(snapshot.count_with_flag(DirtyFlags::MATERIAL), 1);
}

// === Tick Advance Tests ===

#[test]
fn test_advance_tick_returns_dirty_regions() {
    let mut substrate = RegionSubstrate::default();
    let region1 = make_region(1);
    let region2 = make_region(2);

    substrate.register_region(region1, Tick(0));
    substrate.register_region(region2, Tick(0));
    substrate.activate_region(region1, RegionPriority(10));
    substrate.activate_region(region2, RegionPriority(20));

    substrate.mark_dirty(make_chunk(region1, 0, 0), DirtyFlags::GEOMETRY, Tick(1));

    let dirty = substrate.advance_tick(Tick(2));
    assert_eq!(dirty.len(), 1);
    assert!(dirty.contains(&region1));
}

#[test]
fn test_advance_tick_skips_inactive_regions() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);

    substrate.register_region(region, Tick(0));
    // Region is Inactive
    substrate.mark_dirty(make_chunk(region, 0, 0), DirtyFlags::GEOMETRY, Tick(1));

    let dirty = substrate.advance_tick(Tick(2));
    assert!(dirty.is_empty());
}

// === Region Descriptor Tests ===

#[test]
fn test_region_descriptor_bounds() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let bounds = [-512, -512, 512, 512];
    substrate.register_region_with_bounds(region, Tick(0), bounds);

    let desc = substrate.region_descriptor(region).unwrap();
    assert_eq!(desc.bounds, bounds);
}

#[test]
fn test_region_descriptor_center() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let bounds = [0, 0, 100, 200];
    substrate.register_region_with_bounds(region, Tick(0), bounds);

    let desc = substrate.region_descriptor(region).unwrap();
    let center = desc.center();
    assert_eq!(center, [50, 100]);
}

#[test]
fn test_region_descriptor_size() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    let bounds = [-100, -200, 100, 200];
    substrate.register_region_with_bounds(region, Tick(0), bounds);

    let desc = substrate.region_descriptor(region).unwrap();
    let size = desc.size();
    assert_eq!(size, [200, 400]);
}

// === Chunk Descriptor Tests ===

#[test]
fn test_chunk_descriptor_belongs_to() {
    let substrate = RegionSubstrate::default();
    let region1 = make_region(1);
    let region2 = make_region(2);
    let chunk = make_chunk(region1, 5, 3);

    let desc = substrate.chunk_descriptor(chunk);
    assert!(desc.belongs_to(region1));
    assert!(!desc.belongs_to(region2));
}

#[test]
fn test_chunk_descriptor_grid_position() {
    let substrate = RegionSubstrate::default();
    let region = make_region(1);
    let chunk = make_chunk(region, 7, 11);

    let desc = substrate.chunk_descriptor(chunk);
    assert_eq!(desc.grid_position(), [7, 11]);
}

// === Version Tests ===

#[test]
fn test_version_increases_on_dirty() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    let v1 = substrate.region_version(region).unwrap();
    substrate.mark_dirty(
        make_chunk(region, 0, 0),
        DirtyFlags::GEOMETRY,
        Tick(1),
    );
    let v2 = substrate.region_version(region).unwrap();

    assert_eq!(v2.epoch, v1.epoch + 1);
    assert_eq!(v2.last_dirty_tick, Tick(1));
}

// === RegionPriority Tests ===

#[test]
fn test_priority_ordering() {
    let p1 = RegionPriority(10);
    let p2 = RegionPriority(20);
    assert!(p1 < p2); // Lower number = higher priority
}

#[test]
fn test_priority_is_high_priority() {
    assert!(RegionPriority(0).is_high_priority());
    assert!(RegionPriority(63).is_high_priority());
    assert!(!RegionPriority(64).is_high_priority());
    assert!(!RegionPriority(255).is_high_priority());
}

// === RegionState Tests ===

#[test]
fn test_region_state_default_is_inactive() {
    assert_eq!(RegionState::default(), RegionState::Inactive);
}

#[test]
fn test_region_state_is_simulation_active() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    let desc = substrate.region_descriptor(region).unwrap();
    assert!(!desc.is_simulation_active());

    substrate.activate_region(region, RegionPriority(10));
    let desc = substrate.region_descriptor(region).unwrap();
    assert!(desc.is_simulation_active());
}

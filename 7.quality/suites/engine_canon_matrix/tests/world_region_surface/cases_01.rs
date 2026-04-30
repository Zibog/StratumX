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

    assert!(!substrate.deactivate_region(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::Active));
    assert!(substrate.start_streaming_out(region));
    assert!(substrate.deactivate_region(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::Inactive));
}

#[test]
fn test_streaming_lifecycle() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    assert!(substrate.start_streaming_in(region));
    assert_eq!(
        substrate.region_state(region),
        Some(RegionState::StreamingIn)
    );

    assert!(substrate.complete_stream_in(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::Active));

    assert!(substrate.start_streaming_out(region));
    assert_eq!(
        substrate.region_state(region),
        Some(RegionState::StreamingOut)
    );
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

#[test]
fn test_invalid_transition_from_inactive_to_stream_out_is_rejected() {
    let mut substrate = RegionSubstrate::default();
    let region = make_region(1);
    substrate.register_region(region, Tick(0));

    assert!(!substrate.start_streaming_out(region));
    assert_eq!(substrate.region_state(region), Some(RegionState::Inactive));
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


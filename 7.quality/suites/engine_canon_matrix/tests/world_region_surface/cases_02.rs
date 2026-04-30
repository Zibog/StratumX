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

    substrate.mark_dirty(make_chunk(region, 0, 0), DirtyFlags::GEOMETRY, Tick(1));
    substrate.mark_dirty(
        make_chunk(region, 1, 0),
        DirtyFlags::GEOMETRY | DirtyFlags::ECS,
        Tick(2),
    );
    substrate.mark_dirty(make_chunk(region, 2, 0), DirtyFlags::MATERIAL, Tick(3));

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
    substrate.mark_dirty(make_chunk(region, 0, 0), DirtyFlags::GEOMETRY, Tick(1));
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


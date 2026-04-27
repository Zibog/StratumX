use engine_core::Tick;
use engine_world_region::{DirtyFlags, DirtyRegionSnapshot, RegionSubstrate};
use engine_world_spatial::chunk_address;

#[test]
fn mark_dirty_merges_flags_and_bumps_region_version() {
    let chunk = chunk_address(2, 3, 0);
    let mut substrate = RegionSubstrate::default();

    substrate.mark_dirty(chunk, DirtyFlags::GEOMETRY, Tick(10));
    substrate.mark_dirty(chunk, DirtyFlags::MATERIAL, Tick(12));

    assert_eq!(
        substrate.dirty_flags(chunk),
        Some(DirtyFlags::GEOMETRY | DirtyFlags::MATERIAL)
    );

    let version = substrate.region_version(chunk.region).unwrap();
    assert_eq!(version.epoch, 2);
    assert_eq!(version.last_dirty_tick, Tick(12));
}

#[test]
fn dirty_snapshot_is_versioned_and_sorted() {
    let mut substrate = RegionSubstrate::default();
    let later = chunk_address(1, 0, 0);
    let earlier = chunk_address(0, 0, 0);

    substrate.mark_dirty(later, DirtyFlags::ECS, Tick(20));
    substrate.mark_dirty(earlier, DirtyFlags::GEOMETRY, Tick(21));

    let snapshot = substrate.dirty_snapshot(earlier.region).unwrap();
    assert_eq!(snapshot.region, earlier.region);
    assert_eq!(snapshot.version.epoch, 2);
    assert_eq!(snapshot.version.last_dirty_tick, Tick(21));
    assert_eq!(snapshot.chunks.len(), 2);
    assert_eq!(snapshot.chunks[0].chunk, earlier);
    assert_eq!(snapshot.chunks[1].chunk, later);
}

#[test]
fn dirty_snapshot_roundtrips_through_json() {
    let chunk = chunk_address(-1, 5, 2);
    let mut substrate = RegionSubstrate::default();
    substrate.mark_dirty(chunk, DirtyFlags::GEOMETRY | DirtyFlags::ECS, Tick(9));

    let snapshot = substrate.dirty_snapshot(chunk.region).unwrap();
    let json = serde_json::to_string(&snapshot).unwrap();
    let restored: DirtyRegionSnapshot = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, snapshot);
}

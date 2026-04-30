#[test]
fn test_chunk_address_creation() {
    let addr = chunk_address(0, 0, 0);
    assert_eq!(addr.chunk_x, 0);
    assert_eq!(addr.chunk_y, 0);
    assert_eq!(addr.region.x, 0);
    assert_eq!(addr.region.y, 0);
    assert_eq!(addr.region.slab_z, 0);
}

#[test]
fn test_chunk_address_crosses_region_boundary() {
    // REGION_EDGE_CHUNKS = 32, so chunk_x=32 should be in region x=1
    let addr = chunk_address(32, 0, 0);
    assert_eq!(addr.chunk_x, 32);
    assert_eq!(addr.region.x, 1);
    assert_eq!(addr.region.y, 0);
}

#[test]
fn test_chunk_address_negative_crosses_boundary() {
    let addr = chunk_address(-1, 0, 0);
    assert_eq!(addr.chunk_x, -1);
    assert_eq!(addr.region.x, -1); // div_euclid(-1, 32) = -1
}

// === World Coordinate Tests ===

#[test]
fn test_address_for_world_coordinate() {
    let coord = WorldCoordinate {
        meters: glam::Vec3::new(100.0, 200.0, 50.0),
    };
    let addr = address_for_world_coordinate(coord);

    // chunk_x = floor(100.0 / 32.0) = 3
    // chunk_y = floor(200.0 / 32.0) = 6
    // slab_z = floor(50.0 / 16.0) = 3
    assert_eq!(addr.chunk_x, 3);
    assert_eq!(addr.chunk_y, 6);
    assert_eq!(addr.region.slab_z, 3);
}

#[test]
fn test_address_for_origin() {
    let coord = WorldCoordinate {
        meters: glam::Vec3::ZERO,
    };
    let addr = address_for_world_coordinate(coord);
    assert_eq!(addr.chunk_x, 0);
    assert_eq!(addr.chunk_y, 0);
    assert_eq!(addr.region.slab_z, 0);
}

// === Origin Tests ===

#[test]
fn test_region_origin_meters() {
    let region = RegionAddress {
        x: 1,
        y: 2,
        slab_z: 0,
    };
    let origin = region_origin_meters(region);

    // 1 * 32 * 32 = 1024
    // 2 * 32 * 32 = 2048
    // 0 * 16 = 0
    assert_eq!(origin.x, 1024.0);
    assert_eq!(origin.y, 2048.0);
    assert_eq!(origin.z, 0.0);
}

#[test]
fn test_chunk_origin_meters() {
    let chunk = ChunkAddress {
        region: RegionAddress {
            x: 0,
            y: 0,
            slab_z: 1,
        },
        chunk_x: 5,
        chunk_y: 10,
    };
    let origin = chunk_origin_meters(chunk);

    // 5 * 32 = 160
    // 10 * 32 = 320
    // 1 * 16 = 16
    assert_eq!(origin.x, 160.0);
    assert_eq!(origin.y, 320.0);
    assert_eq!(origin.z, 16.0);
}

// === Coordinate Transformation Tests ===

#[test]
fn test_to_region_local() {
    let coord = WorldCoordinate {
        meters: glam::Vec3::new(1100.0, 2100.0, 10.0),
    };
    let region = RegionAddress {
        x: 1,
        y: 2,
        slab_z: 0,
    };
    let local = to_region_local(coord, region);

    // 1100 - 1024 = 76
    // 2100 - 2048 = 52
    // 10 - 0 = 10
    assert!((local.x - 76.0).abs() < 0.001);
    assert!((local.y - 52.0).abs() < 0.001);
    assert!((local.z - 10.0).abs() < 0.001);
}

#[test]
fn test_rebase_for_presentation_equals_region_local() {
    let coord = WorldCoordinate {
        meters: glam::Vec3::new(500.0, 600.0, 0.0),
    };
    let anchor = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };

    let rebased = rebase_for_presentation(coord, anchor);
    let local = to_region_local(coord, anchor);

    assert_eq!(rebased, local);
}

// === Halo Tests ===

#[test]
fn test_chunk_halo_radius_zero() {
    let center = chunk_address(5, 5, 0);
    let halo = chunk_halo(center, 0);
    assert_eq!(halo.len(), 1);
    assert_eq!(halo[0], center);
}

#[test]
fn test_chunk_halo_radius_one() {
    let center = chunk_address(5, 5, 0);
    let halo = chunk_halo(center, 1);
    assert_eq!(halo.len(), 9); // 3x3 grid
}

#[test]
fn test_chunk_halo_radius_two() {
    let center = chunk_address(5, 5, 0);
    let halo = chunk_halo(center, 2);
    assert_eq!(halo.len(), 25); // 5x5 grid
}

#[test]
fn test_chunk_halo_preserves_slab_z() {
    let center = chunk_address(0, 0, 7);
    let halo = chunk_halo(center, 1);
    for chunk in &halo {
        assert_eq!(chunk.region.slab_z, 7);
    }
}

// === Spatial Relation Tests ===

#[test]
fn test_classify_relation_same_chunk() {
    let chunk = chunk_address(5, 5, 0);
    assert_eq!(classify_relation(chunk, chunk), SpatialRelation::Contained);
}


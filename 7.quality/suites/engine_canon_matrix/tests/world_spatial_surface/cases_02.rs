#[test]
fn test_classify_relation_adjacent() {
    let a = chunk_address(5, 5, 0);
    let b = chunk_address(6, 5, 0);
    assert_eq!(classify_relation(a, b), SpatialRelation::Adjacent);
}

#[test]
fn test_classify_relation_disjoint() {
    let a = chunk_address(0, 0, 0);
    let b = chunk_address(10, 10, 0);
    assert_eq!(classify_relation(a, b), SpatialRelation::Disjoint);
}

#[test]
fn test_classify_relation_different_slab_is_disjoint() {
    let a = chunk_address(5, 5, 0);
    let b = chunk_address(5, 5, 1);
    assert_eq!(classify_relation(a, b), SpatialRelation::Disjoint);
}

// === Transform Tests ===

#[test]
fn test_compose_transform_identity() {
    let identity = Transform {
        translation: glam::Vec3::ZERO,
        rotation: glam::Quat::IDENTITY,
        scale: glam::Vec3::ONE,
    };
    let local = Transform {
        translation: glam::Vec3::new(1.0, 2.0, 3.0),
        rotation: glam::Quat::IDENTITY,
        scale: glam::Vec3::new(2.0, 2.0, 2.0),
    };

    let composed = compose_transform(identity, local);
    assert_eq!(composed.translation, local.translation);
    assert_eq!(composed.scale, local.scale);
}

#[test]
fn test_compose_transform_translation() {
    let parent = Transform {
        translation: glam::Vec3::new(10.0, 20.0, 30.0),
        rotation: glam::Quat::IDENTITY,
        scale: glam::Vec3::ONE,
    };
    let local = Transform {
        translation: glam::Vec3::new(1.0, 2.0, 3.0),
        rotation: glam::Quat::IDENTITY,
        scale: glam::Vec3::ONE,
    };

    let composed = compose_transform(parent, local);
    assert_eq!(composed.translation, glam::Vec3::new(11.0, 22.0, 33.0));
}

#[test]
fn test_compose_transform_scale() {
    let parent = Transform {
        translation: glam::Vec3::ZERO,
        rotation: glam::Quat::IDENTITY,
        scale: glam::Vec3::new(2.0, 2.0, 2.0),
    };
    let local = Transform {
        translation: glam::Vec3::ZERO,
        rotation: glam::Quat::IDENTITY,
        scale: glam::Vec3::new(3.0, 3.0, 3.0),
    };

    let composed = compose_transform(parent, local);
    assert_eq!(composed.scale, glam::Vec3::new(6.0, 6.0, 6.0));
}

// === Spatial Address Tests ===

#[test]
fn test_spatial_address_for_world_coordinate_is_chunk() {
    let coord = WorldCoordinate {
        meters: glam::Vec3::new(100.0, 200.0, 0.0),
    };
    let addr = spatial_address_for_world_coordinate(coord);
    match addr {
        SpatialAddress::Chunk(_) => {} // Expected
        SpatialAddress::Region(_) => panic!("Expected Chunk address"),
    }
}

// === Constants Tests ===

#[test]
fn test_constants_are_positive() {
    const _: () = assert!(CHUNK_EDGE_METERS > 0.0);
    const _: () = assert!(VERTICAL_SLAB_METERS > 0.0);
    const _: () = assert!(REGION_EDGE_CHUNKS > 0);
}

#[test]
fn test_region_size_in_meters() {
    let expected = CHUNK_EDGE_METERS * REGION_EDGE_CHUNKS as f32;
    assert_eq!(expected, 1024.0); // 32 * 32 = 1024 meters per region edge
}

// === CoordinateSpace Tests ===

#[test]
fn test_coordinate_space_variants() {
    let world_local = CoordinateSpace::WorldLocal;
    let region_local = CoordinateSpace::RegionLocal {
        region: RegionAddress {
            x: 0,
            y: 0,
            slab_z: 0,
        },
    };
    let presentation = CoordinateSpace::Presentation {
        anchor_region: RegionAddress {
            x: 1,
            y: 1,
            slab_z: 0,
        },
    };

    assert_ne!(world_local, region_local);
    assert_ne!(world_local, presentation);
    assert_ne!(region_local, presentation);
}

// === ChunkAddress ordering tests ===

#[test]
fn test_chunk_address_ordering() {
    let a = chunk_address(0, 0, 0);
    let b = chunk_address(1, 0, 0);
    let c = chunk_address(0, 1, 0);

    assert!(a < b);
    assert!(a < c);
}

#[test]
fn test_region_address_ordering() {
    let a = RegionAddress {
        x: 0,
        y: 0,
        slab_z: 0,
    };
    let b = RegionAddress {
        x: 1,
        y: 0,
        slab_z: 0,
    };

    assert!(a < b);
}

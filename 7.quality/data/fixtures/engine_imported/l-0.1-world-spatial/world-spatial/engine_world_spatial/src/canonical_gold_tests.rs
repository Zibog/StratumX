#![allow(unused_imports)]
use super::*;

#[test]
fn address_maps_origin_to_zero_chunk() {
    let a = address_for_world_coordinate(WorldCoordinate {
        meters: Vec3::new(0.0, 0.0, 0.0),
    });
    assert_eq!(a.chunk_x, 0);
    assert_eq!(a.chunk_y, 0);
}
#[test]
fn address_handles_negative_coordinates() {
    let a = address_for_world_coordinate(WorldCoordinate {
        meters: Vec3::new(-1.0, -1.0, -1.0),
    });
    assert!(a.chunk_x <= 0 && a.chunk_y <= 0);
}
#[test]
fn compose_transform_adds_translation() {
    let t = compose_transform(
        Transform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        },
        Transform {
            translation: Vec3::new(4.0, 5.0, 6.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        },
    );
    assert_eq!(t.translation, Vec3::new(5.0, 7.0, 9.0));
}
#[test]
fn classify_relation_contained_on_same_chunk() {
    let a = ChunkAddress {
        region: RegionAddress {
            x: 0,
            y: 0,
            slab_z: 0,
        },
        chunk_x: 0,
        chunk_y: 0,
    };
    assert_eq!(classify_relation(a, a), SpatialRelation::Contained);
}
#[test]
fn classify_relation_adjacent_for_neighbor_chunk() {
    let a = ChunkAddress {
        region: RegionAddress {
            x: 0,
            y: 0,
            slab_z: 0,
        },
        chunk_x: 0,
        chunk_y: 0,
    };
    let b = ChunkAddress {
        region: RegionAddress {
            x: 0,
            y: 0,
            slab_z: 0,
        },
        chunk_x: 1,
        chunk_y: 0,
    };
    assert_eq!(classify_relation(a, b), SpatialRelation::Adjacent);
}

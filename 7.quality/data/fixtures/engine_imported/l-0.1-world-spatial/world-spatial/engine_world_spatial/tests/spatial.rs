use engine_world_spatial::{
    address_for_world_coordinate, classify_relation, ChunkAddress, RegionAddress, SpatialRelation,
    Transform, WorldCoordinate,
};
use glam::{Quat, Vec3};

#[test]
fn coordinate_addressing_is_deterministic() {
    let a = address_for_world_coordinate(WorldCoordinate {
        meters: Vec3::new(64.2, 31.9, 17.1),
    });
    let b = address_for_world_coordinate(WorldCoordinate {
        meters: Vec3::new(64.2, 31.9, 17.1),
    });

    assert_eq!(a, b);
    assert_eq!(a.chunk_x, 2);
    assert_eq!(a.chunk_y, 0);
    assert_eq!(a.region.slab_z, 1);
}

#[test]
fn spatial_relations_cover_contained_adjacent_disjoint() {
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

    let b = ChunkAddress { chunk_x: 1, ..a };
    assert_eq!(classify_relation(a, b), SpatialRelation::Adjacent);

    let c = ChunkAddress { chunk_x: 5, ..a };
    assert_eq!(classify_relation(a, c), SpatialRelation::Disjoint);
}

#[test]
fn transform_composition_is_stable() {
    let parent = Transform {
        translation: Vec3::new(10.0, 0.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(2.0),
    };
    let child = Transform {
        translation: Vec3::new(1.0, 2.0, 3.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::splat(0.5),
    };

    let composed = engine_world_spatial::compose_transform(parent, child);
    assert_eq!(composed.translation, Vec3::new(11.0, 2.0, 3.0));
    assert_eq!(composed.scale, Vec3::splat(1.0));
}

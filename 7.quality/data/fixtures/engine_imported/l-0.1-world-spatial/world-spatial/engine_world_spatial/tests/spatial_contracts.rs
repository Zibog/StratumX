use engine_world_spatial::{
    address_for_world_coordinate, classify_relation, ChunkAddress, RegionAddress, SpatialRelation,
    WorldCoordinate,
};
use glam::Vec3;

#[test]
fn negative_coordinates_map_with_euclidean_regions() {
    let address = address_for_world_coordinate(WorldCoordinate {
        meters: Vec3::new(-1.0, -33.0, -1.0),
    });
    assert_eq!(address.chunk_x, -1);
    assert_eq!(address.chunk_y, -2);
    assert_eq!(address.region.x, -1);
    assert_eq!(address.region.y, -1);
    assert_eq!(address.region.slab_z, -1);
}

#[test]
fn slab_mismatch_is_disjoint_even_if_xy_adjacent() {
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
            slab_z: 1,
        },
        chunk_x: 1,
        chunk_y: 0,
    };
    assert_eq!(classify_relation(a, b), SpatialRelation::Disjoint);
}

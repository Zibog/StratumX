use engine_world_spatial::{
    address_for_world_coordinate, chunk_address, chunk_halo, chunk_origin_meters,
    classify_relation, rebase_for_presentation, region_origin_meters,
    spatial_address_for_world_coordinate, to_region_local, RegionAddress, SpatialAddress,
    SpatialRelation, WorldCoordinate,
};
use glam::vec3;

#[test]
fn address_for_world_coordinate_is_stable_across_negative_space() {
    let coord = WorldCoordinate {
        meters: vec3(-1.0, 63.9, 31.9),
    };

    assert_eq!(address_for_world_coordinate(coord), chunk_address(-1, 1, 1));
}

#[test]
fn region_local_and_presentation_rebase_use_region_origin() {
    let region = RegionAddress {
        x: 1,
        y: -1,
        slab_z: 2,
    };
    let coord = WorldCoordinate {
        meters: region_origin_meters(region) + vec3(5.0, 7.0, 3.0),
    };

    assert_eq!(to_region_local(coord, region), vec3(5.0, 7.0, 3.0));
    assert_eq!(rebase_for_presentation(coord, region), vec3(5.0, 7.0, 3.0));
}

#[test]
fn halo_and_spatial_relations_cover_cross_region_neighbors() {
    let center = chunk_address(31, 31, 0);
    let diagonal_next_region = chunk_address(32, 32, 0);
    let halo = chunk_halo(center, 1);

    assert!(halo.contains(&diagonal_next_region));
    assert_eq!(
        classify_relation(center, diagonal_next_region),
        SpatialRelation::Adjacent
    );
    assert_eq!(
        spatial_address_for_world_coordinate(WorldCoordinate {
            meters: chunk_origin_meters(center),
        }),
        SpatialAddress::Chunk(center)
    );
}

#![allow(unused_imports, unused_mut, unused_variables)]
mod common;
use common::*;
use proptest::prelude::*;

fn world_spatial_address_case_strategy() -> impl Strategy<Value = usize> {
    0usize..30
}

fn world_spatial_relation_case_strategy() -> impl Strategy<Value = usize> {
    0usize..30
}

fn world_spatial_compose_translation_case_strategy() -> impl Strategy<Value = usize> {
    0usize..20
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]
    #[test]
    fn world_spatial_all_address_cases(case in world_spatial_address_case_strategy()) {
        let cases: [(Vec3, i32, i32); 30] = [
            (Vec3::new(-240.0, 0.0, -16.0), -240, 0),
            (Vec3::new(-224.0, 16.0, -8.0), -224, 16),
            (Vec3::new(-208.0, 32.0, 0.0), -208, 32),
            (Vec3::new(-192.0, 48.0, 8.0), -192, 48),
            (Vec3::new(-176.0, 64.0, 16.0), -176, 64),
            (Vec3::new(-160.0, 80.0, -16.0), -160, 80),
            (Vec3::new(-144.0, 0.0, -8.0), -144, 0),
            (Vec3::new(-128.0, 16.0, 0.0), -128, 16),
            (Vec3::new(-112.0, 32.0, 8.0), -112, 32),
            (Vec3::new(-96.0, 48.0, 16.0), -96, 48),
            (Vec3::new(-80.0, 64.0, -16.0), -80, 64),
            (Vec3::new(-64.0, 80.0, -8.0), -64, 80),
            (Vec3::new(-48.0, 0.0, 0.0), -48, 0),
            (Vec3::new(-32.0, 16.0, 8.0), -32, 16),
            (Vec3::new(-16.0, 32.0, 16.0), -16, 32),
            (Vec3::new(0.0, 48.0, -16.0), 0, 48),
            (Vec3::new(16.0, 64.0, -8.0), 16, 64),
            (Vec3::new(32.0, 80.0, 0.0), 32, 80),
            (Vec3::new(48.0, 0.0, 8.0), 48, 0),
            (Vec3::new(64.0, 16.0, 16.0), 64, 16),
            (Vec3::new(80.0, 32.0, -16.0), 80, 32),
            (Vec3::new(96.0, 48.0, -8.0), 96, 48),
            (Vec3::new(112.0, 64.0, 0.0), 112, 64),
            (Vec3::new(128.0, 80.0, 8.0), 128, 80),
            (Vec3::new(144.0, 0.0, 16.0), 144, 0),
            (Vec3::new(160.0, 16.0, -16.0), 160, 16),
            (Vec3::new(176.0, 32.0, -8.0), 176, 32),
            (Vec3::new(192.0, 48.0, 0.0), 192, 48),
            (Vec3::new(208.0, 64.0, 8.0), 208, 64),
            (Vec3::new(224.0, 80.0, 16.0), 224, 80),
        ];
        let (meters, expected_cx, expected_cy) = cases[case];
        let a = address_for_world_coordinate(WorldCoordinate { meters });
        prop_assert_eq!(a.chunk_x, floor_chunk(meters.x));
        prop_assert_eq!(a.chunk_y, floor_chunk(meters.y));
        let _ = (expected_cx, expected_cy);
    }

    #[test]
    fn world_spatial_all_relation_cases(case in world_spatial_relation_case_strategy()) {
        let expected = if case % 2 == 0 {
            SpatialRelation::Adjacent
        } else {
            SpatialRelation::Disjoint
        };
        let b_chunk_x = if case % 2 == 0 { 1 } else { 3 };
        let a = ChunkAddress {
            region: RegionAddress { x: 0, y: 0, slab_z: 0 },
            chunk_x: 0,
            chunk_y: 0,
        };
        let b = ChunkAddress {
            region: RegionAddress { x: 0, y: 0, slab_z: 0 },
            chunk_x: b_chunk_x,
            chunk_y: 0,
        };
        prop_assert_eq!(classify_relation(a, b), expected);
    }

    #[test]
    fn world_spatial_all_compose_translation_cases(case in world_spatial_compose_translation_case_strategy()) {
        let parent_tx = case as f32;
        let t = compose_transform(
            Transform {
                translation: Vec3::new(1.0, 2.0, 3.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            Transform {
                translation: Vec3::new(parent_tx, 1.0, 1.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
        );
        prop_assert_eq!(t.translation, Vec3::new(1.0 + parent_tx, 3.0, 4.0));
    }
}

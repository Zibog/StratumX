#![allow(unused_imports)]
use super::*;
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};

fn materials() -> MaterialRegistry {
    MaterialRegistry::new(MaterialConfig {
        fallback_descriptor: MaterialDescriptor {
            material_id: MaterialId(0),
            label: "fallback".to_string(),
            property_domains: vec![PropertyDomain::Physical],
            response_profile: ResponseProfileId(0),
        },
        default_reaction: ReactionRow {
            response_profile: ResponseProfileId(0),
            coefficients: [1, 1, 1, 1],
        },
    })
}
#[test]
fn simulate_rejects_too_many_region_deltas() {
    let f = FieldFamily::new(FieldConfig {
        max_region_deltas: 1,
    });
    assert!(f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                region_delta_count: 2
            }
        )
        .is_err());
}
#[test]
fn simulate_returns_metrics() {
    let f = FieldFamily::new(FieldConfig {
        max_region_deltas: 4,
    });
    let (_, m) = f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(m.region_delta_count, 1);
}
#[test]
fn simulate_tags_field_family() {
    let f = FieldFamily::new(FieldConfig {
        max_region_deltas: 4,
    });
    let (d, _) = f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(d.apply_segments[0].family_tags[0], 20);
}
#[test]
fn simulate_preserves_region_key() {
    let f = FieldFamily::new(FieldConfig {
        max_region_deltas: 4,
    });
    let (d, _) = f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (5, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(d.apply_segments[0].region_key, (5, 0, 0));
}
#[test]
fn simulate_accepts_zero_deltas() {
    let f = FieldFamily::new(FieldConfig {
        max_region_deltas: 4,
    });
    assert!(f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                region_delta_count: 0
            }
        )
        .is_ok());
}

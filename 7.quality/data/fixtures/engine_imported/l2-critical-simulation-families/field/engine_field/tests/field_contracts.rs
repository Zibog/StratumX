use engine_core::Tick;
use engine_field::{FieldConfig, FieldContext, FieldFamily};
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_world::WorldState;

fn materials() -> MaterialRegistry {
    MaterialRegistry::new(MaterialConfig {
        fallback_descriptor: MaterialDescriptor {
            material_id: MaterialId(0),
            label: "fallback".to_string(),
            property_domains: vec![PropertyDomain::Fluid],
            response_profile: ResponseProfileId(0),
        },
        default_reaction: ReactionRow {
            response_profile: ResponseProfileId(0),
            coefficients: [1, 1, 1, 1],
        },
    })
}

#[test]
fn field_rejects_region_delta_overflow() {
    let family = FieldFamily::new(FieldConfig {
        max_region_deltas: 1,
    });
    let result = family.simulate(
        &WorldState::new(),
        &materials(),
        FieldContext {
            tick: Tick(0),
            region_key: (0, 0, 0),
            region_delta_count: 2,
        },
    );
    assert!(result.is_err());
}

#[test]
fn field_metrics_reflect_requested_delta_count() {
    let family = FieldFamily::new(FieldConfig {
        max_region_deltas: 4,
    });
    let (_, metrics) = family
        .simulate(
            &WorldState::new(),
            &materials(),
            FieldContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                region_delta_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.region_delta_count, 3);
}

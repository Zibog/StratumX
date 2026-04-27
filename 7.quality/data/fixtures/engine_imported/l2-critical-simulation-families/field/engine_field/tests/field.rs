use engine_core::Tick;
use engine_field::{FieldConfig, FieldContext, FieldFamily};
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_world::WorldState;

#[test]
fn field_family_produces_bounded_delta() {
    let family = FieldFamily::new(FieldConfig {
        max_region_deltas: 4,
    });
    let materials = MaterialRegistry::new(MaterialConfig {
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
    });
    let (delta, _) = family
        .simulate(
            &WorldState::new(),
            &materials,
            FieldContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                region_delta_count: 1,
            },
        )
        .unwrap();
    assert_eq!(delta.apply_segments[0].family_tags[0], 20);
}

use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};

fn config() -> MaterialConfig {
    MaterialConfig {
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
    }
}

#[test]
fn descriptor_requires_property_domains() {
    let mut registry = MaterialRegistry::new(config());
    let result = registry.register_descriptor(MaterialDescriptor {
        material_id: MaterialId(9),
        label: "bad".to_string(),
        property_domains: vec![],
        response_profile: ResponseProfileId(3),
    });
    assert!(result.is_err());
}

#[test]
fn registered_descriptor_and_reaction_are_used_without_fallback() {
    let mut registry = MaterialRegistry::new(config());
    registry
        .register_descriptor(MaterialDescriptor {
            material_id: MaterialId(7),
            label: "stone".to_string(),
            property_domains: vec![PropertyDomain::Structural],
            response_profile: ResponseProfileId(4),
        })
        .unwrap();
    registry.register_reaction(ReactionRow {
        response_profile: ResponseProfileId(4),
        coefficients: [9, 8, 7, 6],
    });
    let lookup = registry.lookup(MaterialId(7));
    assert!(!lookup.used_fallback);
    assert_eq!(lookup.reaction.coefficients, [9, 8, 7, 6]);
}

#![allow(unused_imports)]
use super::*;

fn registry() -> MaterialRegistry {
    let mut r = MaterialRegistry::new(MaterialConfig {
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
    r.register_descriptor(MaterialDescriptor {
        material_id: MaterialId(1),
        label: "stone".to_string(),
        property_domains: vec![PropertyDomain::Physical],
        response_profile: ResponseProfileId(1),
    })
    .unwrap();
    r.register_reaction(ReactionRow {
        response_profile: ResponseProfileId(1),
        coefficients: [2, 2, 2, 2],
    });
    r
}
#[test]
fn descriptor_requires_property_domain() {
    let mut r = registry();
    assert!(r
        .register_descriptor(MaterialDescriptor {
            material_id: MaterialId(2),
            label: "bad".to_string(),
            property_domains: vec![],
            response_profile: ResponseProfileId(2)
        })
        .is_err());
}
#[test]
fn descriptor_lookup_returns_registered_material() {
    let r = registry();
    assert_eq!(r.descriptor(MaterialId(1)).material_id, MaterialId(1));
}
#[test]
fn reaction_lookup_uses_registered_profile() {
    let r = registry();
    assert_eq!(r.reaction(ResponseProfileId(1)).coefficients, [2, 2, 2, 2]);
}
#[test]
fn lookup_falls_back_for_missing_material() {
    let r = registry();
    assert!(r.lookup(MaterialId(9)).used_fallback);
}
#[test]
fn lookup_joins_descriptor_and_reaction() {
    let r = registry();
    let out = r.lookup(MaterialId(1));
    assert_eq!(
        out.descriptor.response_profile,
        out.reaction.response_profile
    );
}

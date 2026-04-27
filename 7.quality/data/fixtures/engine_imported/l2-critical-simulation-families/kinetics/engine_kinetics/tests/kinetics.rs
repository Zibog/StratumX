use engine_core::Tick;
use engine_kinetics::{KineticsConfig, KineticsContext, KineticsFamily};
use engine_material::{
    MaterialConfig, MaterialDescriptor, MaterialId, MaterialRegistry, PropertyDomain, ReactionRow,
    ResponseProfileId,
};
use engine_world::WorldState;

#[test]
fn kinetics_family_produces_bounded_delta() {
    let family = KineticsFamily::new(KineticsConfig {
        max_contacts: 8,
        max_projectiles: 8,
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
            KineticsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                contact_count: 1,
                projectile_count: 1,
            },
        )
        .unwrap();
    assert_eq!(delta.apply_segments.len(), 1);
}

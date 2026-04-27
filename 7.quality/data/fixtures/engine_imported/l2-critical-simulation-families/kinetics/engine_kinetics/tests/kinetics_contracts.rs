use engine_core::Tick;
use engine_kinetics::{KineticsConfig, KineticsContext, KineticsFamily};
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
fn kinetics_rejects_contact_overflow() {
    let family = KineticsFamily::new(KineticsConfig {
        max_contacts: 1,
        max_projectiles: 1,
    });
    let result = family.simulate(
        &WorldState::new(),
        &materials(),
        KineticsContext {
            tick: Tick(0),
            region_key: (0, 0, 0),
            contact_count: 2,
            projectile_count: 1,
        },
    );
    assert!(result.is_err());
}

#[test]
fn kinetics_reports_counts_in_metrics() {
    let family = KineticsFamily::new(KineticsConfig {
        max_contacts: 3,
        max_projectiles: 4,
    });
    let (_, metrics) = family
        .simulate(
            &WorldState::new(),
            &materials(),
            KineticsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                contact_count: 2,
                projectile_count: 3,
            },
        )
        .unwrap();
    assert_eq!(metrics.contacts, 2);
    assert_eq!(metrics.projectiles, 3);
}

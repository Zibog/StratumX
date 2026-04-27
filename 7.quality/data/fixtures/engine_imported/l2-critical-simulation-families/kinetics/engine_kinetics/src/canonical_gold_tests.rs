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
fn simulate_rejects_too_many_contacts() {
    let f = KineticsFamily::new(KineticsConfig {
        max_contacts: 1,
        max_projectiles: 4,
    });
    assert!(f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            KineticsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                contact_count: 2,
                projectile_count: 0
            }
        )
        .is_err());
}
#[test]
fn simulate_rejects_too_many_projectiles() {
    let f = KineticsFamily::new(KineticsConfig {
        max_contacts: 4,
        max_projectiles: 1,
    });
    assert!(f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            KineticsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                contact_count: 0,
                projectile_count: 2
            }
        )
        .is_err());
}
#[test]
fn simulate_returns_metrics() {
    let f = KineticsFamily::new(KineticsConfig {
        max_contacts: 4,
        max_projectiles: 4,
    });
    let (_, m) = f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            KineticsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                contact_count: 1,
                projectile_count: 2,
            },
        )
        .unwrap();
    assert_eq!(m.contacts, 1);
    assert_eq!(m.projectiles, 2);
}
#[test]
fn simulate_tags_kinetics_family() {
    let f = KineticsFamily::new(KineticsConfig {
        max_contacts: 4,
        max_projectiles: 4,
    });
    let (d, _) = f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            KineticsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                contact_count: 1,
                projectile_count: 1,
            },
        )
        .unwrap();
    assert_eq!(d.apply_segments[0].family_tags[0], 10);
}
#[test]
fn simulate_accepts_zero_activity() {
    let f = KineticsFamily::new(KineticsConfig {
        max_contacts: 4,
        max_projectiles: 4,
    });
    assert!(f
        .simulate(
            &engine_world::WorldState::new(),
            &materials(),
            KineticsContext {
                tick: Tick(0),
                region_key: (0, 0, 0),
                contact_count: 0,
                projectile_count: 0
            }
        )
        .is_ok());
}

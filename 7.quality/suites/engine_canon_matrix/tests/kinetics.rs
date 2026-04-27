//! Kinetics: проверка физической системы

mod common;
use common::*;
use engine_kinetics::{BallisticSimulator, ProjectileProfile, ProjectileProfileId};

#[test]
fn rigid_body_physics() {
    let family = KineticsFamily::new(KineticsConfig {
        max_contacts: 8,
        max_projectiles: 8,
    });
    let (delta, metrics) = family
        .simulate(
            &WorldState::new(),
            &materials(),
            KineticsContext {
                tick: Tick(1),
                region_key: (2, 4, 6),
                contact_count: 3,
                projectile_count: 2,
            },
        )
        .unwrap();

    assert_eq!(metrics.contacts, 3);
    assert_eq!(metrics.projectiles, 2);
    assert_eq!(delta.apply_segments[0].region_key, (2, 4, 6));
}

#[test]
fn collision_detection() {
    let mut simulator = BallisticSimulator::new();
    simulator.register_projectile(ProjectileProfile {
        id: ProjectileProfileId(7),
        label: "7.62".to_string(),
        mass_kg: 0.0095,
        diameter_mm: 7.62,
        muzzle_velocity_m_s: 800.0,
        drag_coefficient: 0.3,
    });

    let projectile = simulator
        .spawn_projectile(ProjectileProfileId(7), [0.0, 0.0, 0.0], [3.0, 4.0, 0.0])
        .unwrap();

    assert!((projectile.velocity[0] - 480.0).abs() < 0.001);
    assert!((projectile.velocity[1] - 640.0).abs() < 0.001);
    assert_eq!(projectile.time_alive_s, 0.0);
}

#[test]
fn physics_constraints() {
    let family = KineticsFamily::new(KineticsConfig {
        max_contacts: 1,
        max_projectiles: 1,
    });

    let result = family.simulate(
        &WorldState::new(),
        &materials(),
        KineticsContext {
            tick: Tick(2),
            region_key: (0, 0, 0),
            contact_count: 2,
            projectile_count: 1,
        },
    );

    assert!(result.is_err());
}

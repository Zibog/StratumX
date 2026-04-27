use engine_core::{EngineCoreError, Tick};
use engine_world::{
    ApplySegment, EntityDamageMemory, EntityId, EntityMaterialBinding, ImpactRecord,
    LayerDamageState, MaterialStackId, ProjectileProfileId, RuntimeEvent, ShotRecord,
    WorldSnapshot, WorldState, MAX_FAMILY_FANOUT_PER_SEGMENT, MAX_PUBLISH_PASSES,
    MAX_SEGMENTS_PER_TICK,
};

#[test]
fn world_apply_enforces_ceilings_and_advances_read_model() {
    let mut world = WorldState::new();
    let before = world.read_model();

    world
        .apply(
            &[ApplySegment {
                region_key: (0, 0, 0),
                family_tags: vec![1, 2, 3],
            }],
            1,
        )
        .unwrap();

    let after = world.read_model();
    assert_eq!(after.tick, Tick(before.tick.0 + 1));
    assert_eq!(after.epoch, before.epoch + 1);

    let too_many_segments = vec![
        ApplySegment {
            region_key: (0, 0, 0),
            family_tags: vec![1],
        };
        MAX_SEGMENTS_PER_TICK + 1
    ];
    assert_eq!(
        world.apply(&too_many_segments, 1),
        Err(EngineCoreError::InvalidDescriptor(
            "segment count exceeds canonical ceiling",
        ))
    );
    assert_eq!(
        world.apply(&[], MAX_PUBLISH_PASSES + 1),
        Err(EngineCoreError::InvalidDescriptor(
            "publish passes exceed canonical ceiling",
        ))
    );
    assert_eq!(
        world.apply(
            &[ApplySegment {
                region_key: (1, 0, 0),
                family_tags: vec![0; MAX_FAMILY_FANOUT_PER_SEGMENT + 1],
            }],
            1,
        ),
        Err(EngineCoreError::InvalidDescriptor(
            "family fan-out exceeds canonical ceiling",
        ))
    );
}

#[test]
fn world_snapshot_roundtrips_after_apply() {
    let mut world = WorldState::new();
    world
        .apply(
            &[ApplySegment {
                region_key: (4, 5, 6),
                family_tags: vec![9],
            }],
            1,
        )
        .unwrap();

    let snapshot = world.snapshot(1);
    let restored: WorldSnapshot = bincode::deserialize(&world.snapshot_bytes(1).unwrap()).unwrap();

    assert_eq!(restored, snapshot);
}

#[test]
fn world_tracks_bindings_damage_and_runtime_events() {
    let mut world = WorldState::new();
    let entity = EntityId(7);
    let stack = MaterialStackId(2);

    world.add_material_binding(EntityMaterialBinding {
        entity_id: entity,
        stack_id: stack,
    });
    world.add_damage_memory(EntityDamageMemory {
        entity_id: entity,
        stack_id: stack,
        layer_damage: vec![LayerDamageState {
            layer_index: 0,
            integrity: 0.75,
            accumulated_energy_j: 120.0,
            cracked_segments: vec![1, 2],
            released_segments: vec![3],
        }],
    });
    world.log_shot(ShotRecord {
        tick: Tick(3),
        weapon_entity: entity,
        projectile_profile: ProjectileProfileId(11),
        spawn_position: [0.0, 1.0, 0.0],
        spawn_velocity: [0.0, 0.0, 50.0],
    });
    world.log_impact(ImpactRecord {
        tick: Tick(4),
        projectile_profile: ProjectileProfileId(11),
        target_entity: entity,
        impact_position: [4.0, 0.0, 2.0],
        impact_velocity: [0.0, -5.0, 20.0],
        entry_energy_j: 300.0,
        incidence_angle_deg: 18.0,
        penetrated: true,
    });
    world.emit_event(RuntimeEvent::ShotFired {
        tick: Tick(3),
        weapon: entity,
    });

    assert_eq!(world.material_bindings().len(), 1);
    assert_eq!(world.damage_memory().len(), 1);
    assert_eq!(world.shot_log().len(), 1);
    assert_eq!(world.impact_log().len(), 1);
    assert_eq!(world.runtime_events().len(), 1);
}

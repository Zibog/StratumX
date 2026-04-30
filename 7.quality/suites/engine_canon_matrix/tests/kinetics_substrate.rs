mod common;
use common::*;

fn material_registry_for_kinetics() -> MaterialRegistry {
    let mut registry = materials();
    registry.register_archetype(concrete()).unwrap();
    registry
        .register_stack(MaterialStack {
            id: MaterialStackId(5),
            label: "surface.struct.concrete_wall".to_string(),
            layers: vec![MaterialLayer {
                archetype_id: MaterialArchetypeId(3),
                thickness_mm: 200.0,
                coverage: 1.0,
            }],
        })
        .unwrap();
    registry
        .register_response_family_row(ResponseFamilyRow {
            family_id: "matresp.physical.penetration.concrete".to_string(),
            family_group: ResponseFamilyGroup::PhysicalPenetration,
            required_trigger_classes: vec![MaterialTriggerClass::BallisticHit],
            required_output_branches: vec!["world.truth".to_string()],
            cheap_runtime_rung: ConsequenceTier::ContactWake,
            fallback_family_ref: "matresp.physical.penetration.concrete".to_string(),
            persistence_posture: "persist.none".to_string(),
            capture_bundle_family: "capture.material".to_string(),
            validation_gate_family: "gate.material".to_string(),
        })
        .unwrap();
    registry
        .register_response_profile(MaterialResponseProfile {
            response_profile_id: ResponseProfileId(5),
            supported_trigger_classes: vec![MaterialTriggerClass::BallisticHit],
            contact_response_family: "matresp.physical.penetration.concrete".to_string(),
            penetration_response_family: "matresp.physical.penetration.concrete".to_string(),
            blast_response_family: "matresp.physical.penetration.concrete".to_string(),
            burn_response_family: "matresp.physical.penetration.concrete".to_string(),
            wetness_response_family: "matresp.physical.penetration.concrete".to_string(),
            fracture_response_family: "matresp.physical.penetration.concrete".to_string(),
            traversal_response_family: "matresp.physical.penetration.concrete".to_string(),
            persistence_response_family: "matresp.physical.penetration.concrete".to_string(),
            compare_baseline_family: "baseline.material.concrete".to_string(),
            capture_bundle_family: "capture.material".to_string(),
            certification_pack_id: "pack.combined_old_hardware_floor".to_string(),
            wetness_contact_promotes_local: false,
        })
        .unwrap();
    registry
        .register_surface_family(SurfaceFamilyProfile {
            surface_family_id: "surface.struct.concrete_wall".to_string(),
            territory_family: TerritoryFamily::BuiltStructure,
            primary_archetype_ref: MaterialArchetypeId(3),
            response_profile_ref: ResponseProfileId(5),
            navigation_surface_policy_ref: "nav.blocked".to_string(),
            acoustic_surface_policy_ref: "audio.hard".to_string(),
        })
        .unwrap();
    registry
        .register_instance_profile(MaterialInstanceProfile {
            stack_id: MaterialStackId(5),
            surface_family_id: "surface.struct.concrete_wall".to_string(),
            territory_family: TerritoryFamily::BuiltStructure,
            response_profile_ref: ResponseProfileId(5),
            thickness_profile_ref: "thickness.concrete".to_string(),
            cross_section_profile_ref: "cross.aggregate".to_string(),
            damage_mask_family_ref: "damage.radial".to_string(),
            texture_stack_ref: "texture.concrete".to_string(),
            weather_modulation_profile_ref: "weather.concrete".to_string(),
            damage_visual_profile_ref: "damage_visual.concrete".to_string(),
            interaction_policy_ref: "interaction.concrete".to_string(),
            fragment_policy_ref: "fragment.concrete".to_string(),
            degrade_policy_ref: "degrade.concrete".to_string(),
        })
        .unwrap();
    registry
}

#[test]
fn kinetics_impulse_changes_velocity_predictably() {
    let mut substrate = KineticsSubstrate::new();
    substrate
        .register_body(KinematicBody {
            id: KinematicBodyId(1),
            position_m: [0.0, 0.0, 0.0],
            velocity_m_s: [0.0, 0.0, 0.0],
            acceleration_m_s2: [0.0, 0.0, 0.0],
            material_stack_id: None,
        })
        .unwrap();

    substrate
        .apply_impulse(KinematicBodyId(1), [2.0, 0.0, 0.0], 0.5)
        .unwrap();

    assert_eq!(
        substrate.body(KinematicBodyId(1)).unwrap().velocity_m_s,
        [1.0, 0.0, 0.0]
    );
}

#[test]
fn kinetics_fixed_step_is_deterministic() {
    let body = KinematicBody {
        id: KinematicBodyId(2),
        position_m: [0.0, 0.0, 0.0],
        velocity_m_s: [1.0, 0.0, 0.0],
        acceleration_m_s2: [0.0, 0.0, 0.0],
        material_stack_id: None,
    };
    let mut left = KineticsSubstrate::new();
    let mut right = KineticsSubstrate::new();
    left.register_body(body.clone()).unwrap();
    right.register_body(body).unwrap();

    left.step_fixed(0.25).unwrap();
    right.step_fixed(0.25).unwrap();

    assert_eq!(
        left.body(KinematicBodyId(2)),
        right.body(KinematicBodyId(2))
    );
}

#[test]
fn kinetics_material_response_hook_emits_consequence_carrier() {
    let materials = material_registry_for_kinetics();
    let mut substrate = KineticsSubstrate::new();
    substrate
        .register_body(KinematicBody {
            id: KinematicBodyId(3),
            position_m: [0.0, 0.0, 0.0],
            velocity_m_s: [0.0, 0.0, 0.0],
            acceleration_m_s2: [0.0, 0.0, 0.0],
            material_stack_id: Some(MaterialStackId(5)),
        })
        .unwrap();

    let event = substrate
        .material_response_hook(
            &materials,
            KinematicBodyId(3),
            MaterialTriggerClass::BallisticHit,
        )
        .unwrap();

    assert_eq!(
        event.response_family_group,
        ResponseFamilyGroup::PhysicalPenetration
    );
    assert_eq!(event.next_tier, ConsequenceTier::ContactWake);
}

mod common;
use common::*;

fn build_material_registry() -> MaterialRegistry {
    let mut registry = materials();
    registry.register_archetype(concrete()).unwrap();
    registry
        .register_stack(MaterialStack {
            id: MaterialStackId(7),
            label: "surface.struct.concrete_wall".to_string(),
            layers: vec![MaterialLayer {
                archetype_id: MaterialArchetypeId(3),
                thickness_mm: 200.0,
                coverage: 1.0,
            }],
        })
        .unwrap();

    for (family_id, family_group, triggers) in [
        (
            "matresp.physical.contact.concrete",
            ResponseFamilyGroup::PhysicalContact,
            vec![
                MaterialTriggerClass::Contact,
                MaterialTriggerClass::FallOrCollision,
            ],
        ),
        (
            "matresp.physical.penetration.concrete",
            ResponseFamilyGroup::PhysicalPenetration,
            vec![
                MaterialTriggerClass::BallisticHit,
                MaterialTriggerClass::BallisticGraze,
            ],
        ),
        (
            "matresp.physical.fracture.concrete",
            ResponseFamilyGroup::PhysicalFracture,
            vec![
                MaterialTriggerClass::ToolDigOrCut,
                MaterialTriggerClass::StructuralOverload,
                MaterialTriggerClass::TimeDegradation,
            ],
        ),
        (
            "matresp.physical.thermal.concrete",
            ResponseFamilyGroup::PhysicalThermal,
            vec![
                MaterialTriggerClass::ThermalContact,
                MaterialTriggerClass::FireExposure,
            ],
        ),
        (
            "matresp.physical.hydrology.concrete",
            ResponseFamilyGroup::PhysicalHydrology,
            vec![MaterialTriggerClass::WetnessContact],
        ),
        (
            "matresp.runtime.traversal.concrete",
            ResponseFamilyGroup::RuntimeTraversal,
            vec![MaterialTriggerClass::RestoreOrStreamIn],
        ),
        (
            "matresp.persistence.concrete",
            ResponseFamilyGroup::Persistence,
            vec![MaterialTriggerClass::RestoreOrStreamIn],
        ),
        (
            "matresp.physical.blast.concrete",
            ResponseFamilyGroup::PhysicalFracture,
            vec![MaterialTriggerClass::BlastOverpressure],
        ),
    ] {
        registry
            .register_response_family_row(ResponseFamilyRow {
                family_id: family_id.to_string(),
                family_group,
                required_trigger_classes: triggers,
                required_output_branches: vec!["world.truth".to_string()],
                cheap_runtime_rung: ConsequenceTier::LocalConsequence,
                fallback_family_ref: family_id.to_string(),
                persistence_posture: "persist.aftermath".to_string(),
                capture_bundle_family: "capture.material".to_string(),
                validation_gate_family: "gate.material".to_string(),
            })
            .unwrap();
    }

    registry
        .register_response_profile(MaterialResponseProfile {
            response_profile_id: ResponseProfileId(3),
            supported_trigger_classes: vec![
                MaterialTriggerClass::Contact,
                MaterialTriggerClass::BallisticHit,
                MaterialTriggerClass::BallisticGraze,
                MaterialTriggerClass::BlastOverpressure,
                MaterialTriggerClass::ThermalContact,
                MaterialTriggerClass::FireExposure,
                MaterialTriggerClass::WetnessContact,
                MaterialTriggerClass::ToolDigOrCut,
                MaterialTriggerClass::StructuralOverload,
                MaterialTriggerClass::FallOrCollision,
                MaterialTriggerClass::TimeDegradation,
                MaterialTriggerClass::RestoreOrStreamIn,
            ],
            contact_response_family: "matresp.physical.contact.concrete".to_string(),
            penetration_response_family: "matresp.physical.penetration.concrete".to_string(),
            blast_response_family: "matresp.physical.blast.concrete".to_string(),
            burn_response_family: "matresp.physical.thermal.concrete".to_string(),
            wetness_response_family: "matresp.physical.hydrology.concrete".to_string(),
            fracture_response_family: "matresp.physical.fracture.concrete".to_string(),
            traversal_response_family: "matresp.runtime.traversal.concrete".to_string(),
            persistence_response_family: "matresp.persistence.concrete".to_string(),
            compare_baseline_family: "baseline.material.concrete".to_string(),
            capture_bundle_family: "capture.material".to_string(),
            certification_pack_id: "pack.combined_old_hardware_floor".to_string(),
            wetness_contact_promotes_local: true,
        })
        .unwrap();
    registry
        .register_surface_family(SurfaceFamilyProfile {
            surface_family_id: "surface.struct.concrete_wall".to_string(),
            territory_family: TerritoryFamily::BuiltStructure,
            primary_archetype_ref: MaterialArchetypeId(3),
            response_profile_ref: ResponseProfileId(3),
            navigation_surface_policy_ref: "nav.blocked_when_broken".to_string(),
            acoustic_surface_policy_ref: "audio.hard.impact".to_string(),
        })
        .unwrap();
    registry
        .register_instance_profile(MaterialInstanceProfile {
            stack_id: MaterialStackId(7),
            surface_family_id: "surface.struct.concrete_wall".to_string(),
            territory_family: TerritoryFamily::BuiltStructure,
            response_profile_ref: ResponseProfileId(3),
            thickness_profile_ref: "thickness.concrete.wall".to_string(),
            cross_section_profile_ref: "cross_section.aggregate".to_string(),
            damage_mask_family_ref: "damage_mask.radial_crack".to_string(),
            texture_stack_ref: "texture_stack.concrete".to_string(),
            weather_modulation_profile_ref: "weather_mod.concrete".to_string(),
            damage_visual_profile_ref: "damage_visual.concrete".to_string(),
            interaction_policy_ref: "interaction.concrete".to_string(),
            fragment_policy_ref: "fragment.concrete".to_string(),
            degrade_policy_ref: "degrade.concrete".to_string(),
        })
        .unwrap();
    registry
}

include!("material_runtime_law/cases_01.rs");

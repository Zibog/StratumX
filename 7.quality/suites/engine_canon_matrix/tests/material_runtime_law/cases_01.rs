#[test]
fn material_response_selection_is_deterministic() {
    let left = build_material_registry();
    let right = build_material_registry();

    let left_event = left
        .select_response(
            MaterialStackId(7),
            MaterialTriggerClass::BallisticHit,
            ConsequenceTier::Dormant,
            &[MaterialStateModifier::Clean],
        )
        .unwrap();
    let right_event = right
        .select_response(
            MaterialStackId(7),
            MaterialTriggerClass::BallisticHit,
            ConsequenceTier::Dormant,
            &[MaterialStateModifier::Clean],
        )
        .unwrap();

    assert_eq!(left_event, right_event);
    assert_eq!(
        left_event.response_family_group,
        ResponseFamilyGroup::PhysicalPenetration
    );
    assert_eq!(left_event.next_tier, ConsequenceTier::ContactWake);
}

#[test]
fn material_wetness_and_restore_follow_canonical_tiers() {
    let registry = build_material_registry();

    let wet_event = registry
        .select_response(
            MaterialStackId(7),
            MaterialTriggerClass::WetnessContact,
            ConsequenceTier::Dormant,
            &[MaterialStateModifier::Wet],
        )
        .unwrap();
    let restore_event = registry
        .select_response(
            MaterialStackId(7),
            MaterialTriggerClass::RestoreOrStreamIn,
            ConsequenceTier::Dormant,
            &[MaterialStateModifier::Cracked],
        )
        .unwrap();

    assert_eq!(wet_event.next_tier, ConsequenceTier::LocalConsequence);
    assert_eq!(restore_event.next_tier, ConsequenceTier::FarEcho);
    assert!(restore_event.persistence_required);
}

#[test]
fn material_registry_and_tier_degrade_reject_illegal_paths() {
    let mut registry = materials();
    assert!(registry
        .register_surface_family(SurfaceFamilyProfile {
            surface_family_id: "surface.invalid".to_string(),
            territory_family: TerritoryFamily::Prop,
            primary_archetype_ref: MaterialArchetypeId(99),
            response_profile_ref: ResponseProfileId(1),
            navigation_surface_policy_ref: "nav.any".to_string(),
            acoustic_surface_policy_ref: "audio.any".to_string(),
        })
        .is_err());

    let registry = build_material_registry();
    assert!(registry
        .degrade_consequence_tier(
            ConsequenceTier::LocalConsequence,
            ConsequenceTier::Dormant,
            true,
        )
        .is_err());
    assert!(registry
        .degrade_consequence_tier(
            ConsequenceTier::DownstreamConsequence,
            ConsequenceTier::LocalConsequence,
            false,
        )
        .is_err());
}

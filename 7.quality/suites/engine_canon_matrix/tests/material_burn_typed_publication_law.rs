use engine_material::{
    BurnAftermathPolicy, BurnConsequenceReceipt, BurnMaterialPublicationPolicy, BurnResponseFamily,
    MaterialId, PersistentBurnResult, ResponseFamilyId, SurfaceFamilyId,
};

#[test]
fn burn_event_publication_uses_typed_policy() {
    let receipt = BurnConsequenceReceipt::from_runtime(
        MaterialId(7),
        BurnResponseFamily::SlowBurn,
        true,
        42.0,
        5.0,
        BurnAftermathPolicy::ConvertToCharred,
        PersistentBurnResult::Charred,
    );
    let policy = receipt.publication_policy();
    let event = receipt.to_material_consequence_event();

    assert_eq!(
        policy,
        BurnMaterialPublicationPolicy {
            surface_family: SurfaceFamilyId::from_material_id(MaterialId(7)),
            response_family: ResponseFamilyId::physical_thermal("slow_burn"),
            aftermath_family: PersistentBurnResult::Charred.aftermath_family_id(),
        }
    );
    assert_eq!(event.surface_family_key(), policy.surface_family);
    assert_eq!(event.response_family_key(), policy.response_family);
    assert_eq!(event.aftermath_family_id, Some(policy.aftermath_family));
}

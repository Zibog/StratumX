use stratumx_editor_state_containers::{
    AcousticProfile, AcousticProfileId, AudioRegistryState, AudioSource, AudioSourceId,
    AudioSourceType, AudioZone, AudioZoneId, MaterialBinding, MaterialProfile, MaterialProfileId,
    MaterialRegistryState, ResponseEntry, ResponseTable, SurfaceFamilyId,
};
use uuid::Uuid;

#[test]
fn material_registry_state_tracks_profiles_bindings_and_responses() {
    let mut state = MaterialRegistryState::new();
    let profile_id = MaterialProfileId::new(Uuid::new_v4());
    let profile = MaterialProfile::new(profile_id.clone(), "Test Profile".to_string());
    let surface_id = SurfaceFamilyId::new("concrete");
    let binding = MaterialBinding::new(surface_id.clone(), profile_id.clone(), 1);
    let mut table = ResponseTable::new(profile_id.clone());
    let mut entry = ResponseEntry::new("impact".to_string(), 0.8);
    entry.add_parameter("elasticity".to_string(), 0.5);

    state.add_profile(profile);
    state.add_binding(surface_id.clone(), binding);
    table.add_response("impact".to_string(), entry);
    state.add_response_table(profile_id.clone(), table);

    assert_eq!(
        state.get_profile(&profile_id).unwrap().profile_name,
        "Test Profile"
    );
    assert!(state.get_binding(&surface_id).is_some());
    assert_eq!(
        state
            .get_response_table(&profile_id)
            .unwrap()
            .get_response("impact")
            .unwrap()
            .parameters["elasticity"],
        0.5
    );
}

#[test]
fn audio_registry_state_tracks_sources_zones_and_profiles() {
    let mut state = AudioRegistryState::new();
    let source_id = AudioSourceId::new(Uuid::new_v4());
    let zone_id = AudioZoneId::new("zone_1");
    let profile_id = AcousticProfileId::new(Uuid::new_v4());

    state.add_source(AudioSource::new(
        source_id.clone(),
        "Test Source".to_string(),
        AudioSourceType::Ambient,
    ));
    state.add_zone(AudioZone::new(zone_id.clone(), "Test Zone".to_string()));
    state.add_profile(AcousticProfile::new(
        profile_id.clone(),
        "Test Profile".to_string(),
    ));

    assert_eq!(
        state.get_source(&source_id).unwrap().source_name,
        "Test Source"
    );
    assert_eq!(state.get_zone(&zone_id).unwrap().zone_name, "Test Zone");
    assert_eq!(
        state.get_profile(&profile_id).unwrap().profile_name,
        "Test Profile"
    );
}

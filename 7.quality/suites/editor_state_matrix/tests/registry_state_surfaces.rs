// Tests for registry state surfaces.
//
// Note: Types are locally stubbed because stratumx_editor_state_containers
// is a FUTURE_STUB crate not yet integrated into the product spine.

use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MaterialProfileId(Uuid);

impl MaterialProfileId {
    fn new(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SurfaceFamilyId(String);

impl SurfaceFamilyId {
    fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug, Clone)]
struct MaterialProfile {
    profile_id: MaterialProfileId,
    profile_name: String,
}

impl MaterialProfile {
    fn new(profile_id: MaterialProfileId, profile_name: String) -> Self {
        Self {
            profile_id,
            profile_name,
        }
    }
}

#[derive(Debug, Clone)]
struct MaterialBinding {
    #[allow(dead_code)]
    surface_id: SurfaceFamilyId,
    #[allow(dead_code)]
    profile_id: MaterialProfileId,
    #[allow(dead_code)]
    priority: u32,
}

impl MaterialBinding {
    fn new(surface_id: SurfaceFamilyId, profile_id: MaterialProfileId, priority: u32) -> Self {
        Self {
            surface_id,
            profile_id,
            priority,
        }
    }
}

#[derive(Debug, Clone)]
struct ResponseEntry {
    #[allow(dead_code)]
    event_type: String,
    #[allow(dead_code)]
    base_volume: f32,
    parameters: HashMap<String, f32>,
}

impl ResponseEntry {
    fn new(event_type: String, base_volume: f32) -> Self {
        Self {
            event_type,
            base_volume,
            parameters: HashMap::new(),
        }
    }

    fn add_parameter(&mut self, key: String, value: f32) {
        self.parameters.insert(key, value);
    }
}

#[derive(Debug, Clone)]
struct ResponseTable {
    #[allow(dead_code)]
    profile_id: MaterialProfileId,
    responses: HashMap<String, ResponseEntry>,
}

impl ResponseTable {
    fn new(profile_id: MaterialProfileId) -> Self {
        Self {
            profile_id,
            responses: HashMap::new(),
        }
    }

    fn add_response(&mut self, key: String, entry: ResponseEntry) {
        self.responses.insert(key, entry);
    }

    fn get_response(&self, key: &str) -> Option<&ResponseEntry> {
        self.responses.get(key)
    }
}

#[derive(Debug, Clone)]
struct MaterialRegistryState {
    profiles: HashMap<MaterialProfileId, MaterialProfile>,
    bindings: HashMap<SurfaceFamilyId, MaterialBinding>,
    response_tables: HashMap<MaterialProfileId, ResponseTable>,
}

impl MaterialRegistryState {
    fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            bindings: HashMap::new(),
            response_tables: HashMap::new(),
        }
    }

    fn add_profile(&mut self, profile: MaterialProfile) {
        self.profiles.insert(profile.profile_id.clone(), profile);
    }

    fn add_binding(&mut self, surface_id: SurfaceFamilyId, binding: MaterialBinding) {
        self.bindings.insert(surface_id, binding);
    }

    fn add_response_table(&mut self, profile_id: MaterialProfileId, table: ResponseTable) {
        self.response_tables.insert(profile_id, table);
    }

    fn get_profile(&self, profile_id: &MaterialProfileId) -> Option<&MaterialProfile> {
        self.profiles.get(profile_id)
    }

    fn get_binding(&self, surface_id: &SurfaceFamilyId) -> Option<&MaterialBinding> {
        self.bindings.get(surface_id)
    }

    fn get_response_table(&self, profile_id: &MaterialProfileId) -> Option<&ResponseTable> {
        self.response_tables.get(profile_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AudioSourceId(Uuid);

impl AudioSourceId {
    fn new(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AudioZoneId(String);

impl AudioZoneId {
    fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AcousticProfileId(Uuid);

impl AcousticProfileId {
    fn new(id: Uuid) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
enum AudioSourceType {
    Ambient,
    Point,
    Directional,
}

#[derive(Debug, Clone)]
struct AudioSource {
    source_id: AudioSourceId,
    source_name: String,
    #[allow(dead_code)]
    source_type: AudioSourceType,
}

impl AudioSource {
    fn new(source_id: AudioSourceId, source_name: String, source_type: AudioSourceType) -> Self {
        Self {
            source_id,
            source_name,
            source_type,
        }
    }
}

#[derive(Debug, Clone)]
struct AudioZone {
    zone_id: AudioZoneId,
    zone_name: String,
}

impl AudioZone {
    fn new(zone_id: AudioZoneId, zone_name: String) -> Self {
        Self { zone_id, zone_name }
    }
}

#[derive(Debug, Clone)]
struct AcousticProfile {
    profile_id: AcousticProfileId,
    profile_name: String,
}

impl AcousticProfile {
    fn new(profile_id: AcousticProfileId, profile_name: String) -> Self {
        Self {
            profile_id,
            profile_name,
        }
    }
}

#[derive(Debug, Clone)]
struct AudioRegistryState {
    sources: HashMap<AudioSourceId, AudioSource>,
    zones: HashMap<AudioZoneId, AudioZone>,
    profiles: HashMap<AcousticProfileId, AcousticProfile>,
}

impl AudioRegistryState {
    fn new() -> Self {
        Self {
            sources: HashMap::new(),
            zones: HashMap::new(),
            profiles: HashMap::new(),
        }
    }

    fn add_source(&mut self, source: AudioSource) {
        self.sources.insert(source.source_id.clone(), source);
    }

    fn add_zone(&mut self, zone: AudioZone) {
        self.zones.insert(zone.zone_id.clone(), zone);
    }

    fn add_profile(&mut self, profile: AcousticProfile) {
        self.profiles.insert(profile.profile_id.clone(), profile);
    }

    fn get_source(&self, source_id: &AudioSourceId) -> Option<&AudioSource> {
        self.sources.get(source_id)
    }

    fn get_zone(&self, zone_id: &AudioZoneId) -> Option<&AudioZone> {
        self.zones.get(zone_id)
    }

    fn get_profile(&self, profile_id: &AcousticProfileId) -> Option<&AcousticProfile> {
        self.profiles.get(profile_id)
    }
}

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

// Tests for l6.0-authority-core: Container lifecycle and operations

use stratumx_tooling_l6_0_authority_core::containers::{
    AudioAuthorityContainer, TerrainAuthorityContainer,
};
use stratumx_tooling_l6_0_authority_core::containers::audio::{
    AcousticProfile, AudioSource, AudioSourceType, AudioZone,
};

// ============================================================================
// Terrain Authority Container tests
// ============================================================================

#[test]
fn terrain_lifecycle_transitions() {
    let mut container = TerrainAuthorityContainer::new();
    assert_eq!(
        container.lifecycle_state(),
        stratumx_tooling_l6_0_authority_core::containers::terrain::TerrainLifecycleState::Uninitialized
    );

    container.initialize().expect("initialize should succeed");
    assert!(container.is_initialized());

    assert!(container.initialize().is_err());

    container.dispose().expect("dispose should succeed");
    assert!(container.dispose().is_err());
}

#[test]
fn terrain_layer_management() {
    let mut container = TerrainAuthorityContainer::new();
    container.initialize().unwrap();

    let layer = container.add_layer("Grass".to_string(), Some("mat_grass".to_string()));
    assert_eq!(layer.name, "Grass");

    let layers = container.query_layers();
    assert_eq!(layers.len(), 1);

    container.remove_layer(layer.id);
    assert_eq!(container.query_layers().len(), 0);
}

#[test]
fn terrain_brush_state() {
    let mut container = TerrainAuthorityContainer::new();
    container.initialize().unwrap();

    let brush = container.brush_state();
    assert_eq!(brush.radius, 0.0);

    let brush_mut = container.brush_state_mut();
    brush_mut.radius = 50.0;
    brush_mut.strength = 0.5;

    let brush = container.brush_state();
    assert_eq!(brush.radius, 50.0);
    assert_eq!(brush.strength, 0.5);
}

// ============================================================================
// Audio Authority Container tests
// ============================================================================

#[test]
fn audio_lifecycle_transitions() {
    let mut container = AudioAuthorityContainer::new();
    assert_eq!(
        container.lifecycle_state(),
        stratumx_tooling_l6_0_authority_core::containers::audio_authority_container::AudioLifecycleState::Uninitialized
    );

    container.initialize().expect("initialize should succeed");
    assert_eq!(
        container.lifecycle_state(),
        stratumx_tooling_l6_0_authority_core::containers::audio_authority_container::AudioLifecycleState::Initialized
    );
    assert!(container.is_initialized());

    assert!(container.initialize().is_err());

    container.dispose().expect("dispose should succeed");
    assert_eq!(
        container.lifecycle_state(),
        stratumx_tooling_l6_0_authority_core::containers::audio_authority_container::AudioLifecycleState::Disposed
    );

    assert!(container.dispose().is_err());
}

#[test]
fn audio_preview_connection() {
    let mut container = AudioAuthorityContainer::new();
    container.initialize().expect("initialize should succeed");

    let conn = container
        .establish_preview_connection("runtime_0".to_string())
        .unwrap();
    assert!(container.is_preview_active());
    assert_eq!(conn.runtime_handle, "runtime_0");

    container.disconnect_preview().unwrap();
    assert!(!container.is_preview_active());
}

#[test]
fn audio_query_sources() {
    let mut container = AudioAuthorityContainer::new();
    container.initialize().expect("initialize should succeed");
    assert_eq!(container.query_sources().len(), 0);
}

#[test]
fn audio_add_source() {
    let mut container = AudioAuthorityContainer::new();
    container.initialize().expect("initialize should succeed");

    let source = AudioSource {
        id: "src_0".to_string(),
        name: "TestSource".to_string(),
        source_type: AudioSourceType::Ambient,
        active: true,
    };
    container.add_source(source).unwrap();
    assert_eq!(container.query_sources().len(), 1);
}

#[test]
fn audio_zone_operations() {
    let mut container = AudioAuthorityContainer::new();
    container.initialize().expect("initialize should succeed");

    let zone = AudioZone {
        id: "zone_0".to_string(),
        name: "TestZone".to_string(),
        bounds: [0.0, 0.0, 0.0, 10.0, 10.0, 10.0],
        bound_sources: vec![],
    };
    container.add_zone(zone.clone()).unwrap();
    assert_eq!(container.query_zones().len(), 1);

    container.update_zone("zone_0", |z| {
        z.name = "UpdatedZone".to_string();
    }).unwrap();
    assert_eq!(container.query_zones()[0].name, "UpdatedZone");
}

#[test]
fn audio_profile_operations() {
    let mut container = AudioAuthorityContainer::new();
    container.initialize().expect("initialize should succeed");

    let profile = AcousticProfile {
        id: "prof_0".to_string(),
        name: "TestProfile".to_string(),
        absorption: 0.5,
        scattering: 0.3,
        reverb: 0.2,
        reverb_time: 1.5,
    };
    container.add_profile(profile).unwrap();
    assert_eq!(container.query_profiles().len(), 1);

    container.update_profile("prof_0", |p| {
        p.name = "UpdatedProfile".to_string();
    }).unwrap();
    assert_eq!(container.query_profiles()[0].name, "UpdatedProfile");

    container.remove_profile("prof_0").unwrap();
    assert_eq!(container.query_profiles().len(), 0);
}

#[test]
fn audio_dispose_disconnects_preview() {
    let mut container = AudioAuthorityContainer::new();
    container.initialize().expect("initialize should succeed");
    container
        .establish_preview_connection("runtime_0".to_string())
        .unwrap();
    assert!(container.is_preview_active());

    container.dispose().expect("dispose should succeed");
    assert!(!container.is_preview_active());
    assert!(container.preview_connection().is_none());
}

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use stratumx_editor_state_containers::{
    BasicEventBus, EnvironmentAuthoringService, EventBus, WeatherCondition, WorldIdentity,
    WorldState,
};
use uuid::Uuid;

fn create_service() -> (
    EnvironmentAuthoringService,
    Arc<Mutex<WorldState>>,
    Arc<AtomicUsize>,
) {
    let identity = WorldIdentity::new(
        Uuid::new_v4(),
        "Test World".to_string(),
        PathBuf::from("/test/world"),
    );
    let world_state = Arc::new(Mutex::new(WorldState::new(identity, String::new())));
    let event_bus = Arc::new(BasicEventBus::new());
    let event_count = Arc::new(AtomicUsize::new(0));
    for event_type in [
        "environment.sky_configured",
        "environment.weather_regime_bound",
        "environment.cloud_profile_bound",
    ] {
        let event_count_clone = event_count.clone();
        event_bus.subscribe(
            event_type,
            Box::new(move |_event| {
                event_count_clone.fetch_add(1, Ordering::SeqCst);
            }),
        );
    }

    (
        EnvironmentAuthoringService::new(world_state.clone(), event_bus),
        world_state,
        event_count,
    )
}

#[test]
fn environment_authoring_service_updates_truth_and_bindings() {
    let (mut service, world_state, _) = create_service();
    let sky_profile = Uuid::new_v4();
    service.configure_sky(&[0.5, 0.7, 1.0]).unwrap();
    service.set_weather(WeatherCondition::Rain).unwrap();
    service.update_lighting(0.3, 2).unwrap();
    service.bind_sky_profile(sky_profile).unwrap();

    let world = world_state.lock().unwrap();
    let environment = world.environment_state.as_ref().unwrap();
    assert_eq!(environment.weather, WeatherCondition::Rain);
    assert_eq!(service.get_sky_profile_binding(), Some(sky_profile));
}

#[test]
fn environment_authoring_service_emits_events_for_mutations() {
    let (mut service, _, event_count) = create_service();

    service.configure_sky(&[0.5, 0.7, 1.0]).unwrap();
    service.bind_weather_regime(Uuid::new_v4()).unwrap();
    service.bind_cloud_profile(Uuid::new_v4()).unwrap();

    assert_eq!(event_count.load(Ordering::SeqCst), 3);
}

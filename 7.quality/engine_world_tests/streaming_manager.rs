use engine_world::{world_pos_to_region_key, MemoryPressure, ResidencyState, StreamingManager};

const MB: usize = 1024 * 1024;

#[test]
fn streaming_manager_tracks_residency_and_pressure() {
    let mut manager = StreamingManager::new(20);
    let region_a = world_pos_to_region_key([10.0, 20.0, 30.0]);
    let region_b = world_pos_to_region_key([120.0, 0.0, -10.0]);

    manager.request_load(region_a, 1.0).unwrap();
    manager.complete_load(region_a, 9 * MB).unwrap();
    manager.request_load(region_b, 2.0).unwrap();
    manager.complete_load(region_b, 8 * MB).unwrap();

    assert_eq!(region_a, (0, 0, 0));
    assert_eq!(region_b, (1, 0, -1));
    assert_eq!(manager.resident_count(), 2);
    assert_eq!(manager.memory_pressure(), MemoryPressure::Elevated);
    assert!(manager.resident_regions().contains(&region_a));
    assert!(manager.resident_regions().contains(&region_b));
}

#[test]
fn request_load_evicts_existing_region_when_budget_is_exceeded() {
    let mut manager = StreamingManager::new(10);
    let first = (0, 0, 0);
    let second = (1, 0, 0);

    manager.request_load(first, 1.0).unwrap();
    manager.complete_load(first, 10 * MB).unwrap();
    manager.request_load(second, 2.0).unwrap();

    assert!(!manager.regions.contains_key(&first));
    assert_eq!(
        manager.regions.get(&second).unwrap().residency_state,
        ResidencyState::Loading
    );
}

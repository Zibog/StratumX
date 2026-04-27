//! Destruction Response: проверка системы разрушений

use engine_material::{DestructionResponse, FailureMode, StructureType};
use engine_world::WorldState;

#[test]
fn object_destruction() {
    let response = DestructionResponse::from_impact(
        StructureType::WoodenWall,
        [0.0, 0.0, 0.0],
        1_000.0,
        [1.0, 0.0, 0.0],
    );

    assert!(!response.destroyed);
    assert_eq!(response.failure_mode, FailureMode::None);
    assert!(response.fragments.is_empty());
}

#[test]
fn debris_generation() {
    let response = DestructionResponse::from_impact(
        StructureType::GlassWindow,
        [0.0, 1.0, 0.0],
        1_000.0,
        [0.0, 0.0, 1.0],
    );

    assert!(response.destroyed);
    assert_eq!(response.failure_mode, FailureMode::Shatter);
    assert_eq!(
        response.fragments.len(),
        StructureType::GlassWindow.fragment_count() as usize
    );
    assert!(response.dust_cloud_radius_m > 0.0);
}

#[test]
fn destruction_propagation() {
    let mut world = WorldState::new();
    let index = world.material_world_mut().apply_destruction(
        StructureType::Tree,
        [2.0, 0.0, 3.0],
        6_000.0,
        [0.0, 1.0, 0.0],
    );

    let response = world
        .material_world()
        .get_destruction_response(index)
        .expect("destruction response should be stored");

    assert!(response.destroyed);
    assert_eq!(response.structure_type, StructureType::Tree);
    assert_eq!(response.failure_mode, FailureMode::Topple);
}

use engine_world::{ReferenceRegionBootstrapper, ReferenceRegionScene, WorldState};

#[test]
fn bootstrap_populates_proof_region_state_and_material_indices() {
    let scene = ReferenceRegionScene::create_default();
    let mut world = WorldState::new();
    let bootstrapper = ReferenceRegionBootstrapper::new(scene.clone());

    let proof = bootstrapper.bootstrap(&mut world).unwrap();

    assert_eq!(proof.terrain_patches.len(), scene.terrain_patches.len());
    assert_eq!(proof.structures.len(), scene.structures.len());
    assert_eq!(proof.container_indices.len(), scene.containers.len());
    assert_eq!(proof.fire_indices.len(), scene.fire_sources.len());
    assert_eq!(proof.npcs.len(), scene.npcs.len());
    assert_eq!(proof.squads.len(), scene.squads.len());
    assert_eq!(proof.creatures.len(), scene.creatures.len());
    assert_eq!(proof.doors.len(), scene.doors.len());
    assert_eq!(proof.traders.len(), scene.traders.len());
    assert_eq!(proof.quests.len(), scene.quests.len());
    assert!(world
        .material_world()
        .get_hydrology_state(proof.container_indices[0])
        .is_some());
    assert!(world
        .material_world()
        .get_combustible_object(proof.fire_indices[0])
        .is_some());
}

#[test]
fn proof_region_routes_explosion_and_structure_destruction() {
    let scene = ReferenceRegionScene::create_default();
    let mut world = WorldState::new();
    let proof = ReferenceRegionBootstrapper::new(scene)
        .bootstrap(&mut world)
        .unwrap();

    let blast_index = proof.apply_explosion(&mut world, [5.0, 0.0, 5.0], 2_500.0);
    assert!(world
        .material_world()
        .get_terrain_response(blast_index)
        .is_some());

    let structure_index = proof
        .apply_structure_destruction(&mut world, 2, [30.0, 0.0, 30.0], 5_000.0, [1.0, 0.0, 0.0])
        .unwrap();
    assert!(world
        .material_world()
        .get_destruction_response(structure_index)
        .is_some());

    assert_eq!(
        proof.apply_structure_destruction(
            &mut world,
            3,
            [60.0, 0.0, 20.0],
            5_000.0,
            [1.0, 0.0, 0.0],
        ),
        None
    );
}

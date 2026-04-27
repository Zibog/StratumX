// Bootstrap - seed creation only

use engine_core::EngineCoreResult;
use engine_world::{StreamingManager, WorldState};

use super::bindings::{add_material_binding, initialize_damage_memory};
use super::environment_seed::{create_default_sky, get_default_sky_bundle_path};
use super::inventory_seed::{
    create_default_actor_inventory, create_default_container, create_default_door,
    create_default_navigation_path, ActorInventory, Container, DoorObject, NavigationPath,
};
use super::materials_seed::{create_material_registry, create_wall_material_stack};
use super::structures_seed::{create_camera, create_wall, create_weapon};
use super::terrain_seed::create_terrain_patch;
use super::world_identity::REFERENCE_SCENE_NAME;

/// Minimal seed runtime state - holds initial world configuration.
pub struct StartupReferenceSeedRuntime {
    pub world: WorldState,
    pub streaming_manager: StreamingManager,
    pub door: DoorObject,
    pub navigation_path: NavigationPath,
    pub actor_inventory: ActorInventory,
    pub container: Container,
}

/// Launch the startup reference seed - creates a minimal demo world.
pub fn launch_startup_reference_seed() -> EngineCoreResult<StartupReferenceSeedRuntime> {
    let mut materials = create_material_registry()?;
    let wall_stack_id = create_wall_material_stack(&mut materials)?;
    let world = create_demo_world(wall_stack_id, &materials)?;
    let streaming_manager = StreamingManager::new(512);

    Ok(StartupReferenceSeedRuntime {
        world,
        streaming_manager,
        door: create_default_door(),
        navigation_path: create_default_navigation_path(),
        actor_inventory: create_default_actor_inventory(),
        container: create_default_container(),
    })
}

fn create_demo_world(
    wall_stack_id: engine_material::MaterialStackId,
    materials: &engine_material::MaterialRegistry,
) -> EngineCoreResult<WorldState> {
    use engine_world::EntityId;

    let mut world = WorldState::new();

    let terrain_entity = EntityId(1);
    let wall_entity = EntityId(2);
    let weapon_entity = EntityId(3);
    let world_stack_id = engine_world::MaterialStackId(wall_stack_id.0);

    let scene = engine_world::VerticalSliceScene {
        scene_name: REFERENCE_SCENE_NAME.into(),
        terrain: create_terrain_patch(terrain_entity),
        wall: create_wall(wall_entity, world_stack_id),
        weapon: create_weapon(weapon_entity),
        camera: create_camera(),
        sky: create_default_sky(),
        sky_bundle_path: get_default_sky_bundle_path(),
    };

    world.set_vertical_slice_scene(scene);
    add_material_binding(&mut world, wall_entity, world_stack_id);
    initialize_damage_memory(&mut world, wall_stack_id, materials)?;

    Ok(world)
}

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
use super::world_identity::PROOF_REGION_SCENE_NAME;

/// Minimal seed runtime state - holds initial world configuration.
pub struct StartupReferenceSeedRuntime {
    pub world: WorldState,
    pub streaming_manager: StreamingManager,
    pub door: DoorObject,
    pub navigation_path: NavigationPath,
    pub actor_inventory: ActorInventory,
    pub container: Container,
    pub ballistic_simulator: engine_kinetics::BallisticSimulator,
}

/// Launch the startup reference seed and build a minimal proof region world.
pub fn launch_startup_reference_seed() -> EngineCoreResult<StartupReferenceSeedRuntime> {
    let mut materials = create_material_registry()?;
    let wall_stack_id = create_wall_material_stack(&mut materials)?;
    let world = create_proof_region(wall_stack_id, &materials)?;
    let streaming_manager = StreamingManager::new(512);
    let ballistic_simulator = create_ballistic_simulator();

    Ok(StartupReferenceSeedRuntime {
        world,
        streaming_manager,
        door: create_default_door(),
        navigation_path: create_default_navigation_path(),
        actor_inventory: create_default_actor_inventory(),
        container: create_default_container(),
        ballistic_simulator,
    })
}

fn create_proof_region(
    wall_stack_id: engine_material::MaterialStackId,
    materials: &engine_material::MaterialRegistry,
) -> EngineCoreResult<WorldState> {
    use engine_world::EntityId;

    let mut world = WorldState::new();

    let terrain_entity = EntityId(1);
    let wall_entity = EntityId(2);
    let weapon_entity = EntityId(3);
    let world_stack_id = engine_world::MaterialStackId(wall_stack_id.0);

    let scene = engine_world::ProofRegionScene {
        scene_name: PROOF_REGION_SCENE_NAME.into(),
        terrain: create_terrain_patch(terrain_entity),
        wall: create_wall(wall_entity, world_stack_id),
        weapon: create_weapon(weapon_entity),
        camera: create_camera(),
        sky: create_default_sky(),
        sky_bundle_path: get_default_sky_bundle_path(),
    };

    world.set_proof_region_scene(scene);
    add_material_binding(&mut world, wall_entity, world_stack_id);
    initialize_damage_memory(&mut world, wall_stack_id, materials)?;

    Ok(world)
}

fn create_ballistic_simulator() -> engine_kinetics::BallisticSimulator {
    use engine_kinetics::{ProjectileProfile, ProjectileProfileId, WeaponProfile, WeaponProfileId};

    let mut simulator = engine_kinetics::BallisticSimulator::new();

    let projectile_profile = ProjectileProfile {
        id: ProjectileProfileId(1),
        label: "7.62x51mm NATO".to_string(),
        mass_kg: 0.0096,
        diameter_mm: 7.62,
        muzzle_velocity_m_s: 840.0,
        drag_coefficient: 0.295,
    };

    let weapon_profile = WeaponProfile {
        id: WeaponProfileId(1),
        label: "Test Rifle".to_string(),
        projectile_profile: ProjectileProfileId(1),
        fire_rate_rpm: 600.0,
    };

    simulator.register_projectile(projectile_profile);
    simulator.register_weapon(weapon_profile);

    simulator
}

impl StartupReferenceSeedRuntime {
    pub fn open_door(&mut self) -> Result<(), String> {
        self.door.open()
    }

    pub fn close_door(&mut self) -> Result<(), String> {
        self.door.close()
    }

    pub fn set_door_blocked(&mut self, reason: String) {
        self.door.set_blocked(reason);
    }

    pub fn set_door_locked(&mut self) {
        self.door.set_locked();
    }

    pub fn get_door_state(&self) -> (super::inventory_seed::DoorState, Option<String>) {
        (self.door.state, self.door.blocked_reason.clone())
    }

    pub fn set_navigation_path(&mut self, start: [f32; 3], destination: [f32; 3]) {
        self.navigation_path.start = start;
        self.navigation_path.destination = destination;
        self.navigation_path.update_from_door(&self.door);
    }

    pub fn navigation_status(&self) -> (String, Option<String>) {
        use super::inventory_seed::DoorState;
        match self.door.state {
            DoorState::Open => ("Valid".to_string(), None),
            DoorState::Blocked => ("Blocked".to_string(), self.door.blocked_reason.clone()),
            DoorState::Locked => (
                "Blocked".to_string(),
                self.door
                    .blocked_reason
                    .clone()
                    .or_else(|| Some("Door is locked".to_string())),
            ),
            DoorState::Closed => ("Blocked".to_string(), Some("Door is closed".to_string())),
        }
    }
}

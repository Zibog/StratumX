// Structures seed - wall, weapon, camera defaults

use engine_world::{CameraState, EntityId, WallState, WeaponState};

pub fn create_wall(
    wall_entity: EntityId,
    world_stack_id: engine_world::MaterialStackId,
) -> WallState {
    WallState {
        entity_id: wall_entity,
        position: [0.0, 1.5, 10.0],
        dimensions: [4.0, 3.0, 0.25],
        stack_id: world_stack_id,
    }
}

pub fn create_weapon(weapon_entity: EntityId) -> WeaponState {
    WeaponState {
        entity_id: weapon_entity,
        profile_id: engine_world::WeaponProfileId(1),
        position: [0.0, 1.5, 0.0],
        aim_direction: [0.0, 0.0, 1.0],
    }
}

pub fn create_camera() -> CameraState {
    CameraState {
        position: [-2.0, 2.0, 5.0],
        look_at: [0.0, 1.5, 10.0],
        fov_deg: 75.0,
    }
}

// Bindings - связывание компонентов (entity-material bindings, damage memory)

use engine_core::EngineCoreResult;
use engine_material::{MaterialRegistry, MaterialStackId};
use engine_world::{
    EntityDamageMemory, EntityId, EntityMaterialBinding, LayerDamageState, WorldState,
};

pub fn initialize_damage_memory(
    world: &mut WorldState,
    wall_stack_id: MaterialStackId,
    materials: &MaterialRegistry,
) -> EngineCoreResult<()> {
    let wall_entity = EntityId(2);
    let world_stack_id = engine_world::MaterialStackId(wall_stack_id.0);

    let stack =
        materials
            .stack(wall_stack_id)
            .ok_or(engine_core::EngineCoreError::InvalidDescriptor(
                "wall stack not found",
            ))?;

    let layer_damage: Vec<LayerDamageState> = stack
        .layers
        .iter()
        .enumerate()
        .map(|(idx, _)| LayerDamageState {
            layer_index: idx as u8,
            integrity: 1.0,
            accumulated_energy_j: 0.0,
            cracked_segments: Vec::new(),
            released_segments: Vec::new(),
        })
        .collect();

    world.add_damage_memory(EntityDamageMemory {
        entity_id: wall_entity,
        stack_id: world_stack_id,
        layer_damage,
    });

    Ok(())
}

pub fn add_material_binding(
    world: &mut WorldState,
    wall_entity: EntityId,
    world_stack_id: engine_world::MaterialStackId,
) {
    world.add_material_binding(EntityMaterialBinding {
        entity_id: wall_entity,
        stack_id: world_stack_id,
    });
}

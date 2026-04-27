use crate::{ReferenceRegionScene, ReferenceRegionState, WorldState};
use engine_core::EngineCoreResult;

pub struct ReferenceRegionBootstrapper {
    scene: ReferenceRegionScene,
}

impl ReferenceRegionBootstrapper {
    pub fn new(scene: ReferenceRegionScene) -> Self {
        Self { scene }
    }

    pub fn bootstrap(&self, world: &mut WorldState) -> EngineCoreResult<ReferenceRegionState> {
        let mut state = ReferenceRegionState::default();

        for patch in &self.scene.terrain_patches {
            state.terrain_patches.push(patch.clone());
        }

        for structure in &self.scene.structures {
            state.structures.push(structure.clone());
        }

        for container in &self.scene.containers {
            let container_index = world.material_world_mut().create_hydrology_container(
                container.position,
                container.capacity_liters,
                container.height_m,
                container.cross_section_m2,
            );

            if container.has_leak {
                world.material_world_mut().add_leak_to_container(
                    container_index,
                    container.leak_position,
                    container.leak_diameter_mm,
                );
            }

            world.material_world_mut().set_rainfall_on_container(
                container_index,
                self.scene.weather_config.rainfall_intensity,
            );

            state.container_indices.push(container_index);
        }

        for fire_source in &self.scene.fire_sources {
            let fire_index = world
                .material_world_mut()
                .create_combustible_object(fire_source.position, fire_source.material);

            if fire_source.ignited {
                for _ in 0..50 {
                    world
                        .material_world_mut()
                        .apply_heat_to_object(fire_index, 300.0, 0.0);
                }
            }

            state.fire_indices.push(fire_index);
        }

        for npc in &self.scene.npcs {
            state.npcs.push(npc.clone());
        }

        for squad in &self.scene.squads {
            state.squads.push(squad.clone());
        }

        for creature in &self.scene.creatures {
            state.creatures.push(creature.clone());
        }

        for door in &self.scene.doors {
            state.doors.push(door.clone());
        }

        for trader in &self.scene.traders {
            state.traders.push(trader.clone());
        }

        for quest in &self.scene.quests {
            state.quests.push(quest.clone());
        }

        Ok(state)
    }
}

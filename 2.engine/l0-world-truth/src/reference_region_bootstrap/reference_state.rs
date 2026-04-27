use crate::{StructureType, WorldState};

#[derive(Debug, Clone, Default)]
pub struct ReferenceRegionState {
    pub terrain_patches: Vec<crate::TerrainPatch>,
    pub structures: Vec<crate::Structure>,
    pub container_indices: Vec<usize>,
    pub fire_indices: Vec<usize>,
    pub npcs: Vec<crate::NpcSpawn>,
    pub squads: Vec<crate::SquadSpawn>,
    pub creatures: Vec<crate::CreatureSpawn>,
    pub doors: Vec<crate::Door>,
    pub traders: Vec<crate::Trader>,
    pub quests: Vec<crate::Quest>,
}

impl ReferenceRegionState {
    pub fn apply_explosion(
        &self,
        world: &mut WorldState,
        position: [f32; 3],
        energy_j: f32,
    ) -> usize {
        let terrain_type = self.get_terrain_type_at(position);
        world
            .material_world_mut()
            .apply_terrain_blast(position, energy_j, terrain_type)
    }

    pub fn apply_structure_destruction(
        &self,
        world: &mut WorldState,
        structure_id: u64,
        impact_position: [f32; 3],
        impact_energy_j: f32,
        impact_direction: [f32; 3],
    ) -> Option<usize> {
        let structure = self.structures.iter().find(|s| s.id == structure_id)?;
        if !structure.destructible {
            return None;
        }

        let structure_type = match structure.structure_type {
            StructureType::Tree => engine_material::StructureType::Tree,
            StructureType::WoodenBuilding => engine_material::StructureType::WoodenWall,
            StructureType::BrickWall => engine_material::StructureType::BrickWall,
            StructureType::Tunnel => return None,
        };

        let index = world.material_world_mut().apply_destruction(
            structure_type,
            impact_position,
            impact_energy_j,
            impact_direction,
        );

        Some(index)
    }

    fn get_terrain_type_at(&self, position: [f32; 3]) -> engine_material::TerrainMaterialType {
        for patch in &self.terrain_patches {
            let in_x = position[0] >= patch.position[0]
                && position[0] <= patch.position[0] + patch.size[0];
            let in_z = position[2] >= patch.position[2]
                && position[2] <= patch.position[2] + patch.size[1];
            if in_x && in_z {
                return patch.material_type;
            }
        }
        engine_material::TerrainMaterialType::Dirt
    }
}

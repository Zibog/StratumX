use super::actors::{CreatureSpawn, NpcSpawn, SquadSpawn};
use super::gameplay::{Quest, Trader};
use super::types::{Container, Door, FireSource, Structure, TerrainPatch, WeatherConfig};
use engine_material::CombustibleMaterial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceRegionScene {
    pub region_id: u64,
    pub name: String,
    pub terrain_patches: Vec<TerrainPatch>,
    pub structures: Vec<Structure>,
    pub containers: Vec<Container>,
    pub fire_sources: Vec<FireSource>,
    pub npcs: Vec<NpcSpawn>,
    pub squads: Vec<SquadSpawn>,
    pub creatures: Vec<CreatureSpawn>,
    pub doors: Vec<Door>,
    pub traders: Vec<Trader>,
    pub quests: Vec<Quest>,
    pub weather_config: WeatherConfig,
}

impl ReferenceRegionScene {
    pub fn create_default() -> Self {
        use super::gameplay::InventoryItem;
        use super::types::StructureType;

        Self {
            region_id: 1,
            name: "Reference Region".to_string(),
            terrain_patches: vec![
                TerrainPatch {
                    position: [0.0, 0.0, 0.0],
                    size: [50.0, 50.0],
                    material_type: engine_material::TerrainMaterialType::Dirt,
                },
                TerrainPatch {
                    position: [50.0, 0.0, 0.0],
                    size: [20.0, 50.0],
                    material_type: engine_material::TerrainMaterialType::Asphalt,
                },
            ],
            structures: vec![
                Structure {
                    id: 1,
                    structure_type: StructureType::Tree,
                    position: [10.0, 0.0, 10.0],
                    rotation: 0.0,
                    destructible: true,
                },
                Structure {
                    id: 2,
                    structure_type: StructureType::WoodenBuilding,
                    position: [30.0, 0.0, 30.0],
                    rotation: 0.0,
                    destructible: true,
                },
                Structure {
                    id: 3,
                    structure_type: StructureType::Tunnel,
                    position: [60.0, 0.0, 20.0],
                    rotation: 0.0,
                    destructible: false,
                },
            ],
            containers: vec![Container {
                id: 1,
                position: [20.0, 0.0, 20.0],
                capacity_liters: 200.0,
                height_m: 1.0,
                cross_section_m2: 0.5,
                has_leak: true,
                leak_position: [20.0, 0.3, 20.0],
                leak_diameter_mm: 5.0,
            }],
            fire_sources: vec![FireSource {
                id: 1,
                position: [15.0, 0.0, 15.0],
                material: CombustibleMaterial::Wood,
                ignited: true,
            }],
            npcs: vec![
                NpcSpawn {
                    id: 1,
                    position: [12.0, 0.0, 12.0],
                    camp_id: Some(1),
                    aggression: 0.3,
                    needs_hunger: 0.5,
                    needs_rest: 0.3,
                },
                NpcSpawn {
                    id: 2,
                    position: [14.0, 0.0, 14.0],
                    camp_id: Some(1),
                    aggression: 0.2,
                    needs_hunger: 0.4,
                    needs_rest: 0.5,
                },
                NpcSpawn {
                    id: 3,
                    position: [16.0, 0.0, 12.0],
                    camp_id: Some(1),
                    aggression: 0.4,
                    needs_hunger: 0.6,
                    needs_rest: 0.2,
                },
            ],
            squads: vec![SquadSpawn {
                id: 1,
                leader_position: [40.0, 0.0, 40.0],
                member_count: 3,
                hostile: true,
            }],
            creatures: vec![CreatureSpawn {
                id: 1,
                species: "deer".to_string(),
                position: [5.0, 0.0, 5.0],
                migration_target: [45.0, 0.0, 45.0],
            }],
            doors: vec![Door {
                id: 1,
                position: [35.0, 0.0, 35.0],
                handle_position: [35.5, 1.0, 35.0],
                opened: false,
            }],
            traders: vec![Trader {
                id: 1,
                position: [25.0, 0.0, 25.0],
                inventory_items: vec![
                    InventoryItem {
                        item_id: 1,
                        quantity: 10,
                        price: 50,
                    },
                    InventoryItem {
                        item_id: 2,
                        quantity: 5,
                        price: 100,
                    },
                ],
            }],
            quests: vec![Quest {
                id: 1,
                name: "Investigate Camp".to_string(),
                trigger_position: [15.0, 0.0, 15.0],
                trigger_radius: 5.0,
                completed: false,
            }],
            weather_config: WeatherConfig {
                storm_center: [25.0, 25.0],
                storm_radius_km: 10.0,
                storm_intensity: 0.7,
                storm_velocity: [5.0, 2.0],
                rainfall_intensity: 20.0,
            },
        }
    }
}

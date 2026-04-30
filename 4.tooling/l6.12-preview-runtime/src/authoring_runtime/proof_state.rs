use std::collections::BTreeMap;

use link_egress_observations::StormFrontDto;
use serde::{Deserialize, Serialize};

const REGION_SIZE_METERS: f32 = 100.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemState {
    pub id: u32,
    pub name: String,
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialWorldState {
    pub barrel_water_liters: f32,
    pub barrel_leak_active: bool,
    pub fire_object_burning: bool,
    pub fire_object_wetness_percent: f32,
    pub fire_object_fuel_remaining_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlastResponseState {
    pub crater_radius_m: f32,
    pub crater_depth_m: f32,
    pub debris_count: u32,
    pub ejecta_volume_m3: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportObjectState {
    pub structure_type: String,
    pub integrity: f32,
    pub destroyed: bool,
    pub failure_mode: String,
    pub fragment_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestructionState {
    pub terrain_material: String,
    pub last_blast: Option<BlastResponseState>,
    pub wall_integrity: f32,
    pub support: SupportObjectState,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DoorStateKind {
    Closed,
    Open,
    Blocked,
    Locked,
}

impl DoorStateKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "Closed",
            Self::Open => "Open",
            Self::Blocked => "Blocked",
            Self::Locked => "Locked",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegionResidencyState {
    Loading,
    Resident,
    Evicting,
}

impl RegionResidencyState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "Loading",
            Self::Resident => "Resident",
            Self::Evicting => "Evicting",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionResidencyRecord {
    pub state: RegionResidencyState,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoorInventoryState {
    pub door_state: DoorStateKind,
    pub blocked_reason: Option<String>,
    pub navigation_path: Option<([f32; 3], [f32; 3])>,
    pub inventory_items: Vec<InventoryItemState>,
    pub equipped_weapon: Option<u32>,
    pub container_items: Vec<InventoryItemState>,
    pub region_residency: BTreeMap<(i32, i32, i32), RegionResidencyRecord>,
    pub budget_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcProfileState {
    pub npc_id: u32,
    pub name: String,
    pub position: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityScheduleState {
    pub activity: String,
    pub start_time: f32,
    pub duration: f32,
    pub location: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationState {
    pub npc_profile: Option<NpcProfileState>,
    pub aggression: f32,
    pub greed: f32,
    pub loyalty: f32,
    pub courage: f32,
    pub discipline: f32,
    pub sociability: f32,
    pub needs: BTreeMap<String, f32>,
    pub schedule: Option<ActivityScheduleState>,
    pub scarcity_factor: f32,
    pub is_criminal: bool,
    pub reputation: f32,
    pub wanted_level: u8,
    pub faction_info: Option<(u32, f32, u8)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquadState {
    pub squad_id: u32,
    pub member_ids: Vec<u32>,
    pub roles: Vec<String>,
    pub cover_positions: BTreeMap<u32, [f32; 3]>,
    pub tactic_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticsState {
    pub squad: Option<SquadState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatureState {
    pub creature_id: u32,
    pub species: String,
    pub position: [f32; 3],
    pub hunger: f32,
    pub fear: f32,
    pub migrating: bool,
    pub migration_target: Option<[f32; 3]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcologyState {
    pub creature: Option<CreatureState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkyState {
    pub time_of_day_hours: f32,
    pub day_of_year: u16,
    pub latitude_deg: f32,
    pub sun_elevation_deg: f32,
    pub sun_intensity: f32,
    pub cloud_coverage: f32,
    pub fog_density: f32,
    pub rain_enabled: bool,
    pub rain_intensity_mm_per_hour: f32,
    pub wind_vector: [f32; 3],
    pub storm_fronts: Vec<StormFrontDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorProofState {
    pub material_world: MaterialWorldState,
    pub destruction: DestructionState,
    pub door_inventory: DoorInventoryState,
    pub population: PopulationState,
    pub tactics: TacticsState,
    pub ecology: EcologyState,
    pub sky: SkyState,
    pub captured_sky_baseline: Option<SkyState>,
    pub next_storm_front_id: u32,
    pub simulation_time: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullWorldStateSnapshot {
    pub world_version: u32,
    pub save_timestamp: f64,
    pub simulation_time: f32,
    pub proof_state: EditorProofState,
}

impl Default for EditorProofState {
    fn default() -> Self {
        Self {
            material_world: MaterialWorldState {
                barrel_water_liters: 0.0,
                barrel_leak_active: false,
                fire_object_burning: false,
                fire_object_wetness_percent: 0.0,
                fire_object_fuel_remaining_percent: 100.0,
            },
            destruction: DestructionState {
                terrain_material: "Dirt".to_string(),
                last_blast: None,
                wall_integrity: 1.0,
                support: SupportObjectState {
                    structure_type: "ConcretePillar".to_string(),
                    integrity: 1.0,
                    destroyed: false,
                    failure_mode: "none".to_string(),
                    fragment_count: 0,
                },
            },
            door_inventory: DoorInventoryState {
                door_state: DoorStateKind::Closed,
                blocked_reason: None,
                navigation_path: None,
                inventory_items: vec![],
                equipped_weapon: None,
                container_items: vec![InventoryItemState {
                    id: 1,
                    name: "AK-47".to_string(),
                    item_type: "Weapon".to_string(),
                }],
                region_residency: BTreeMap::new(),
                budget_bytes: 512 * 1024 * 1024,
            },
            population: PopulationState {
                npc_profile: None,
                aggression: 0.5,
                greed: 0.5,
                loyalty: 0.5,
                courage: 0.5,
                discipline: 0.5,
                sociability: 0.5,
                needs: BTreeMap::new(),
                schedule: None,
                scarcity_factor: 0.0,
                is_criminal: false,
                reputation: 0.0,
                wanted_level: 0,
                faction_info: None,
            },
            tactics: TacticsState { squad: None },
            ecology: EcologyState { creature: None },
            sky: default_sky_state(),
            captured_sky_baseline: None,
            next_storm_front_id: 1,
            simulation_time: 0.0,
        }
    }
}

impl EditorProofState {
    pub fn reset_sky_to_default(&mut self) {
        self.sky = default_sky_state();
    }

    pub fn capture_sky_baseline(&mut self) {
        self.captured_sky_baseline = Some(self.sky.clone());
    }

    pub fn restore_sky_baseline(&mut self) -> Result<(), String> {
        let baseline = self
            .captured_sky_baseline
            .clone()
            .ok_or_else(|| "No sky baseline has been captured".to_string())?;
        self.sky = baseline;
        Ok(())
    }

    pub fn step_sky_simulation(&mut self, dt_seconds: f32) {
        self.simulation_time += dt_seconds.max(0.0);
        self.sky.time_of_day_hours = wrap_hours(self.sky.time_of_day_hours + dt_seconds / 3600.0);
        self.sky.sun_elevation_deg =
            ((self.sky.time_of_day_hours - 12.0) / 12.0 * 90.0).clamp(-90.0, 90.0);
        for front in &mut self.sky.storm_fronts {
            front.position[0] += front.velocity[0] * dt_seconds;
            front.position[1] += front.velocity[1] * dt_seconds;
            front.position[2] += front.velocity[2] * dt_seconds;
        }
    }

    pub fn create_storm_front(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius_km: f32,
        intensity: f32,
        rain_intensity_mm_per_hour: f32,
    ) -> StormFrontDto {
        let front = StormFrontDto {
            front_id: self.next_storm_front_id,
            position,
            velocity,
            radius_km,
            intensity,
            rain_intensity_mm_per_hour,
        };
        self.next_storm_front_id += 1;
        self.sky.storm_fronts.push(front.clone());
        front
    }

    pub fn update_storm_front(
        &mut self,
        front_id: u32,
        position: Option<[f32; 3]>,
        velocity: Option<[f32; 3]>,
        radius_km: Option<f32>,
        intensity: Option<f32>,
        rain_intensity_mm_per_hour: Option<f32>,
    ) -> bool {
        let Some(front) = self
            .sky
            .storm_fronts
            .iter_mut()
            .find(|front| front.front_id == front_id)
        else {
            return false;
        };

        if let Some(position) = position {
            front.position = position;
        }
        if let Some(velocity) = velocity {
            front.velocity = velocity;
        }
        if let Some(radius_km) = radius_km {
            front.radius_km = radius_km;
        }
        if let Some(intensity) = intensity {
            front.intensity = intensity;
        }
        if let Some(rain_intensity_mm_per_hour) = rain_intensity_mm_per_hour {
            front.rain_intensity_mm_per_hour = rain_intensity_mm_per_hour;
        }

        true
    }

    pub fn update_material_world(&mut self, delta_time: f32) {
        let dt = delta_time.max(0.0);
        self.simulation_time += dt;

        if self.sky.rain_enabled {
            self.material_world.barrel_water_liters +=
                self.sky.rain_intensity_mm_per_hour * dt * 0.01;
            self.material_world.fire_object_wetness_percent = clamp_percent(
                self.material_world.fire_object_wetness_percent
                    + self.sky.rain_intensity_mm_per_hour * dt * 0.05,
            );
        }

        if self.material_world.barrel_leak_active {
            self.material_world.barrel_water_liters =
                (self.material_world.barrel_water_liters - dt).max(0.0);
        } else if !self.sky.rain_enabled {
            self.material_world.barrel_water_liters =
                (self.material_world.barrel_water_liters - dt * 0.01).max(0.0);
        }

        if self.material_world.fire_object_burning {
            self.material_world.fire_object_fuel_remaining_percent = clamp_percent(
                self.material_world.fire_object_fuel_remaining_percent - dt * 2.0,
            );
            if self.material_world.fire_object_fuel_remaining_percent <= 0.0 {
                self.material_world.fire_object_burning = false;
            }
        }

        if self.material_world.fire_object_wetness_percent > 50.0 {
            self.material_world.fire_object_burning = false;
        }
    }

    pub fn current_smoke_particle_count(&self) -> usize {
        if !self.material_world.fire_object_burning {
            return 0;
        }

        let wind_magnitude = vector_magnitude(self.sky.wind_vector);
        (25.0 + wind_magnitude * 5.0).round() as usize
    }

    pub fn ignite_fire_object(&mut self) -> bool {
        let success = self.material_world.fire_object_wetness_percent < 50.0
            && self.material_world.fire_object_fuel_remaining_percent > 0.0;
        self.material_world.fire_object_burning = success;
        success
    }

    pub fn trigger_blast(&mut self, position: [f32; 3], energy_j: f32) {
        let blast = derive_blast_response(
            &self.destruction.terrain_material,
            self.sky.rain_enabled,
            self.sky.rain_intensity_mm_per_hour,
            energy_j,
        );
        self.destruction.last_blast = Some(blast);

        let wall_damage = proximity_damage(position, [0.0, 1.5, 10.0], energy_j, 6.0, 180000.0);
        self.destruction.wall_integrity =
            (self.destruction.wall_integrity - wall_damage).max(0.0);

        let support_energy =
            proximity_damage(position, [3.0, 0.0, 8.0], energy_j, 6.0, 1.0) * energy_j;
        if support_energy > 0.0 {
            self.apply_support_damage(support_energy);
        }
    }

    pub fn set_wall_integrity(&mut self, integrity: f32) {
        self.destruction.wall_integrity = integrity.clamp(0.0, 1.0);
    }

    pub fn wall_destroyed(&self) -> bool {
        self.destruction.wall_integrity <= 0.0
    }

    pub fn set_support_object_type(&mut self, structure_type: String) {
        self.destruction.support.structure_type = structure_type;
        self.destruction.support.integrity = 1.0;
        self.destruction.support.destroyed = false;
        self.destruction.support.failure_mode = "none".to_string();
        self.destruction.support.fragment_count = 0;
    }

    pub fn apply_support_damage(&mut self, energy_j: f32) {
        let threshold = match self.destruction.support.structure_type.as_str() {
            "WoodenWall" => 7000.0,
            "Tree" => 5000.0,
            _ => 20000.0,
        };
        let integrity_loss = (energy_j / threshold).min(1.0);
        self.destruction.support.integrity =
            (self.destruction.support.integrity - integrity_loss).max(0.0);

        if self.destruction.support.integrity <= 0.0 {
            self.destruction.support.destroyed = true;
            self.destruction.support.fragment_count =
                if self.destruction.support.structure_type == "Tree" {
                    6
                } else {
                    4
                };
            self.destruction.support.failure_mode =
                if self.destruction.support.structure_type == "Tree" {
                    "Topple".to_string()
                } else {
                    "Fracture".to_string()
                };
        } else if self.destruction.support.integrity < 1.0 {
            self.destruction.support.destroyed = false;
            self.destruction.support.fragment_count = 0;
            self.destruction.support.failure_mode = "Crack".to_string();
        } else {
            self.destruction.support.destroyed = false;
            self.destruction.support.fragment_count = 0;
            self.destruction.support.failure_mode = "none".to_string();
        }
    }

    pub fn reset_destruction_state(&mut self) {
        self.destruction = EditorProofState::default().destruction;
    }

    pub fn open_door(&mut self) {
        self.door_inventory.door_state = DoorStateKind::Open;
        self.door_inventory.blocked_reason = None;
    }

    pub fn close_door(&mut self) {
        self.door_inventory.door_state = DoorStateKind::Closed;
        self.door_inventory.blocked_reason = None;
    }

    pub fn block_door(&mut self, reason: String) {
        self.door_inventory.door_state = DoorStateKind::Blocked;
        self.door_inventory.blocked_reason = Some(reason);
    }

    pub fn lock_door(&mut self) {
        self.door_inventory.door_state = DoorStateKind::Locked;
        self.door_inventory.blocked_reason = Some("Door is locked".to_string());
    }

    pub fn navigation_status(&self) -> (String, Option<String>) {
        match self.door_inventory.door_state {
            DoorStateKind::Open => ("Valid".to_string(), None),
            DoorStateKind::Blocked => (
                "Blocked".to_string(),
                self.door_inventory.blocked_reason.clone(),
            ),
            DoorStateKind::Locked => (
                "Blocked".to_string(),
                self.door_inventory
                    .blocked_reason
                    .clone()
                    .or_else(|| Some("Door is locked".to_string())),
            ),
            DoorStateKind::Closed => (
                "Blocked".to_string(),
                Some("Door is closed".to_string()),
            ),
        }
    }

    pub fn current_memory_bytes(&self) -> usize {
        self.door_inventory
            .region_residency
            .values()
            .filter(|record| record.state == RegionResidencyState::Resident)
            .map(|record| record.size_bytes)
            .sum()
    }

    pub fn memory_pressure(&self) -> &'static str {
        let usage = self.current_memory_bytes() as f32 / self.door_inventory.budget_bytes as f32;
        if usage < 0.5 {
            "Healthy"
        } else if usage < 1.0 {
            "Elevated"
        } else {
            "Critical"
        }
    }

    pub fn resident_regions(&self) -> Vec<(i32, i32, i32)> {
        self.door_inventory
            .region_residency
            .iter()
            .filter_map(|(key, record)| {
                (record.state == RegionResidencyState::Resident).then_some(*key)
            })
            .collect()
    }

    pub fn save_snapshot(&self) -> FullWorldStateSnapshot {
        FullWorldStateSnapshot {
            world_version: 1,
            save_timestamp: current_timestamp_seconds(),
            simulation_time: self.simulation_time,
            proof_state: self.clone(),
        }
    }

    pub fn count_domains(&self) -> usize {
        let mut count = 4;
        if self.population.npc_profile.is_some() {
            count += 1;
        }
        if self.tactics.squad.is_some() {
            count += 1;
        }
        if self.ecology.creature.is_some() {
            count += 1;
        }
        count
    }
}

pub fn default_sky_state() -> SkyState {
    SkyState {
        time_of_day_hours: 12.0,
        day_of_year: 172,
        latitude_deg: 45.0,
        sun_elevation_deg: 60.0,
        sun_intensity: 1.0,
        cloud_coverage: 0.3,
        fog_density: 0.0,
        rain_enabled: false,
        rain_intensity_mm_per_hour: 0.0,
        wind_vector: [0.0, 0.0, 0.0],
        storm_fronts: vec![],
    }
}

pub fn world_pos_to_region(position: [f32; 3]) -> (i32, i32, i32) {
    (
        (position[0] / REGION_SIZE_METERS).floor() as i32,
        (position[1] / REGION_SIZE_METERS).floor() as i32,
        (position[2] / REGION_SIZE_METERS).floor() as i32,
    )
}

fn derive_blast_response(
    terrain_material: &str,
    rain_enabled: bool,
    rain_intensity_mm_per_hour: f32,
    energy_j: f32,
) -> BlastResponseState {
    let energy_scale = (energy_j / 50_000.0).max(0.2);
    let (base_radius, base_depth) = match terrain_material {
        "Asphalt" => (1.4, 0.25),
        "Dirt" => (2.4, 0.8),
        _ => (1.9, 0.5),
    };
    let rain_softening = if rain_enabled {
        1.0 + rain_intensity_mm_per_hour.min(100.0) / 500.0
    } else {
        1.0
    };

    BlastResponseState {
        crater_radius_m: base_radius * energy_scale.sqrt() * rain_softening,
        crater_depth_m: base_depth * energy_scale * rain_softening,
        debris_count: (8.0 + energy_scale * 6.0).round() as u32,
        ejecta_volume_m3: base_radius * base_depth * energy_scale * rain_softening,
    }
}

fn proximity_damage(
    blast_position: [f32; 3],
    target_position: [f32; 3],
    energy_j: f32,
    effective_radius_m: f32,
    divisor: f32,
) -> f32 {
    let distance = distance(blast_position, target_position);
    if distance >= effective_radius_m {
        return 0.0;
    }

    let proximity = 1.0 - distance / effective_radius_m;
    ((energy_j / divisor) * proximity).clamp(0.0, 1.0)
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn vector_magnitude(vector: [f32; 3]) -> f32 {
    (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt()
}

fn clamp_percent(value: f32) -> f32 {
    value.clamp(0.0, 100.0)
}

fn wrap_hours(value: f32) -> f32 {
    let wrapped = value % 24.0;
    if wrapped < 0.0 {
        wrapped + 24.0
    } else {
        wrapped
    }
}

fn current_timestamp_seconds() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

use serde::{Deserialize, Serialize};

// ============================================================================
// SCENE DTO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneDto {
    pub scene_name: String,
    pub terrain: TerrainPatchDto,
    pub wall: WallDto,
    pub weapon: WeaponDto,
    pub camera: CameraDto,
    pub sky_bundle_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainPatchDto {
    pub entity_id: u32,
    pub origin: [f32; 3],
    pub world_size: [f32; 2],
    pub resolution: [u32; 2],
    pub mesh_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WallDto {
    pub entity_id: u32,
    pub position: [f32; 3],
    pub dimensions: [f32; 3],
    pub stack_id: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponDto {
    pub entity_id: u32,
    pub profile_id: u16,
    pub position: [f32; 3],
    pub aim_direction: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraDto {
    pub position: [f32; 3],
    pub look_at: [f32; 3],
    pub fov_deg: f32,
}

// ============================================================================
// MATERIAL STACK DTO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialStackDto {
    pub id: u16,
    pub label: String,
    pub layers: Vec<MaterialLayerDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayerDto {
    pub archetype_id: u16,
    pub archetype_label: String,
    pub thickness_mm: f32,
    pub coverage: f32,
}

// ============================================================================
// DAMAGE MEMORY DTO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageMemoryDto {
    pub entity_id: u32,
    pub stack_id: u16,
    pub layer_damage: Vec<LayerDamageDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerDamageDto {
    pub layer_index: u8,
    pub integrity: f32,
    pub accumulated_energy_j: f32,
    pub cracked_segments: Vec<u16>,
    pub released_segments: Vec<u16>,
}

// ============================================================================
// BALLISTIC RESULT DTO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BallisticSimulationResultDto {
    pub impacts: Vec<ImpactResultDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactResultDto {
    pub target_entity: u32,
    pub impact_position: [f32; 3],
    pub impact_velocity: [f32; 3],
    pub entry_energy_j: f32,
    pub incidence_angle_deg: f32,
    pub layer_events: Vec<LayerImpactEventDto>,
    pub final_verdict: ImpactVerdictDto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactVerdictDto {
    Stopped,
    Penetrated,
    Ricochet,
    Embedded,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerImpactEventDto {
    pub layer_index: u8,
    pub entry_energy_j: f32,
    pub exit_energy_j: f32,
    pub energy_absorbed_j: f32,
    pub verdict: ImpactVerdictDto,
}

// ============================================================================
// RUNTIME EVENT DTO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeEventDto {
    ShotFired {
        tick: u64,
        weapon: u32,
    },
    ProjectileImpact {
        tick: u64,
        target: u32,
        energy_j: f32,
    },
    SegmentCracked {
        tick: u64,
        entity: u32,
        layer: u8,
        segment: u16,
    },
    SegmentReleased {
        tick: u64,
        entity: u32,
        layer: u8,
        segment: u16,
    },
}

// ============================================================================
// SHOT LOG DTO
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShotLogEntryDto {
    pub tick: u64,
    pub weapon_entity: u32,
    pub projectile_profile: u16,
    pub spawn_position: [f32; 3],
    pub spawn_velocity: [f32; 3],
}

// ============================================================================
// VERTICAL SLICE OBSERVATION
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerticalSliceObservation {
    pub request_id: u64,
    pub scene: Option<SceneDto>,
    pub material_stacks: Vec<MaterialStackDto>,
    pub damage_memory: Vec<DamageMemoryDto>,
    pub ballistic_result: Option<BallisticSimulationResultDto>,
    pub runtime_events: Vec<RuntimeEventDto>,
    pub shot_log: Vec<ShotLogEntryDto>,
}

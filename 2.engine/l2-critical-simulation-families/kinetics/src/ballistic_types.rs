use serde::{Deserialize, Serialize};

// ============================================================================
// PROJECTILE PROFILE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProjectileProfileId(pub u16);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectileProfile {
    pub id: ProjectileProfileId,
    pub label: String,
    pub mass_kg: f32,
    pub diameter_mm: f32,
    pub muzzle_velocity_m_s: f32,
    pub drag_coefficient: f32,
}

// ============================================================================
// WEAPON PROFILE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WeaponProfileId(pub u16);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponProfile {
    pub id: WeaponProfileId,
    pub label: String,
    pub projectile_profile: ProjectileProfileId,
    pub fire_rate_rpm: f32,
}

// ============================================================================
// PROJECTILE STATE
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectileState {
    pub profile_id: ProjectileProfileId,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub time_alive_s: f32,
}

// ============================================================================
// IMPACT VERDICT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactVerdict {
    Stopped,
    Penetrated,
    Ricochet,
    Embedded,
}

// ============================================================================
// LAYER IMPACT EVENT
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerImpactEvent {
    pub layer_index: u8,
    pub entry_energy_j: f32,
    pub exit_energy_j: f32,
    pub energy_absorbed_j: f32,
    pub verdict: ImpactVerdict,
}

// ============================================================================
// IMPACT RESULT
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactResult {
    pub target_entity: u32,
    pub impact_position: [f32; 3],
    pub impact_velocity: [f32; 3],
    pub entry_energy_j: f32,
    pub incidence_angle_deg: f32,
    pub layer_events: Vec<LayerImpactEvent>,
    pub final_verdict: ImpactVerdict,
}

// ============================================================================
// BALLISTIC SIMULATION RESULT
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BallisticSimulationResult {
    pub projectile_states: Vec<ProjectileState>,
    pub impacts: Vec<ImpactResult>,
}

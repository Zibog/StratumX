use super::identity::{EntityId, ProjectileProfileId};
use engine_core::Tick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShotRecord {
    pub tick: Tick,
    pub weapon_entity: EntityId,
    pub projectile_profile: ProjectileProfileId,
    pub spawn_position: [f32; 3],
    pub spawn_velocity: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactRecord {
    pub tick: Tick,
    pub projectile_profile: ProjectileProfileId,
    pub target_entity: EntityId,
    pub impact_position: [f32; 3],
    pub impact_velocity: [f32; 3],
    pub entry_energy_j: f32,
    pub incidence_angle_deg: f32,
    pub penetrated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeEvent {
    ShotFired {
        tick: Tick,
        weapon: EntityId,
    },
    ProjectileImpact {
        tick: Tick,
        target: EntityId,
        energy_j: f32,
    },
    SegmentCracked {
        tick: Tick,
        entity: EntityId,
        layer: u8,
        segment: u16,
    },
    SegmentReleased {
        tick: Tick,
        entity: EntityId,
        layer: u8,
        segment: u16,
    },
}

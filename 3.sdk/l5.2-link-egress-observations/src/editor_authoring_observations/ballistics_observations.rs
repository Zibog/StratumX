//! Ballistics observation types for editor authoring
//!
//! Re-uses ballistic result DTOs from the vertical slice module
//! and provides a dedicated observation enum for ballistics commands.

use serde::{Deserialize, Serialize};

// Re-use the ballistic DTOs from vertical slice observations
use crate::vertical_slice_observations::BallisticSimulationResultDto;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BallisticsObservation {
    ShotFired {
        tick: u64,
        weapon_entity_id: u32,
    },
    ProjectileImpact {
        tick: u64,
        target_entity_id: u32,
        impact_position: [f32; 3],
        impact_velocity: [f32; 3],
        entry_energy_j: f32,
    },
    SimulationComplete {
        result: BallisticSimulationResultDto,
    },
    NoActiveActor,
}

use super::fracture::{generate_fragments, Fragment};
use super::support_failure::{determine_failure_mode, FailureMode, StructureType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestructionResponse {
    pub structure_type: StructureType,
    pub failure_mode: FailureMode,
    pub fragments: Vec<Fragment>,
    pub dust_cloud_radius_m: f32,
    pub destroyed: bool,
}

impl DestructionResponse {
    pub fn from_impact(
        structure_type: StructureType,
        impact_position: [f32; 3],
        impact_energy_j: f32,
        impact_direction: [f32; 3],
    ) -> Self {
        let (failure_mode, destroyed) = determine_failure_mode(structure_type, impact_energy_j);
        let mut fragments = Vec::new();
        if destroyed {
            fragments = generate_fragments(
                impact_position,
                impact_direction,
                impact_energy_j,
                structure_type.fragment_count(),
                failure_mode,
                structure_type.can_burn(),
            );
        }
        let dust_cloud_radius_m = if destroyed { 2.0 } else { 0.0 };
        Self {
            structure_type,
            failure_mode,
            fragments,
            dust_cloud_radius_m,
            destroyed,
        }
    }
}

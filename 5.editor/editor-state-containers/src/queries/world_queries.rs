//! World query types

use super::ReadModel;
use crate::owners::world_owner::WorldOwner;

#[derive(Debug, Clone)]
pub struct WorldIdentityView {
    pub world_name: String,
    pub snapshot_ref: String,
}

impl ReadModel<WorldOwner> for WorldIdentityView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            world_name: owner.world_identity.world_name.clone(),
            snapshot_ref: owner.world_snapshot_ref.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorldTerrainSummaryView {
    pub has_terrain: bool,
    pub heightmap_resolution: Option<(u32, u32)>,
}

impl ReadModel<WorldOwner> for WorldTerrainSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            has_terrain: owner.terrain_state.is_some(),
            heightmap_resolution: owner.terrain_state.as_ref().map(|t| t.resolution),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorldEnvironmentSummaryView {
    pub has_environment: bool,
    pub weather_condition: Option<crate::WeatherCondition>,
}

impl ReadModel<WorldOwner> for WorldEnvironmentSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            has_environment: owner.environment_state.is_some(),
            weather_condition: owner.environment_state.as_ref().map(|e| e.weather.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorldDiagnosticsSummaryView {
    pub diagnostic_count: usize,
    pub error_count: usize,
}

impl ReadModel<WorldOwner> for WorldDiagnosticsSummaryView {
    fn build(owner: &WorldOwner) -> Self {
        Self {
            diagnostic_count: owner.world_diagnostics.len(),
            error_count: owner
                .world_diagnostics
                .iter()
                .filter(|d| matches!(d.severity, crate::Severity::Error))
                .count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterialCoverageSummaryView {
    pub total_materials: usize,
}

impl ReadModel<WorldOwner> for MaterialCoverageSummaryView {
    fn build(_owner: &WorldOwner) -> Self {
        Self { total_materials: 0 }
    }
}

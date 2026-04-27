// Terrain Diagnostics

use super::state::TerrainAuthoringState;
use editor_dto_law::LodPosture;

#[derive(Debug, Clone)]
pub struct TerrainDiagnostics {
    pub bound: bool,
    pub present: bool,
    pub walkable: bool,
    pub material_bound: bool,
    pub lod_posture: LodPosture,
    pub degraded: bool,
}

impl TerrainDiagnostics {
    pub fn is_healthy(&self) -> bool {
        self.bound && self.present && self.walkable && self.material_bound && !self.degraded
    }

    pub fn get_issues(&self) -> Vec<String> {
        let mut issues = Vec::new();

        if !self.bound {
            issues.push("Terrain not bound to world".to_string());
        }
        if !self.present {
            issues.push("Terrain not present in scene".to_string());
        }
        if !self.walkable {
            issues.push("Terrain collision not enabled".to_string());
        }
        if !self.material_bound {
            issues.push("Terrain material not bound".to_string());
        }
        if self.degraded {
            issues.push("Terrain in degraded state".to_string());
        }

        issues
    }
}

impl TerrainAuthoringState {
    pub fn diagnostics(&self) -> TerrainDiagnostics {
        TerrainDiagnostics {
            bound: self.binding_ref.is_some(),
            present: self.present,
            walkable: self.walkable,
            material_bound: self.material_profile_ref.is_some(),
            lod_posture: self.lod_posture.clone(),
            degraded: self.degraded,
        }
    }
}

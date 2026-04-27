// Environment Diagnostics

use super::state::EnvironmentAuthoringState;
use editor_dto_law::WeatherStateSnapshot;

#[derive(Debug, Clone)]
pub struct EnvironmentDiagnostics {
    pub bound: bool,
    pub present: bool,
    pub sun_profile_present: bool,
    pub weather_director_active: bool,
    pub degraded: bool,
}

impl EnvironmentDiagnostics {
    pub fn is_healthy(&self) -> bool {
        self.bound
            && self.present
            && self.sun_profile_present
            && self.weather_director_active
            && !self.degraded
    }
}

impl EnvironmentAuthoringState {
    pub fn diagnostics(&self) -> EnvironmentDiagnostics {
        EnvironmentDiagnostics {
            bound: self.binding_ref.is_some(),
            present: self.present,
            sun_profile_present: true,
            weather_director_active: self.binding_ref.is_some(),
            degraded: self.degraded,
        }
    }

    pub fn weather_snapshot(&self) -> Option<WeatherStateSnapshot> {
        if !self.present {
            return None;
        }
        let sky_state = self.engine_bridge.sky_state();
        Some(WeatherStateSnapshot {
            time_of_day: self.time_of_day,
            sun_elevation_deg: 45.0,
            cloud_coverage: sky_state.cloud_coverage,
            fog_density: 0.0,
            rain_enabled: sky_state.rain_enabled,
            weather_regime: format!("{:?}", sky_state.weather_director.target_regime),
            cloud_shadow_active: sky_state.cloud_shadow_projector.active,
            weather_cells_count: sky_state.weather_cells.len(),
        })
    }
}

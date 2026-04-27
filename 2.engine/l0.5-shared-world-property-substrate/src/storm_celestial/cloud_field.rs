use serde::{Deserialize, Serialize};

/// Cloud profile state - visual appearance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudProfileState {
    pub coverage: f32,
    pub density: f32,
    pub erosion: f32,
    pub base_height_km: f32,
    pub top_height_km: f32,
    pub shadow_strength: f32,
}

/// Cloud shadow projector state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudShadowProjectorState {
    pub active: bool,
    pub shadow_map_posture: ShadowMapPosture,
    pub density_driven: bool,
    pub near_update_rate_hz: f32,
    pub mid_update_rate_hz: f32,
    pub far_update_rate_hz: f32,
}

/// Shadow map posture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowMapPosture {
    Full,
    Simplified,
    Fallback,
    Disabled,
}

/// Distance tier for weather cells
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherDistanceTier {
    Near,
    Mid,
    Far,
}

/// Calculate distance tier based on distance in kilometers
pub fn calculate_distance_tier(distance_km: f32) -> WeatherDistanceTier {
    if distance_km < 2.0 {
        WeatherDistanceTier::Near
    } else if distance_km < 10.0 {
        WeatherDistanceTier::Mid
    } else {
        WeatherDistanceTier::Far
    }
}

/// Weather cell shadow data
#[allow(dead_code)]
pub struct WeatherCellShadowData {
    pub position: [f32; 3],
    pub radius_km: f32,
    pub shadow_opacity: f32,
}

/// Get cloud shadow opacity at a position from weather cells
#[allow(dead_code)]
pub fn calculate_cloud_shadow_opacity(
    position: [f32; 3],
    weather_cells: &[WeatherCellShadowData],
    cloud_coverage: f32,
    cloud_shadow_strength: f32,
) -> f32 {
    let mut total_opacity = 0.0;

    for cell in weather_cells {
        let dx = position[0] - cell.position[0];
        let dz = position[2] - cell.position[2];
        let distance_km = ((dx * dx + dz * dz).sqrt()) / 1000.0;

        if distance_km < cell.radius_km {
            let falloff = 1.0 - (distance_km / cell.radius_km);
            total_opacity += cell.shadow_opacity * falloff;
        }
    }

    total_opacity += cloud_coverage * cloud_shadow_strength * 0.3;
    total_opacity.min(1.0)
}

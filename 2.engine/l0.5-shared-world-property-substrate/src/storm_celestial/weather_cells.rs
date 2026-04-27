use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherCellState {
    pub cell_id: u32,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub radius_km: f32,
    pub density: f32,
    pub coverage: f32,
    pub vertical_growth: f32,
    pub precipitation_rate: f32,
    pub lightning_probability: f32,
    pub shadow_opacity: f32,
}

pub fn create_weather_cell(
    cell_id: u32,
    position: [f32; 3],
    velocity: [f32; 3],
    radius_km: f32,
    density: f32,
) -> WeatherCellState {
    WeatherCellState {
        cell_id,
        position,
        velocity,
        radius_km,
        density: density.clamp(0.0, 1.0),
        coverage: 0.5,
        vertical_growth: 1.0,
        precipitation_rate: 0.0,
        lightning_probability: 0.0,
        shadow_opacity: density * 0.6,
    }
}

pub fn update_weather_cell(
    cell: &mut WeatherCellState,
    precipitation_rate: Option<f32>,
    lightning_probability: Option<f32>,
) {
    if let Some(precip) = precipitation_rate {
        cell.precipitation_rate = precip.max(0.0);
    }
    if let Some(lightning) = lightning_probability {
        cell.lightning_probability = lightning.clamp(0.0, 1.0);
    }
    cell.shadow_opacity = cell.density * 0.6;
}

pub fn evolve_weather_cell(
    cell: &mut WeatherCellState,
    dt_seconds: f32,
    is_heavy_storm: bool,
    is_clear_or_post_storm: bool,
) {
    cell.position[0] += cell.velocity[0] * dt_seconds;
    cell.position[1] += cell.velocity[1] * dt_seconds;
    cell.position[2] += cell.velocity[2] * dt_seconds;
    if cell.density > 0.7 {
        cell.vertical_growth += dt_seconds * 0.1 * cell.coverage;
        cell.vertical_growth = cell.vertical_growth.min(2.0);
    } else if cell.density < 0.3 {
        cell.vertical_growth -= dt_seconds * 0.05;
        cell.vertical_growth = cell.vertical_growth.max(0.1);
    }
    if is_heavy_storm {
        cell.density += dt_seconds * 0.02;
        cell.density = cell.density.min(1.0);
    } else if is_clear_or_post_storm {
        cell.density -= dt_seconds * 0.01;
        cell.density = cell.density.max(0.1);
    }
    cell.shadow_opacity = (cell.density * 0.6 + cell.vertical_growth * 0.2).min(0.9);
    cell.coverage += (cell.density - cell.coverage) * dt_seconds * 0.1;
    cell.coverage = cell.coverage.clamp(0.0, 1.0);
}

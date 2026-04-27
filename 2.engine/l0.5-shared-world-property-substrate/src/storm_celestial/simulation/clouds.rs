use super::super::weather_cells::WeatherCellState;

/// Create a new weather cell and return its ID.
pub fn create_weather_cell(
    weather_cells: &mut Vec<WeatherCellState>,
    next_cell_id: &mut u32,
    position: [f32; 3],
    velocity: [f32; 3],
    radius_km: f32,
    density: f32,
) -> u32 {
    let cell_id = *next_cell_id;
    *next_cell_id += 1;
    weather_cells.push(WeatherCellState {
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
    });
    cell_id
}

/// Update properties of an existing weather cell.
pub fn update_weather_cell(
    weather_cells: &mut [WeatherCellState],
    cell_id: u32,
    precipitation_rate: Option<f32>,
    lightning_probability: Option<f32>,
) -> bool {
    if let Some(cell) = weather_cells.iter_mut().find(|c| c.cell_id == cell_id) {
        if let Some(precip) = precipitation_rate {
            cell.precipitation_rate = precip.max(0.0);
        }
        if let Some(lightning) = lightning_probability {
            cell.lightning_probability = lightning.clamp(0.0, 1.0);
        }
        cell.shadow_opacity = cell.density * 0.6;
        true
    } else {
        false
    }
}

/// Remove a weather cell by ID.
pub fn remove_weather_cell(weather_cells: &mut Vec<WeatherCellState>, cell_id: u32) -> bool {
    let initial_len = weather_cells.len();
    weather_cells.retain(|c| c.cell_id != cell_id);
    weather_cells.len() < initial_len
}

/// Calculate the distance tier from observer to a weather cell.
pub fn get_weather_cell_distance_tier(
    cell_position: [f32; 3],
    observer_position: [f32; 3],
) -> super::super::cloud_field::WeatherDistanceTier {
    let dx = cell_position[0] - observer_position[0];
    let dy = cell_position[1] - observer_position[1];
    let dz = cell_position[2] - observer_position[2];
    let distance_km = ((dx * dx + dy * dy + dz * dz).sqrt()) / 1000.0;
    super::super::cloud_field::calculate_distance_tier(distance_km)
}

/// Calculate cloud shadow opacity at a given position.
pub fn get_cloud_shadow_opacity_at(
    cloud_shadow_active: bool,
    weather_cells: &[WeatherCellState],
    cloud_profile_coverage: f32,
    cloud_profile_shadow_strength: f32,
    position: [f32; 3],
) -> f32 {
    if !cloud_shadow_active {
        return 0.0;
    }
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
    total_opacity += cloud_profile_coverage * cloud_profile_shadow_strength * 0.3;
    total_opacity.min(1.0)
}

use super::super::solar_cycle::calculate_sun_position;

/// Update sun position based on celestial parameters.
/// Returns (sun_direction, elevation, sun_intensity).
pub fn update_sun_position(
    time_of_day_hours: f32,
    day_of_year: u16,
    latitude_deg: f32,
) -> ([f32; 3], f32, f32) {
    calculate_sun_position(time_of_day_hours, day_of_year, latitude_deg)
}

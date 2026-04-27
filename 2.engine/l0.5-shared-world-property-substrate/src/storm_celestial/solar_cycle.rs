use serde::{Deserialize, Serialize};

/// Celestial time and sun position state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CelestialTimeState {
    pub time_of_day_hours: f32,
    pub day_of_year: u16,
    pub latitude_deg: f32,
    pub sun_direction: [f32; 3],
    pub sun_elevation_deg: f32,
    pub sun_intensity: f32,
}

/// Atmosphere profile state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtmosphereProfileState {
    pub rayleigh_strength: f32,
    pub mie_strength: f32,
    pub fog_density: f32,
    pub horizon_visibility_km: f32,
}

/// Sun profile state - analytical sun disk and halo
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SunProfileState {
    pub angular_radius_deg: f32,
    pub disk_intensity: f32,
    pub color_temperature_kelvin: f32,
    pub halo_intensity: f32,
    pub halo_falloff: f32,
    pub sunset_shift_strength: f32,
    pub cloud_scatter_response: f32,
    pub disk_softness: f32,
    pub optional_pulse_curve: Option<f32>,
}

/// Calculate sun position based on time, day of year, and latitude
pub fn calculate_sun_position(
    time_of_day_hours: f32,
    day_of_year: u16,
    latitude_deg: f32,
) -> ([f32; 3], f32, f32) {
    let hour_angle = (time_of_day_hours - 12.0) * 15.0;
    let declination = 23.45
        * ((360.0 / 365.0) * (day_of_year as f32 - 81.0))
            .to_radians()
            .sin();

    let lat_rad = latitude_deg.to_radians();
    let dec_rad = declination.to_radians();
    let hour_rad = hour_angle.to_radians();

    let sin_elevation =
        lat_rad.sin() * dec_rad.sin() + lat_rad.cos() * dec_rad.cos() * hour_rad.cos();
    let elevation = sin_elevation.asin().to_degrees();

    let azimuth_rad = if elevation > -0.5 {
        let cos_azimuth = (dec_rad.sin() - lat_rad.sin() * sin_elevation)
            / (lat_rad.cos() * sin_elevation.acos().cos());
        let azimuth = cos_azimuth.clamp(-1.0, 1.0).acos();
        if hour_angle > 0.0 {
            azimuth
        } else {
            -azimuth
        }
    } else {
        0.0
    };

    let elev_rad = elevation.to_radians();
    let sun_direction = [
        azimuth_rad.sin() * elev_rad.cos(),
        elev_rad.sin(),
        azimuth_rad.cos() * elev_rad.cos(),
    ];

    let sun_intensity = if elevation > 0.0 {
        (elevation / 90.0).clamp(0.0, 1.0)
    } else {
        0.0
    };

    (sun_direction, elevation, sun_intensity)
}

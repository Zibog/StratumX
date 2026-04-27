use super::super::cloud_field::{CloudProfileState, CloudShadowProjectorState, ShadowMapPosture};
use super::super::solar_cycle::{AtmosphereProfileState, CelestialTimeState, SunProfileState};
use super::super::storm_fronts::StormFrontState;
use super::super::weather_cells::WeatherCellState;
use super::weather_director::{WeatherDirectorState, WeatherRegime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkyWeatherState {
    pub celestial: CelestialTimeState,
    pub atmosphere: AtmosphereProfileState,
    pub sun_profile: SunProfileState,
    pub weather_director: WeatherDirectorState,
    pub cloud_profile: CloudProfileState,
    pub cloud_shadow_projector: CloudShadowProjectorState,
    pub cloud_coverage: f32,
    pub wind_vector: [f32; 3],
    pub rain_enabled: bool,
    pub rain_intensity_mm_per_hour: f32,
    pub storm_fronts: Vec<StormFrontState>,
    pub weather_cells: Vec<WeatherCellState>,
    pub(crate) next_storm_id: u32,
    pub(crate) next_cell_id: u32,
}

impl SkyWeatherState {
    pub fn new_default() -> Self {
        Self {
            celestial: CelestialTimeState {
                time_of_day_hours: 14.0,
                day_of_year: 180,
                latitude_deg: 51.0,
                sun_direction: [0.0, 0.7, 0.7],
                sun_elevation_deg: 45.0,
                sun_intensity: 1.0,
            },
            atmosphere: AtmosphereProfileState {
                rayleigh_strength: 1.0,
                mie_strength: 1.0,
                fog_density: 0.02,
                horizon_visibility_km: 50.0,
            },
            sun_profile: SunProfileState {
                angular_radius_deg: 0.53,
                disk_intensity: 1.0,
                color_temperature_kelvin: 5778.0,
                halo_intensity: 0.15,
                halo_falloff: 2.0,
                sunset_shift_strength: 0.8,
                cloud_scatter_response: 0.6,
                disk_softness: 0.05,
                optional_pulse_curve: None,
            },
            weather_director: WeatherDirectorState {
                target_regime: WeatherRegime::Scattered,
                transition_progress: 1.0,
                authoring_lock: false,
                storm_bias: 0.3,
                fog_bias: 0.2,
                rain_bias: 0.4,
            },
            cloud_profile: CloudProfileState {
                coverage: 0.35,
                density: 0.6,
                erosion: 0.4,
                base_height_km: 1.5,
                top_height_km: 3.0,
                shadow_strength: 0.5,
            },
            cloud_shadow_projector: CloudShadowProjectorState {
                active: true,
                shadow_map_posture: ShadowMapPosture::Full,
                density_driven: true,
                near_update_rate_hz: 30.0,
                mid_update_rate_hz: 10.0,
                far_update_rate_hz: 2.0,
            },
            cloud_coverage: 0.35,
            wind_vector: [3.0, 0.0, 1.0],
            rain_enabled: false,
            rain_intensity_mm_per_hour: 0.0,
            storm_fronts: Vec::new(),
            weather_cells: Vec::new(),
            next_storm_id: 1,
            next_cell_id: 1,
        }
    }
}

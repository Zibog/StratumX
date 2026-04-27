use super::clouds::{self, create_weather_cell, remove_weather_cell, update_weather_cell};
use super::fronts::{create_storm_front, update_storm_front};
use super::sky_weather::SkyWeatherState;
use super::weather_director::WeatherRegime;

impl SkyWeatherState {
    pub fn set_weather_regime(&mut self, regime: WeatherRegime) {
        if self.weather_director.authoring_lock {
            return;
        }
        self.weather_director.target_regime = regime;
        self.weather_director.transition_progress = 0.0;
    }

    pub fn lock_weather_authoring(&mut self, locked: bool) {
        self.weather_director.authoring_lock = locked;
    }

    pub fn set_weather_bias(&mut self, storm: f32, fog: f32, rain: f32) {
        self.weather_director.storm_bias = storm.clamp(0.0, 1.0);
        self.weather_director.fog_bias = fog.clamp(0.0, 1.0);
        self.weather_director.rain_bias = rain.clamp(0.0, 1.0);
    }

    pub fn update_weather_transition(&mut self, dt_seconds: f32) {
        if self.weather_director.transition_progress < 1.0 {
            self.weather_director.transition_progress += dt_seconds * 0.1;
            self.weather_director.transition_progress =
                self.weather_director.transition_progress.min(1.0);
            self.apply_weather_regime();
        }
    }

    fn apply_weather_regime(&mut self) {
        let progress = self.weather_director.transition_progress;
        match self.weather_director.target_regime {
            WeatherRegime::Clear => {
                self.cloud_profile.coverage = 0.1 * progress;
                self.rain_enabled = false;
            }
            WeatherRegime::Scattered => {
                self.cloud_profile.coverage = 0.35 * progress;
                self.rain_enabled = false;
            }
            WeatherRegime::Overcast => {
                self.cloud_profile.coverage = 0.85 * progress;
                self.cloud_profile.density = 0.7 * progress;
                self.rain_enabled = false;
            }
            WeatherRegime::IncomingStorm => {
                self.cloud_profile.coverage = 0.9 * progress;
                self.cloud_profile.density = 0.8 * progress;
                self.rain_enabled = progress > 0.5;
                self.rain_intensity_mm_per_hour = 5.0 * (progress - 0.5).max(0.0) * 2.0;
            }
            WeatherRegime::HeavyStorm => {
                self.cloud_profile.coverage = 1.0;
                self.cloud_profile.density = 0.95 * progress;
                self.rain_enabled = true;
                self.rain_intensity_mm_per_hour = 20.0 * progress;
            }
            WeatherRegime::PostStormCalm => {
                self.cloud_profile.coverage = 0.6 * progress;
                self.cloud_profile.density = 0.4 * progress;
                self.rain_enabled = false;
                self.atmosphere.fog_density = 0.1 * progress;
            }
            WeatherRegime::FogMorning => {
                self.cloud_profile.coverage = 0.3 * progress;
                self.atmosphere.fog_density = 0.3 * progress;
                self.rain_enabled = false;
            }
            WeatherRegime::WindyOvercast => {
                self.cloud_profile.coverage = 0.75 * progress;
                self.cloud_profile.erosion = 0.7 * progress;
                let wind_strength = 8.0 * progress;
                self.wind_vector = [wind_strength, 0.0, wind_strength * 0.5];
                self.rain_enabled = false;
            }
        }
        self.cloud_coverage = self.cloud_profile.coverage;
    }

    pub fn create_weather_cell(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius_km: f32,
        density: f32,
    ) -> u32 {
        create_weather_cell(
            &mut self.weather_cells,
            &mut self.next_cell_id,
            position,
            velocity,
            radius_km,
            density,
        )
    }

    pub fn update_weather_cell(
        &mut self,
        cell_id: u32,
        precipitation_rate: Option<f32>,
        lightning_probability: Option<f32>,
    ) -> bool {
        update_weather_cell(
            &mut self.weather_cells,
            cell_id,
            precipitation_rate,
            lightning_probability,
        )
    }

    pub fn remove_weather_cell(&mut self, cell_id: u32) -> bool {
        remove_weather_cell(&mut self.weather_cells, cell_id)
    }

    pub fn get_weather_cell_distance_tier(
        &self,
        cell_position: [f32; 3],
        observer_position: [f32; 3],
    ) -> super::super::cloud_field::WeatherDistanceTier {
        self::clouds::get_weather_cell_distance_tier(cell_position, observer_position)
    }

    pub fn create_storm_front(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius_km: f32,
        intensity: f32,
        rain_intensity_mm_per_hour: f32,
    ) -> u32 {
        create_storm_front(
            &mut self.storm_fronts,
            &mut self.next_storm_id,
            position,
            velocity,
            radius_km,
            intensity,
            rain_intensity_mm_per_hour,
        )
    }

    pub fn update_storm_front(
        &mut self,
        front_id: u32,
        position: Option<[f32; 3]>,
        velocity: Option<[f32; 3]>,
        radius_km: Option<f32>,
        intensity: Option<f32>,
        rain_intensity_mm_per_hour: Option<f32>,
    ) -> bool {
        update_storm_front(
            &mut self.storm_fronts,
            front_id,
            position,
            velocity,
            radius_km,
            intensity,
            rain_intensity_mm_per_hour,
        )
    }
}

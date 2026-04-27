use super::sky_weather::SkyWeatherState;
use super::sun::update_sun_position as calc_sunposition;
use super::weather_director::WeatherRegime;

impl SkyWeatherState {
    pub fn set_time_of_day(&mut self, hours: f32) {
        self.celestial.time_of_day_hours = hours.clamp(0.0, 24.0);
        self.update_sun_position();
    }

    pub fn set_day_of_year(&mut self, day: u16) {
        self.celestial.day_of_year = day.clamp(1, 365);
        self.update_sun_position();
    }

    pub fn set_latitude(&mut self, latitude_deg: f32) {
        self.celestial.latitude_deg = latitude_deg.clamp(-90.0, 90.0);
        self.update_sun_position();
    }

    pub fn set_fog_density(&mut self, value: f32) {
        let (fog_density, horizon_visibility_km) = super::fog::set_fog_density(value);
        self.atmosphere.fog_density = fog_density;
        self.atmosphere.horizon_visibility_km = horizon_visibility_km;
    }

    pub fn set_rain(&mut self, enabled: bool, intensity_mm_per_hour: f32) {
        let (rain_enabled, rain_intensity) = super::rain::set_rain(enabled, intensity_mm_per_hour);
        self.rain_enabled = rain_enabled;
        self.rain_intensity_mm_per_hour = rain_intensity;
    }

    pub fn set_wind_vector(&mut self, value: [f32; 3]) {
        self.wind_vector = super::wind::set_wind_vector(value);
    }

    pub fn step_simulation(&mut self, dt_seconds: f32) {
        self.update_weather_transition(dt_seconds);
        for front in &mut self.storm_fronts {
            front.position[0] += front.velocity[0] * dt_seconds;
            front.position[1] += front.velocity[1] * dt_seconds;
            front.position[2] += front.velocity[2] * dt_seconds;
        }
        for cell in &mut self.weather_cells {
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
            match self.weather_director.target_regime {
                WeatherRegime::HeavyStorm => {
                    cell.density += dt_seconds * 0.02;
                    cell.density = cell.density.min(1.0);
                }
                WeatherRegime::Clear | WeatherRegime::PostStormCalm => {
                    cell.density -= dt_seconds * 0.01;
                    cell.density = cell.density.max(0.1);
                }
                _ => {}
            }
            cell.shadow_opacity = (cell.density * 0.6 + cell.vertical_growth * 0.2).min(0.9);
            cell.coverage += (cell.density - cell.coverage) * dt_seconds * 0.1;
            cell.coverage = cell.coverage.clamp(0.0, 1.0);
        }
        self.update_sun_position();
    }

    pub(crate) fn update_sun_position(&mut self) {
        let (sun_direction, elevation, sun_intensity) = calc_sunposition(
            self.celestial.time_of_day_hours,
            self.celestial.day_of_year,
            self.celestial.latitude_deg,
        );
        self.celestial.sun_direction = sun_direction;
        self.celestial.sun_elevation_deg = elevation;
        self.celestial.sun_intensity = sun_intensity;
    }
}

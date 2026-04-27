use super::super::cloud_field::ShadowMapPosture;
use super::clouds::get_cloud_shadow_opacity_at;
use super::sky_weather::SkyWeatherState;

impl SkyWeatherState {
    pub fn set_cloud_shadow_active(&mut self, active: bool) {
        self.cloud_shadow_projector.active = active;
    }

    pub fn set_cloud_shadow_posture(&mut self, posture: ShadowMapPosture) {
        self.cloud_shadow_projector.shadow_map_posture = posture;
    }

    pub fn set_cloud_shadow_update_rates(&mut self, near_hz: f32, mid_hz: f32, far_hz: f32) {
        self.cloud_shadow_projector.near_update_rate_hz = near_hz.max(0.1);
        self.cloud_shadow_projector.mid_update_rate_hz = mid_hz.max(0.1);
        self.cloud_shadow_projector.far_update_rate_hz = far_hz.max(0.1);
    }

    pub fn get_cloud_shadow_opacity_at(&self, position: [f32; 3]) -> f32 {
        get_cloud_shadow_opacity_at(
            self.cloud_shadow_projector.active,
            &self.weather_cells,
            self.cloud_profile.coverage,
            self.cloud_profile.shadow_strength,
            position,
        )
    }
}

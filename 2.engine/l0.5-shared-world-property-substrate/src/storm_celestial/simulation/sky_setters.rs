use super::sky_weather::SkyWeatherState;

impl SkyWeatherState {
    pub fn set_cloud_coverage(&mut self, value: f32) {
        self.cloud_coverage = value.clamp(0.0, 1.0);
    }

    pub fn set_sun_temperature(&mut self, kelvin: f32) {
        self.sun_profile.color_temperature_kelvin = kelvin.clamp(1000.0, 40000.0);
    }

    pub fn set_sun_halo(&mut self, intensity: f32, falloff: f32) {
        self.sun_profile.halo_intensity = intensity.clamp(0.0, 1.0);
        self.sun_profile.halo_falloff = falloff.max(0.1);
    }

    pub fn set_sun_disk_radius(&mut self, angular_radius_deg: f32) {
        self.sun_profile.angular_radius_deg = angular_radius_deg.clamp(0.1, 5.0);
    }

    pub fn set_sun_disk_intensity(&mut self, intensity: f32) {
        self.sun_profile.disk_intensity = intensity.clamp(0.0, 10.0);
    }

    pub fn set_sunset_shift(&mut self, strength: f32) {
        self.sun_profile.sunset_shift_strength = strength.clamp(0.0, 1.0);
    }

    pub fn set_cloud_scatter_response(&mut self, response: f32) {
        self.sun_profile.cloud_scatter_response = response.clamp(0.0, 1.0);
    }

    pub fn set_sun_disk_softness(&mut self, softness: f32) {
        self.sun_profile.disk_softness = softness.clamp(0.0, 1.0);
    }

    pub fn set_sun_pulse_curve(&mut self, pulse: Option<f32>) {
        self.sun_profile.optional_pulse_curve = pulse.map(|p| p.clamp(0.0, 1.0));
    }
}

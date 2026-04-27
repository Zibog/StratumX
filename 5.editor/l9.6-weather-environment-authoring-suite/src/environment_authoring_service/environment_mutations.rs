use crate::{EnvironmentState, WeatherCondition};

pub fn configure_sky(environment: &mut EnvironmentState, sky_color: [f32; 3], _sun_intensity: f32) {
    environment.ambient_light = sky_color;
}

pub fn set_weather(environment: &mut EnvironmentState, weather: WeatherCondition) {
    environment.weather_condition = weather;
}

pub fn update_lighting(
    environment: &mut EnvironmentState,
    ambient_intensity: f32,
    _shadow_quality: u32,
) {
    environment.ambient_light = [ambient_intensity, ambient_intensity, ambient_intensity];
}

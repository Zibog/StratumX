//! Sky fixture for testing

use engine_material::SkyWeatherState;

/// Creates a default sky state for testing
pub fn create_test_sky() -> SkyWeatherState {
    let mut sky = SkyWeatherState::new_default();
    sky.set_weather_regime(engine_material::WeatherRegime::Clear);
    sky.update_weather_transition(10.0);
    sky
}

/// Creates a sky state at specific time of day
pub fn create_sky_at_time(time_hours: f32) -> SkyWeatherState {
    let mut sky = create_test_sky();
    sky.set_time_of_day(time_hours);
    sky
}

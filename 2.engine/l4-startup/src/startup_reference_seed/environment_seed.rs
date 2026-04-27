// Environment seed - sky/weather setup

use engine_material::SkyWeatherState;

pub fn create_default_sky() -> SkyWeatherState {
    SkyWeatherState::new_default()
}

pub fn get_default_sky_bundle_path() -> Option<String> {
    Some("9.assets/shared/sky/sky_bundle.json".to_string())
}

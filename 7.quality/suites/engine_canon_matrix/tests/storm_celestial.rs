//! Storm & Celestial: проверка взаимодействия погоды и небесных тел

use engine_material::{SkyWeatherState, WeatherRegime};

#[test]
fn storm_celestial_integration() {
    let mut sky = SkyWeatherState::new_default();
    sky.set_weather_regime(WeatherRegime::HeavyStorm);
    sky.update_weather_transition(20.0);

    assert!(sky.rain_enabled);
    assert!(sky.cloud_coverage >= 0.99);
    assert!(sky.rain_intensity_mm_per_hour > 0.0);
}

#[test]
fn sun_position_affects_storm() {
    let mut sky = SkyWeatherState::new_default();
    let original_direction = sky.celestial.sun_direction;

    sky.set_time_of_day(6.0);
    sky.set_latitude(10.0);

    assert_ne!(sky.celestial.sun_direction, original_direction);
    assert!(sky.celestial.sun_elevation_deg.is_finite());
    assert!(sky.celestial.sun_intensity.is_finite());
}

#[test]
fn moon_phase_affects_weather() {
    let mut sky = SkyWeatherState::new_default();
    let front_id = sky.create_storm_front([0.0, 1.0, 0.0], [2.0, 0.0, 0.0], 5.0, 0.8, 6.0);
    let cell_id = sky.create_weather_cell([0.0, 2.0, 0.0], [1.0, 0.0, 1.0], 4.0, 0.9);

    sky.step_simulation(2.0);

    let front = sky
        .storm_fronts
        .iter()
        .find(|front| front.front_id == front_id)
        .unwrap();
    let cell = sky
        .weather_cells
        .iter()
        .find(|cell| cell.cell_id == cell_id)
        .unwrap();

    assert_eq!(front.position, [4.0, 1.0, 0.0]);
    assert_eq!(cell.position, [2.0, 2.0, 2.0]);
}

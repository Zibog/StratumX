use stratumx_editor_l9_6_weather_environment_authoring_suite::*;

#[test]
fn test_theater_id_creation() {
    let id = TheaterId::new(42);
    assert_eq!(id.0, 42);
}

#[test]
fn test_theater_id_equality() {
    let id1 = TheaterId::new(1);
    let id2 = TheaterId::new(1);
    let id3 = TheaterId::new(2);
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_storm_front_id_creation() {
    let id = StormFrontId::new(99);
    assert_eq!(id.0, 99);
}

#[test]
fn test_climate_config_new() {
    let config = ClimateConfig::new("Test Theater".to_string(), "/path/to/sky".to_string());
    assert_eq!(config.name, "Test Theater");
    assert_eq!(config.sky_bundle_path, "/path/to/sky");
    assert!(config.enabled);
    assert_eq!(config.base_temperature_celsius, 20.0);
    assert_eq!(config.humidity, 0.5);
}

#[test]
fn test_climate_config_validate_success() {
    let config = ClimateConfig::new("Valid Theater".to_string(), "/valid/path".to_string());
    assert!(config.validate().is_ok());
}

#[test]
fn test_climate_config_validate_empty_name() {
    let config = ClimateConfig {
        name: String::new(),
        sky_bundle_path: "/path".to_string(),
        base_temperature_celsius: 20.0,
        humidity: 0.5,
        wind_base_speed: 5.0,
        precipitation_probability: 0.3,
        cloud_coverage: 0.5,
        fog_density: 0.0,
        time_of_day_hours: 12.0,
        day_of_year: 180,
        latitude_deg: 45.0,
        enabled: true,
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_climate_config_validate_empty_path() {
    let config = ClimateConfig {
        name: "Test".to_string(),
        sky_bundle_path: String::new(),
        base_temperature_celsius: 20.0,
        humidity: 0.5,
        wind_base_speed: 5.0,
        precipitation_probability: 0.3,
        cloud_coverage: 0.5,
        fog_density: 0.0,
        time_of_day_hours: 12.0,
        day_of_year: 180,
        latitude_deg: 45.0,
        enabled: true,
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_climate_config_validate_temperature_range() {
    let mut config = ClimateConfig::new("Test".to_string(), "/path".to_string());
    config.base_temperature_celsius = 150.0; // out of range
    assert!(config.validate().is_err());
}

#[test]
fn test_climate_config_validate_humidity_range() {
    let mut config = ClimateConfig::new("Test".to_string(), "/path".to_string());
    config.humidity = 1.5; // out of range
    assert!(config.validate().is_err());
}

#[test]
fn test_climate_config_validate_wind_range() {
    let mut config = ClimateConfig::new("Test".to_string(), "/path".to_string());
    config.wind_base_speed = 150.0; // out of range
    assert!(config.validate().is_err());
}

#[test]
fn test_climate_config_validate_time_range() {
    let mut config = ClimateConfig::new("Test".to_string(), "/path".to_string());
    config.time_of_day_hours = 30.0; // out of range
    assert!(config.validate().is_err());
}

#[test]
fn test_climate_config_validate_latitude_range() {
    let mut config = ClimateConfig::new("Test".to_string(), "/path".to_string());
    config.latitude_deg = 100.0; // out of range
    assert!(config.validate().is_err());
}

#[test]
fn test_climate_config_validate_day_of_year() {
    let mut config = ClimateConfig::new("Test".to_string(), "/path".to_string());
    config.day_of_year = 400; // out of range
    assert!(config.validate().is_err());
}

#[test]
fn test_storm_params_new() {
    let params = StormParams::new("Test Storm".to_string(), [0.0, 0.0, 0.0]);
    assert_eq!(params.name, "Test Storm");
    assert!(params.active);
    assert_eq!(params.radius, 1000.0);
}

#[test]
fn test_storm_params_validate_success() {
    let params = StormParams::new("Valid Storm".to_string(), [0.0, 0.0, 0.0]);
    assert!(params.validate().is_ok());
}

#[test]
fn test_storm_params_validate_empty_name() {
    let params = StormParams {
        name: String::new(),
        center_position: [0.0, 0.0, 0.0],
        radius: 1000.0,
        intensity: 0.5,
        velocity: [0.0, 0.0, 0.0],
        rain_intensity_mm_per_hour: 10.0,
        wind_speed_multiplier: 1.5,
        lightning_frequency: 0.0,
        active: true,
    };
    assert!(params.validate().is_err());
}

#[test]
fn test_storm_params_validate_negative_radius() {
    let mut params = StormParams::new("Test".to_string(), [0.0, 0.0, 0.0]);
    params.radius = -100.0;
    assert!(params.validate().is_err());
}

#[test]
fn test_storm_params_validate_excessive_radius() {
    let mut params = StormParams::new("Test".to_string(), [0.0, 0.0, 0.0]);
    params.radius = 200000.0; // exceeds 100km limit
    assert!(params.validate().is_err());
}

#[test]
fn test_storm_params_validate_intensity_range() {
    let mut params = StormParams::new("Test".to_string(), [0.0, 0.0, 0.0]);
    params.intensity = 1.5; // out of range
    assert!(params.validate().is_err());
}

#[test]
fn test_storm_params_validate_rain_range() {
    let mut params = StormParams::new("Test".to_string(), [0.0, 0.0, 0.0]);
    params.rain_intensity_mm_per_hour = 300.0; // out of range
    assert!(params.validate().is_err());
}

#[test]
fn test_storm_params_validate_wind_multiplier_range() {
    let mut params = StormParams::new("Test".to_string(), [0.0, 0.0, 0.0]);
    params.wind_speed_multiplier = 15.0; // out of range
    assert!(params.validate().is_err());
}

#[test]
fn test_storm_params_validate_lightning_range() {
    let mut params = StormParams::new("Test".to_string(), [0.0, 0.0, 0.0]);
    params.lightning_frequency = 100.0; // out of range
    assert!(params.validate().is_err());
}

#[test]
fn test_weather_state_new() {
    let state = WeatherState::new();
    assert_eq!(state.temperature_celsius, 20.0);
    assert_eq!(state.humidity, 0.5);
    assert_eq!(state.wind_speed, 5.0);
    assert_eq!(state.cloud_coverage, 0.5);
    assert_eq!(state.visibility_meters, 10000.0);
    assert!(!state.is_raining);
    assert!(!state.lightning_active);
    assert_eq!(state.active_storm_count, 0);
}

#[test]
fn test_weather_environment_suite_new() {
    let suite = WeatherEnvironmentAuthoringSuite::new();
    assert_eq!(suite.theater_count(), 0);
    assert_eq!(suite.storm_front_count(), 0);
    assert!(suite.get_active_theater().is_none());
}

#[test]
fn test_weather_environment_suite_default() {
    let suite = WeatherEnvironmentAuthoringSuite::default();
    assert_eq!(suite.theater_count(), 0);
}

#[test]
fn test_configure_climate_theater_success() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig::new("Test Theater".to_string(), "/sky/path".to_string());
    let result = suite.configure_climate_theater(config);
    assert!(result.is_ok());
    let theater_id = result.unwrap();
    assert_eq!(suite.theater_count(), 1);
    assert!(suite.get_theater(theater_id).is_some());
    // First theater becomes active
    assert_eq!(suite.get_active_theater(), Some(theater_id));
}

#[test]
fn test_configure_climate_theater_invalid_name() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig {
        name: String::new(),
        sky_bundle_path: "/path".to_string(),
        base_temperature_celsius: 20.0,
        humidity: 0.5,
        wind_base_speed: 5.0,
        precipitation_probability: 0.3,
        cloud_coverage: 0.5,
        fog_density: 0.0,
        time_of_day_hours: 12.0,
        day_of_year: 180,
        latitude_deg: 45.0,
        enabled: true,
    };
    let result = suite.configure_climate_theater(config);
    assert!(result.is_err());
}

#[test]
fn test_create_storm_front_success() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig::new("Test Theater".to_string(), "/sky/path".to_string());
    suite.configure_climate_theater(config).unwrap();

    let params = StormParams::new("Test Storm".to_string(), [0.0, 0.0, 0.0]);
    let result = suite.create_storm_front(params);
    assert!(result.is_ok());
    assert_eq!(suite.storm_front_count(), 1);
}

#[test]
fn test_create_storm_front_no_active_theater() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    // No theater configured, but storm front should still be created
    let params = StormParams::new("Test Storm".to_string(), [0.0, 0.0, 0.0]);
    let result = suite.create_storm_front(params);
    assert!(result.is_ok());
}

#[test]
fn test_create_storm_front_invalid_params() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let params = StormParams {
        name: String::new(),
        center_position: [0.0, 0.0, 0.0],
        radius: 1000.0,
        intensity: 0.5,
        velocity: [0.0, 0.0, 0.0],
        rain_intensity_mm_per_hour: 10.0,
        wind_speed_multiplier: 1.5,
        lightning_frequency: 0.0,
        active: true,
    };
    let result = suite.create_storm_front(params);
    assert!(result.is_err());
}

#[test]
fn test_update_storm_front_success() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig::new("Test Theater".to_string(), "/sky/path".to_string());
    suite.configure_climate_theater(config).unwrap();

    let params = StormParams::new("Initial Storm".to_string(), [0.0, 0.0, 0.0]);
    let storm_id = suite.create_storm_front(params).unwrap();

    let new_params = StormParams::new("Updated Storm".to_string(), [10.0, 10.0, 10.0]);
    let result = suite.update_storm_front(storm_id, new_params);
    assert!(result.is_ok());

    let storm = suite.get_storm_front(storm_id).unwrap();
    assert_eq!(storm.params.name, "Updated Storm");
}

#[test]
fn test_update_storm_front_unknown_id() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let unknown_id = StormFrontId::new(999);
    let params = StormParams::new("Test".to_string(), [0.0, 0.0, 0.0]);
    let result = suite.update_storm_front(unknown_id, params);
    assert!(result.is_err());
}

#[test]
fn test_get_weather_state_default() {
    let suite = WeatherEnvironmentAuthoringSuite::new();
    let state = suite.get_weather_state();
    // Default state values
    assert_eq!(state.temperature_celsius, 20.0);
    assert!(!state.is_raining);
}

#[test]
fn test_get_weather_state_with_theater() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig::new("Test Theater".to_string(), "/sky/path".to_string());
    suite.configure_climate_theater(config).unwrap();

    let state = suite.get_weather_state();
    assert_eq!(state.temperature_celsius, 20.0);
    assert_eq!(state.humidity, 0.5);
}

#[test]
fn test_get_weather_state_with_storm() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig::new("Test Theater".to_string(), "/sky/path".to_string());
    suite.configure_climate_theater(config).unwrap();

    let params = StormParams::new("Rain Storm".to_string(), [0.0, 0.0, 0.0]);
    suite.create_storm_front(params).unwrap();

    let state = suite.get_weather_state();
    assert_eq!(state.active_storm_count, 1);
}

#[test]
fn test_set_active_theater_success() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig::new("Theater A".to_string(), "/path_a".to_string());
    let _theater_a = suite.configure_climate_theater(config).unwrap();

    let config2 = ClimateConfig::new("Theater B".to_string(), "/path_b".to_string());
    let theater_b = suite.configure_climate_theater(config2).unwrap();

    let result = suite.set_active_theater(theater_b);
    assert!(result.is_ok());
    assert_eq!(suite.get_active_theater(), Some(theater_b));
}

#[test]
fn test_set_active_theater_unknown() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let unknown_id = TheaterId::new(999);
    let result = suite.set_active_theater(unknown_id);
    assert!(result.is_err());
}

#[test]
fn test_get_theater_storm_fronts() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();
    let config = ClimateConfig::new("Test Theater".to_string(), "/sky/path".to_string());
    let theater_id = suite.configure_climate_theater(config).unwrap();

    let params = StormParams::new("Test Storm".to_string(), [0.0, 0.0, 0.0]);
    let storm_id = suite.create_storm_front(params).unwrap();

    let storm_fronts = suite.get_theater_storm_fronts(theater_id);
    assert_eq!(storm_fronts.len(), 1);
    assert_eq!(storm_fronts[0], storm_id);
}

#[test]
fn test_multiple_theaters_and_storms() {
    let mut suite = WeatherEnvironmentAuthoringSuite::new();

    let config1 = ClimateConfig::new("Theater 1".to_string(), "/path1".to_string());
    let theater1 = suite.configure_climate_theater(config1).unwrap();

    let params1 = StormParams::new("Storm 1".to_string(), [0.0, 0.0, 0.0]);
    suite.create_storm_front(params1).unwrap();

    let config2 = ClimateConfig::new("Theater 2".to_string(), "/path2".to_string());
    let theater2 = suite.configure_climate_theater(config2).unwrap();

    suite.set_active_theater(theater2).unwrap();
    let params2 = StormParams::new("Storm 2".to_string(), [10.0, 10.0, 10.0]);
    suite.create_storm_front(params2).unwrap();

    assert_eq!(suite.theater_count(), 2);
    assert_eq!(suite.storm_front_count(), 2);

    let theater1_storms = suite.get_theater_storm_fronts(theater1);
    let theater2_storms = suite.get_theater_storm_fronts(theater2);
    assert_eq!(theater1_storms.len(), 1);
    assert_eq!(theater2_storms.len(), 1);
}

#[test]
fn test_weather_error_display() {
    let err = WeatherError::InvalidConfiguration("test".to_string());
    let display = format!("{}", err);
    assert!(display.contains("Invalid configuration"));

    let err = WeatherError::UnknownTheater;
    let display = format!("{}", err);
    assert!(display.contains("Unknown climate theater"));

    let err = WeatherError::UnknownStormFront;
    let display = format!("{}", err);
    assert!(display.contains("Unknown storm front"));
}

#[test]
fn test_serialization_theater_id() {
    let id = TheaterId::new(123);
    let json = serde_json::to_string(&id).unwrap();
    let deserialized: TheaterId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, deserialized);
}

#[test]
fn test_serialization_storm_front_id() {
    let id = StormFrontId::new(456);
    let json = serde_json::to_string(&id).unwrap();
    let deserialized: StormFrontId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, deserialized);
}

#[test]
fn test_serialization_climate_config() {
    let config = ClimateConfig::new("Serialize Test".to_string(), "/path".to_string());
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ClimateConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config, deserialized);
}

#[test]
fn test_serialization_storm_params() {
    let params = StormParams::new("Serialize Storm".to_string(), [1.0, 2.0, 3.0]);
    let json = serde_json::to_string(&params).unwrap();
    let deserialized: StormParams = serde_json::from_str(&json).unwrap();
    assert_eq!(params, deserialized);
}

#[test]
fn test_serialization_weather_state() {
    let state = WeatherState::new();
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: WeatherState = serde_json::from_str(&json).unwrap();
    assert_eq!(state, deserialized);
}

#[test]
fn test_serialization_weather_suite() {
    let suite = WeatherEnvironmentAuthoringSuite::new();
    let json = serde_json::to_string(&suite).unwrap();
    let deserialized: WeatherEnvironmentAuthoringSuite = serde_json::from_str(&json).unwrap();
    assert_eq!(suite, deserialized);
}

#[test]
fn test_cell_placement_strategy_random() {
    use weather_scenarios::CellPlacementStrategy;
    let strategy = CellPlacementStrategy::Random {
        center: [0.0, 0.0, 0.0],
        radius_km: 10.0,
    };
    let positions = strategy.generate_positions(5);
    assert_eq!(positions.len(), 5);
}

#[test]
fn test_cell_placement_strategy_grid() {
    use weather_scenarios::CellPlacementStrategy;
    let strategy = CellPlacementStrategy::Grid {
        center: [0.0, 0.0, 0.0],
        spacing_km: 1.0,
    };
    let positions = strategy.generate_positions(4);
    assert_eq!(positions.len(), 4);
}

#[test]
fn test_cell_placement_strategy_line() {
    use weather_scenarios::CellPlacementStrategy;
    let strategy = CellPlacementStrategy::Line {
        start: [0.0, 0.0, 0.0],
        end: [10.0, 0.0, 0.0],
    };
    let positions = strategy.generate_positions(3);
    assert_eq!(positions.len(), 3);
    // First position should be at start
    assert!((positions[0][0] - 0.0).abs() < 0.01);
    // Last position should be at end
    assert!((positions[2][0] - 10.0).abs() < 0.01);
}

#[test]
fn test_cell_placement_strategy_circle() {
    use weather_scenarios::CellPlacementStrategy;
    let strategy = CellPlacementStrategy::Circle {
        center: [0.0, 0.0, 0.0],
        radius_km: 5.0,
    };
    let positions = strategy.generate_positions(4);
    assert_eq!(positions.len(), 4);
}

//! Terrain Sky Roundtrip Tests

#[cfg(test)]
mod tests {
    use stratumx_test_support::*;

    #[test]
    fn test_terrain_import_export() {
        let terrain = create_test_terrain();
        assert!(terrain.world_size[0] > 0.0);
    }

    #[test]
    fn test_sky_time_of_day() {
        let sky = create_sky_at_time(12.0);
        assert_eq!(sky.celestial.time_of_day_hours, 12.0);
    }

    #[test]
    fn test_terrain_sky_binding() {
        let terrain = create_terrain_with_size(512, 512);
        let sky = create_test_sky();

        assert!(terrain.world_size[0] > 0.0);
        assert_eq!(
            sky.weather_director.target_regime,
            engine_material::WeatherRegime::Clear
        );
    }
}

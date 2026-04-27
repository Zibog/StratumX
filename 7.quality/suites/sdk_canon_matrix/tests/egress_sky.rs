//! Egress Sky: проверка экспорта неба

use link_egress_observations::{AssetStatusDto, SkyBundleStatusDto, StormFrontDto};

#[test]
fn export_sky_profile() {
    let dto = SkyBundleStatusDto {
        manifest_loaded: true,
        bundle_id: Some("sky.default".to_string()),
        stars_status: AssetStatusDto::Found,
        moon_albedo_status: AssetStatusDto::Found,
        moon_normal_status: AssetStatusDto::Missing,
        sun_disk_status: AssetStatusDto::Found,
        blue_noise_status: AssetStatusDto::Found,
        noise_source_status: AssetStatusDto::NotRequired,
        noise_source_count: 3,
    };
    let value = serde_json::to_value(dto).unwrap();

    assert!(value.get("manifestLoaded").is_some());
    assert!(value.get("bundleId").is_some());
    assert!(value.get("noiseSourceCount").is_some());
}

#[test]
fn export_weather_data() {
    let dto = StormFrontDto {
        front_id: 4,
        position: [1.0, 2.0, 3.0],
        velocity: [0.5, 0.0, -0.5],
        radius_km: 12.0,
        intensity: 0.8,
        rain_intensity_mm_per_hour: 15.0,
    };
    let roundtrip: StormFrontDto =
        serde_json::from_value(serde_json::to_value(&dto).unwrap()).unwrap();

    assert_eq!(roundtrip.position, dto.position);
    assert_eq!(roundtrip.velocity, dto.velocity);
    assert_eq!(roundtrip.rain_intensity_mm_per_hour, 15.0);
}

#[test]
fn export_lighting_data() {
    let encoded = serde_json::to_string(&AssetStatusDto::NotRequired).unwrap();

    assert_eq!(encoded, "\"notRequired\"");
}

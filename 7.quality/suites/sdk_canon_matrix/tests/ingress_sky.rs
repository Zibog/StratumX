//! Ingress Sky: проверка импорта неба

use engine_material::SkyWeatherState;
use link_ingress_packets::{SkyCommand, StormCommand};

#[test]
fn import_sky_profile() {
    let command = SkyCommand::SetTimeOfDay { hours: 6.5 };
    let decoded: SkyCommand =
        serde_json::from_str(&serde_json::to_string(&command).unwrap()).unwrap();
    let mut sky = SkyWeatherState::new_default();

    match decoded {
        SkyCommand::SetTimeOfDay { hours } => sky.set_time_of_day(hours),
        other => panic!("unexpected command: {other:?}"),
    }

    assert_eq!(sky.celestial.time_of_day_hours, 6.5);
}

#[test]
fn import_celestial_bodies() {
    let command = StormCommand::CreateStormFront {
        position: [0.0, 1.0, 2.0],
        velocity: [3.0, 0.0, -1.0],
        radius_km: 8.0,
        intensity: 0.75,
        rain_intensity_mm_per_hour: 12.0,
    };
    let decoded: StormCommand =
        serde_json::from_value(serde_json::to_value(&command).unwrap()).unwrap();

    match decoded {
        StormCommand::CreateStormFront {
            position,
            velocity,
            radius_km,
            intensity,
            rain_intensity_mm_per_hour,
        } => {
            assert_eq!(position, [0.0, 1.0, 2.0]);
            assert_eq!(velocity, [3.0, 0.0, -1.0]);
            assert_eq!(radius_km, 8.0);
            assert_eq!(intensity, 0.75);
            assert_eq!(rain_intensity_mm_per_hour, 12.0);
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn import_atmosphere_settings() {
    let command = SkyCommand::SetFogDensity { value: 0.35 };
    let decoded: SkyCommand =
        serde_json::from_value(serde_json::to_value(&command).unwrap()).unwrap();
    let mut sky = SkyWeatherState::new_default();

    match decoded {
        SkyCommand::SetFogDensity { value } => sky.set_fog_density(value),
        other => panic!("unexpected command: {other:?}"),
    }

    assert!(sky.atmosphere.fog_density >= 0.0);
    assert!(sky.atmosphere.horizon_visibility_km > 0.0);
}

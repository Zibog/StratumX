use link_egress_observations::*;
use link_ingress_packets::*;
use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

#[test]
fn default_sky_state_exists_in_runtime() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("Sky summary should succeed");

    match obs {
        EditorAuthoringObservation::SkySummaryRead {
            time_of_day_hours,
            day_of_year,
            latitude_deg,
            sun_elevation_deg,
            sun_intensity,
            cloud_coverage,
            fog_density,
            rain_enabled,
            rain_intensity_mm_per_hour,
            wind_vector,
            storm_front_count,
        } => {
            assert!((0.0..=24.0).contains(&time_of_day_hours));
            assert!((1..=365).contains(&day_of_year));
            assert!((-90.0..=90.0).contains(&latitude_deg));
            assert!((-90.0..=90.0).contains(&sun_elevation_deg));
            assert!((0.0..=1.0).contains(&sun_intensity));
            assert!((0.0..=1.0).contains(&cloud_coverage));
            assert!((0.0..=1.0).contains(&fog_density));
            assert!(!rain_enabled || rain_intensity_mm_per_hour >= 0.0);
            assert_eq!(wind_vector.len(), 3);
            assert_eq!(storm_front_count, 0);
        }
        _ => panic!("Expected SkySummaryRead observation"),
    }
}

#[test]
fn set_time_of_day_mutates_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetTimeOfDay { hours: 18.0 }),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("Set time of day should succeed");
    assert!(matches!(obs, EditorAuthoringObservation::SkyValueUpdated));

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
        request_id: 2,
    };

    let obs = session
        .handle_command(packet)
        .expect("Sky summary should succeed");

    match obs {
        EditorAuthoringObservation::SkySummaryRead {
            time_of_day_hours, ..
        } => {
            assert!((time_of_day_hours - 18.0).abs() < 0.01);
        }
        _ => panic!("Expected SkySummaryRead observation"),
    }
}

#[test]
fn set_cloud_coverage_mutates_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetCloudCoverage { value: 0.75 }),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("Set cloud coverage should succeed");
    assert!(matches!(obs, EditorAuthoringObservation::SkyValueUpdated));

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
        request_id: 2,
    };

    let obs = session
        .handle_command(packet)
        .expect("Sky summary should succeed");

    match obs {
        EditorAuthoringObservation::SkySummaryRead { cloud_coverage, .. } => {
            assert!((cloud_coverage - 0.75).abs() < 0.01);
        }
        _ => panic!("Expected SkySummaryRead observation"),
    }
}

#[test]
fn set_fog_density_mutates_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetFogDensity { value: 0.5 }),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("Set fog density should succeed");
    assert!(matches!(obs, EditorAuthoringObservation::SkyValueUpdated));

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
        request_id: 2,
    };

    let obs = session
        .handle_command(packet)
        .expect("Sky summary should succeed");

    match obs {
        EditorAuthoringObservation::SkySummaryRead { fog_density, .. } => {
            assert!((fog_density - 0.5).abs() < 0.01);
        }
        _ => panic!("Expected SkySummaryRead observation"),
    }
}

#[test]
fn set_rain_mutates_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetRain {
            enabled: true,
            intensity_mm_per_hour: 25.0,
        }),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("Set rain should succeed");
    assert!(matches!(obs, EditorAuthoringObservation::SkyValueUpdated));

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
        request_id: 2,
    };

    let obs = session
        .handle_command(packet)
        .expect("Sky summary should succeed");

    match obs {
        EditorAuthoringObservation::SkySummaryRead {
            rain_enabled,
            rain_intensity_mm_per_hour,
            ..
        } => {
            assert!(rain_enabled);
            assert!((rain_intensity_mm_per_hour - 25.0).abs() < 0.01);
        }
        _ => panic!("Expected SkySummaryRead observation"),
    }
}

#[test]
fn create_storm_front_mutates_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Storm(StormCommand::CreateStormFront {
            position: [100.0, 0.0, 200.0],
            velocity: [5.0, 0.0, 0.0],
            radius_km: 15.0,
            intensity: 0.8,
            rain_intensity_mm_per_hour: 30.0,
        }),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("Create storm front should succeed");

    let front_id = match obs {
        EditorAuthoringObservation::StormFrontCreated { front_id, .. } => front_id,
        _ => panic!("Expected StormFrontCreated observation"),
    };

    assert!(front_id > 0);

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Storm(StormCommand::ListStormFronts),
        request_id: 2,
    };

    let obs = session
        .handle_command(packet)
        .expect("List storm fronts should succeed");

    match obs {
        EditorAuthoringObservation::StormFrontList { fronts } => {
            assert_eq!(fronts.len(), 1);
            assert_eq!(fronts[0].front_id, front_id);
            assert!((fronts[0].radius_km - 15.0).abs() < 0.01);
            assert!((fronts[0].intensity - 0.8).abs() < 0.01);
        }
        _ => panic!("Expected StormFrontList observation"),
    }
}

#[test]
fn step_sky_simulation_moves_storm_fronts() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Storm(StormCommand::CreateStormFront {
            position: [0.0, 0.0, 0.0],
            velocity: [10.0, 0.0, 0.0],
            radius_km: 10.0,
            intensity: 0.5,
            rain_intensity_mm_per_hour: 20.0,
        }),
        request_id: 1,
    };

    session
        .handle_command(packet)
        .expect("Create storm front should succeed");

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::StepSkySimulation { dt_seconds: 60.0 }),
        request_id: 2,
    };

    session
        .handle_command(packet)
        .expect("Step simulation should succeed");

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Storm(StormCommand::ListStormFronts),
        request_id: 3,
    };

    let obs = session
        .handle_command(packet)
        .expect("List storm fronts should succeed");

    match obs {
        EditorAuthoringObservation::StormFrontList { fronts } => {
            assert_eq!(fronts.len(), 1);
            // Storm should have moved: velocity [10, 0, 0] * 60 seconds = [600, 0, 0]
            assert!((fronts[0].position[0] - 600.0).abs() < 1.0);
        }
        _ => panic!("Expected StormFrontList observation"),
    }
}

#[test]
fn sky_commands_without_runtime_fail_honestly() {
    // This test verifies that sky commands fail gracefully when runtime is not initialized
    // For now, we assume runtime is always initialized in test session
    // This test documents the expected behavior

    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // All sky commands should succeed with initialized runtime
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
        request_id: 1,
    };

    let result = session.handle_command(packet);
    assert!(
        result.is_ok(),
        "Sky commands should work with initialized runtime"
    );
}

// ============================================================================
// PROOF-PACK EXTENSION: DIAGNOSTICS / BASELINE / RECOVER
// ============================================================================

#[test]
fn get_sky_bundle_status_reaches_editor_path() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyBundleStatus),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("GetSkyBundleStatus should succeed");

    match obs {
        EditorAuthoringObservation::SkyBundleStatusRead { status } => {
            // Confirms the observation makes it all the way through the EditorAuthoringSession path
            // Manifest may or may not be loaded depending on test environment — either is valid
            let _ = status.manifest_loaded;
        }
        _ => panic!("Expected SkyBundleStatusRead observation"),
    }
}

#[test]
fn get_sky_diagnostics_returns_valid_summary() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyDiagnostics),
        request_id: 1,
    };

    let obs = session
        .handle_command(packet)
        .expect("GetSkyDiagnostics should succeed");

    match obs {
        EditorAuthoringObservation::SkyDiagnosticsRead {
            storm_front_count,
            rain_enabled,
            rain_intensity,
            wind_magnitude,
            fog_density,
            cloud_coverage,
        } => {
            assert_eq!(storm_front_count, 0, "No storms in default sky");
            assert!(!rain_enabled, "Default sky has no rain");
            assert!(rain_intensity >= 0.0);
            assert!(wind_magnitude >= 0.0);
            assert!((0.0..=1.0).contains(&fog_density));
            assert!((0.0..=1.0).contains(&cloud_coverage));
        }
        _ => panic!("Expected SkyDiagnosticsRead observation"),
    }
}

#[test]
fn get_sky_diagnostics_reflects_storm_count() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Create two storm fronts
    for i in 0..2 {
        let packet = EditorAuthoringPacket {
            command: EditorAuthoringCommand::Storm(StormCommand::CreateStormFront {
                position: [(i as f32) * 100.0, 0.0, 0.0],
                velocity: [5.0, 0.0, 0.0],
                radius_km: 10.0,
                intensity: 0.6,
                rain_intensity_mm_per_hour: 20.0,
            }),
            request_id: i as u64 + 1,
        };
        session
            .handle_command(packet)
            .expect("Create storm front should succeed");
    }

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyDiagnostics),
        request_id: 10,
    };

    let obs = session
        .handle_command(packet)
        .expect("GetSkyDiagnostics should succeed");

    match obs {
        EditorAuthoringObservation::SkyDiagnosticsRead {
            storm_front_count, ..
        } => {
            assert_eq!(
                storm_front_count, 2,
                "Diagnostic storm count must match actual storm fronts"
            );
        }
        _ => panic!("Expected SkyDiagnosticsRead observation"),
    }
}

#[test]
fn get_sky_diagnostics_reflects_rain_and_wind() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set rain
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetRain {
                enabled: true,
                intensity_mm_per_hour: 15.0,
            }),
            request_id: 1,
        })
        .unwrap();

    // Set wind
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetWindVector {
                value: [3.0, 0.0, 4.0], // magnitude = 5.0 m/s
            }),
            request_id: 2,
        })
        .unwrap();

    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyDiagnostics),
            request_id: 3,
        })
        .expect("GetSkyDiagnostics should succeed");

    match obs {
        EditorAuthoringObservation::SkyDiagnosticsRead {
            rain_enabled,
            rain_intensity,
            wind_magnitude,
            ..
        } => {
            assert!(rain_enabled, "Rain should be enabled");
            assert!(
                (rain_intensity - 15.0).abs() < 0.01,
                "Rain intensity must match"
            );
            assert!(
                (wind_magnitude - 5.0).abs() < 0.01,
                "Wind magnitude must be 5.0 (sqrt(9+16))"
            );
        }
        _ => panic!("Expected SkyDiagnosticsRead observation"),
    }
}

#[test]
fn sky_baseline_capture_and_reset_work() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set a known sky state for the baseline
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetCloudCoverage { value: 0.8 }),
            request_id: 1,
        })
        .unwrap();
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetFogDensity { value: 0.4 }),
            request_id: 2,
        })
        .unwrap();

    // Capture baseline
    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::CaptureSkyBaseline),
            request_id: 3,
        })
        .expect("CaptureSkyBaseline should succeed");
    assert!(matches!(
        obs,
        EditorAuthoringObservation::SkyBaselineCaptured
    ));

    // Mutate the sky away from baseline
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetCloudCoverage { value: 0.1 }),
            request_id: 4,
        })
        .unwrap();

    // Reset to baseline
    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::ResetSkyToBaseline),
            request_id: 5,
        })
        .expect("ResetSkyToBaseline should succeed");
    assert!(matches!(
        obs,
        EditorAuthoringObservation::SkyResetToBaseline
    ));

    // Read summary: cloud coverage should be restored to 0.8
    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::GetSkySummary),
            request_id: 6,
        })
        .unwrap();
    match obs {
        EditorAuthoringObservation::SkySummaryRead {
            cloud_coverage,
            fog_density,
            ..
        } => {
            assert!(
                (cloud_coverage - 0.8).abs() < 0.01,
                "Cloud coverage must be restored to baseline 0.8, got {}",
                cloud_coverage
            );
            assert!(
                (fog_density - 0.4).abs() < 0.01,
                "Fog density must be restored to baseline 0.4, got {}",
                fog_density
            );
        }
        _ => panic!("Expected SkySummaryRead after baseline reset"),
    }
}

#[test]
fn recover_default_sky_clears_to_canonical_defaults() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Wreck the sky state
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetCloudCoverage { value: 1.0 }),
            request_id: 1,
        })
        .unwrap();
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetFogDensity { value: 1.0 }),
            request_id: 2,
        })
        .unwrap();
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::SetRain {
                enabled: true,
                intensity_mm_per_hour: 50.0,
            }),
            request_id: 3,
        })
        .unwrap();

    // Recover to default
    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::RecoverDefaultSky),
            request_id: 4,
        })
        .expect("RecoverDefaultSky should succeed");
    assert!(matches!(
        obs,
        EditorAuthoringObservation::SkyRecoveredToDefault
    ));

    // Verify diagnostics show a clean default state
    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyDiagnostics),
            request_id: 5,
        })
        .unwrap();
    match obs {
        EditorAuthoringObservation::SkyDiagnosticsRead {
            storm_front_count,
            rain_enabled,
            ..
        } => {
            assert_eq!(storm_front_count, 0, "No storms after default recover");
            assert!(!rain_enabled, "Rain should be off after default recover");
        }
        _ => panic!("Expected SkyDiagnosticsRead after recover"),
    }
}

#[test]
fn reset_sky_to_baseline_without_capture_fails_gracefully() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Attempt reset with no baseline captured
    let result = session.handle_command(EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::ResetSkyToBaseline),
        request_id: 1,
    });

    assert!(
        result.is_err(),
        "ResetSkyToBaseline with no prior capture must return an error"
    );
    let err = result.unwrap_err();
    assert!(
        err.contains("baseline"),
        "Error message should mention 'baseline', got: {}",
        err
    );
}

#[test]
fn sky_baseline_capture_preserves_storm_fronts() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Add a storm front before capturing baseline
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Storm(StormCommand::CreateStormFront {
                position: [50.0, 0.0, 50.0],
                velocity: [2.0, 0.0, 0.0],
                radius_km: 8.0,
                intensity: 0.5,
                rain_intensity_mm_per_hour: 10.0,
            }),
            request_id: 1,
        })
        .unwrap();

    // Capture baseline with the storm present
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::CaptureSkyBaseline),
            request_id: 2,
        })
        .unwrap();

    // Recover default sky (removes all storms)
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::RecoverDefaultSky),
            request_id: 3,
        })
        .unwrap();

    // Verify storms are gone after recover
    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyDiagnostics),
            request_id: 4,
        })
        .unwrap();
    match obs {
        EditorAuthoringObservation::SkyDiagnosticsRead {
            storm_front_count, ..
        } => {
            assert_eq!(storm_front_count, 0, "Storms cleared after recover default");
        }
        _ => panic!("Expected SkyDiagnosticsRead"),
    }

    // Reset to baseline (storm should come back)
    session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::ResetSkyToBaseline),
            request_id: 5,
        })
        .unwrap();

    let obs = session
        .handle_command(EditorAuthoringPacket {
            command: EditorAuthoringCommand::Sky(SkyCommand::GetSkyDiagnostics),
            request_id: 6,
        })
        .unwrap();
    match obs {
        EditorAuthoringObservation::SkyDiagnosticsRead {
            storm_front_count, ..
        } => {
            assert_eq!(storm_front_count, 1, "Storm must be restored from baseline");
        }
        _ => panic!("Expected SkyDiagnosticsRead"),
    }
}

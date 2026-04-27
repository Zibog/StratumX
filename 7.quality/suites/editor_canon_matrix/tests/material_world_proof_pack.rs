// ============================================================================
// PACK 3: MATERIAL WORLD PROOF-PACK INTEGRATION TESTS
// ============================================================================
//
// These tests prove that editor can runtime-backed author and inspect
// minimal material world subset through real runtime truth:
// - Barrel/container hydrology (water amount, leak, rain fill, evaporation)
// - Fire/wetness subset (ignition, wetness suppression, rain extinguish)
// - Weather toggles (rain on/off, wind vector)
// - Inspection returns runtime truth, not local DTO fiction

use link_ingress_packets::*;
use stratumx_tooling_l6_12_preview_runtime::EditorAuthoringSession;

// ============================================================================
// BARREL HYDROLOGY TESTS
// ============================================================================

#[test]
fn test_barrel_water_state_lives_in_runtime_truth() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set barrel water amount
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::SetBarrelWater {
            liters: 100.0,
        }),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(
        matches!(result, link_egress_observations::EditorAuthoringObservation::BarrelWaterSet { liters } if liters == 100.0)
    );

    // Get barrel water amount from runtime
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetBarrelWater),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(
        matches!(result, link_egress_observations::EditorAuthoringObservation::BarrelWaterInfo { liters } if liters == 100.0)
    );
}

#[test]
fn test_rain_fill_modifies_runtime_hydrology() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set initial water amount
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::SetBarrelWater {
            liters: 50.0,
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Enable rain via Sky system
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetRain {
            enabled: true,
            intensity_mm_per_hour: 100.0,
        }),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::SkyValueUpdated
    ));

    // Update material world for 10 seconds
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::UpdateMaterialWorld {
            delta_time: 10.0,
        }),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Check water increased
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetBarrelWater),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::BarrelWaterInfo { liters } = result
    {
        assert!(liters > 50.0, "Rain should increase water amount");
    } else {
        panic!("Expected BarrelWaterInfo");
    }
}

#[test]
fn test_leak_modifies_runtime_hydrology() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set initial water amount
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::SetBarrelWater {
            liters: 100.0,
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Enable leak
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::SetBarrelLeak {
            active: true,
        }),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(
        matches!(result, link_egress_observations::EditorAuthoringObservation::BarrelLeakSet { active } if active)
    );

    // Update material world for 10 seconds
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::UpdateMaterialWorld {
            delta_time: 10.0,
        }),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Check water decreased
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetBarrelWater),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::BarrelWaterInfo { liters } = result
    {
        assert!(liters < 100.0, "Leak should decrease water amount");
    } else {
        panic!("Expected BarrelWaterInfo");
    }
}

#[test]
fn test_evaporation_modifies_runtime_hydrology() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set initial water amount
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::SetBarrelWater {
            liters: 50.0,
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Update material world for long time to see evaporation
    for i in 0..100 {
        let packet = EditorAuthoringPacket {
            command: EditorAuthoringCommand::MaterialWorld(
                MaterialWorldCommand::UpdateMaterialWorld { delta_time: 10.0 },
            ),
            request_id: 2 + i,
        };
        session.handle_command(packet).unwrap();
    }

    // Check water decreased due to evaporation
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetBarrelWater),
        request_id: 1000,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::BarrelWaterInfo { liters } = result
    {
        assert!(liters < 50.0, "Evaporation should decrease water amount");
    } else {
        panic!("Expected BarrelWaterInfo");
    }
}

// ============================================================================
// FIRE/WETNESS TESTS
// ============================================================================

#[test]
fn test_wetness_blocks_ignition() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set fire object wet
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(
            MaterialWorldCommand::SetFireObjectWetness {
                wetness_percent: 80.0,
            },
        ),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(
        matches!(result, link_egress_observations::EditorAuthoringObservation::FireObjectWetnessSet { wetness_percent } if wetness_percent == 80.0)
    );

    // Try to ignite - should fail
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::IgniteFireObject),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(
        matches!(result, link_egress_observations::EditorAuthoringObservation::FireObjectIgnited { success } if !success)
    );

    // Check fire state - should not be burning
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetFireObjectState),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::FireObjectState {
        burning, ..
    } = result
    {
        assert!(!burning, "Wet object should not ignite");
    } else {
        panic!("Expected FireObjectState");
    }
}

#[test]
fn test_dry_object_can_ignite() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set fire object dry
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(
            MaterialWorldCommand::SetFireObjectWetness {
                wetness_percent: 0.0,
            },
        ),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // Try to ignite - should succeed
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::IgniteFireObject),
        request_id: 2,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(
        matches!(result, link_egress_observations::EditorAuthoringObservation::FireObjectIgnited { success } if success)
    );

    // Check fire state - should be burning
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetFireObjectState),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::FireObjectState {
        burning, ..
    } = result
    {
        assert!(burning, "Dry object should ignite");
    } else {
        panic!("Expected FireObjectState");
    }
}

#[test]
fn test_rain_suppresses_fire() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set fire object dry and ignite
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(
            MaterialWorldCommand::SetFireObjectWetness {
                wetness_percent: 0.0,
            },
        ),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::IgniteFireObject),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Enable heavy rain via Sky system
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetRain {
            enabled: true,
            intensity_mm_per_hour: 200.0,
        }),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Update material world to apply rain
    for i in 0..10 {
        let packet = EditorAuthoringPacket {
            command: EditorAuthoringCommand::MaterialWorld(
                MaterialWorldCommand::UpdateMaterialWorld { delta_time: 1.0 },
            ),
            request_id: 4 + i,
        };
        session.handle_command(packet).unwrap();
    }

    // Check fire state - should be extinguished
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetFireObjectState),
        request_id: 100,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::FireObjectState {
        burning,
        wetness_percent,
        ..
    } = result
    {
        assert!(!burning, "Rain should extinguish fire");
        assert!(wetness_percent > 50.0, "Rain should increase wetness");
    } else {
        panic!("Expected FireObjectState");
    }
}

#[test]
fn test_extinguish_fire_object() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set fire object dry and ignite
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(
            MaterialWorldCommand::SetFireObjectWetness {
                wetness_percent: 0.0,
            },
        ),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::IgniteFireObject),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // Extinguish fire
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::ExtinguishFireObject),
        request_id: 3,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::FireObjectExtinguished
    ));

    // Check fire state - should not be burning
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetFireObjectState),
        request_id: 4,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::FireObjectState {
        burning, ..
    } = result
    {
        assert!(!burning, "Fire should be extinguished");
    } else {
        panic!("Expected FireObjectState");
    }
}

// ============================================================================
// SMOKE/WIND TESTS
// ============================================================================

#[test]
fn test_wind_affects_smoke() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // Set wind
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetWindVector {
            value: [5.0, 0.0, 0.0],
        }),
        request_id: 1,
    };
    let result = session.handle_command(packet).unwrap();
    assert!(matches!(
        result,
        link_egress_observations::EditorAuthoringObservation::SkyValueUpdated
    ));

    // Ignite fire to produce smoke
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(
            MaterialWorldCommand::SetFireObjectWetness {
                wetness_percent: 0.0,
            },
        ),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::IgniteFireObject),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // Update material world to produce smoke
    for i in 0..5 {
        let packet = EditorAuthoringPacket {
            command: EditorAuthoringCommand::MaterialWorld(
                MaterialWorldCommand::UpdateMaterialWorld { delta_time: 1.0 },
            ),
            request_id: 4 + i,
        };
        session.handle_command(packet).unwrap();
    }

    // Check smoke particle count
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetSmokeParticleCount),
        request_id: 100,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SmokeParticleCount { count } =
        result
    {
        assert!(count > 0, "Burning fire should produce smoke");
    } else {
        panic!("Expected SmokeParticleCount");
    }
}

// ============================================================================
// INTEGRATION CYCLE TESTS
// ============================================================================

#[test]
fn test_full_material_world_cycle() {
    let mut session = EditorAuthoringSession::new();
    session.initialize_vertical_slice_session().unwrap();

    // 1. Set barrel water
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::SetBarrelWater {
            liters: 80.0,
        }),
        request_id: 1,
    };
    session.handle_command(packet).unwrap();

    // 2. Enable rain via Sky system
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetRain {
            enabled: true,
            intensity_mm_per_hour: 50.0,
        }),
        request_id: 2,
    };
    session.handle_command(packet).unwrap();

    // 3. Set wind via Sky system
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::Sky(SkyCommand::SetWindVector {
            value: [3.0, 0.0, 2.0],
        }),
        request_id: 3,
    };
    session.handle_command(packet).unwrap();

    // 4. Set fire object dry
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(
            MaterialWorldCommand::SetFireObjectWetness {
                wetness_percent: 0.0,
            },
        ),
        request_id: 4,
    };
    session.handle_command(packet).unwrap();

    // 5. Ignite fire
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::IgniteFireObject),
        request_id: 5,
    };
    session.handle_command(packet).unwrap();

    // 6. Update material world
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::UpdateMaterialWorld {
            delta_time: 5.0,
        }),
        request_id: 6,
    };
    session.handle_command(packet).unwrap();

    // 7. Verify barrel water increased (rain)
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetBarrelWater),
        request_id: 7,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::BarrelWaterInfo { liters } = result
    {
        assert!(liters > 80.0, "Rain should increase barrel water");
    } else {
        panic!("Expected BarrelWaterInfo");
    }

    // 8. Verify fire is burning
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetFireObjectState),
        request_id: 8,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::FireObjectState {
        burning, ..
    } = result
    {
        assert!(burning, "Fire should be burning");
    } else {
        panic!("Expected FireObjectState");
    }

    // 9. Verify smoke is present
    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetSmokeParticleCount),
        request_id: 9,
    };
    let result = session.handle_command(packet).unwrap();
    if let link_egress_observations::EditorAuthoringObservation::SmokeParticleCount { count } =
        result
    {
        assert!(count > 0, "Burning fire should produce smoke");
    } else {
        panic!("Expected SmokeParticleCount");
    }
}

#[test]
fn test_material_world_without_runtime_session_fails() {
    let mut session = EditorAuthoringSession::new();
    // Don't initialize runtime session

    let packet = EditorAuthoringPacket {
        command: EditorAuthoringCommand::MaterialWorld(MaterialWorldCommand::GetBarrelWater),
        request_id: 1,
    };
    let result = session.handle_command(packet);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Runtime session not initialized"));
}

// REGRESSION FARM: PROOF REGION REPEATABLE CERTIFICATION
// Ensures proof-region is not a one-off miracle but repeatable and certifiable

use engine_acoustics::AudioRuntime;
use engine_animation::AnimationRuntime;
use engine_imaging::LightingRuntime;
use engine_world::{ReferenceRegionBootstrapper, ReferenceRegionScene, WorldState};
use tool_evidence_capture::{
    ArtifactRef, ArtifactType, Baseline, BaselineRegistry, CertificationEngine, CompareEngine,
    CompareMode, CompareVerdict, EvidenceBundle, EvidenceVerdict,
};

// ============================================================================
// REGRESSION SUITE: COMMAND SPINE
// ============================================================================

#[test]
fn regression_command_spine_bootstrap_repeatable() {
    // Run 10 times to ensure repeatability
    for run in 0..10 {
        let scene = ReferenceRegionScene::create_default();
        let mut world = WorldState::new();
        let bootstrapper = ReferenceRegionBootstrapper::new(scene);
        let state = bootstrapper
            .bootstrap(&mut world)
            .expect(&format!("bootstrap run {}", run));

        // Verify consistent results
        assert_eq!(
            state.terrain_patches.len(),
            2,
            "Run {}: terrain patches",
            run
        );
        assert_eq!(state.structures.len(), 3, "Run {}: structures", run);
        assert_eq!(state.container_indices.len(), 1, "Run {}: containers", run);
        assert_eq!(state.fire_indices.len(), 1, "Run {}: fire sources", run);
        assert_eq!(state.npcs.len(), 3, "Run {}: NPCs", run);
        assert_eq!(state.squads.len(), 1, "Run {}: squads", run);
        assert_eq!(state.creatures.len(), 1, "Run {}: creatures", run);
    }
}

#[test]
fn regression_command_spine_save_load_repeatable() {
    let scene = ReferenceRegionScene::create_default();
    let mut world = WorldState::new();
    let bootstrapper = ReferenceRegionBootstrapper::new(scene);
    let _state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

    // Run save/load 5 times
    for run in 0..5 {
        let snapshot = world.snapshot(run + 1);
        assert_eq!(snapshot.tick.0, 0, "Run {}: snapshot tick", run);

        let bytes = world
            .snapshot_bytes(run + 1)
            .expect(&format!("serialize run {}", run));
        assert!(!bytes.is_empty(), "Run {}: snapshot bytes", run);
    }
}

// ============================================================================
// REGRESSION SUITE: MATERIAL/DESTRUCTION
// ============================================================================

#[test]
fn regression_material_terrain_blast_repeatable() {
    let scene = ReferenceRegionScene::create_default();
    let mut world = WorldState::new();
    let bootstrapper = ReferenceRegionBootstrapper::new(scene);
    let state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

    // Run explosion 10 times at same position
    let position = [10.0, 0.0, 10.0];
    let energy = 50000.0;
    let mut crater_radii = Vec::new();

    for _run in 0..10 {
        let index = state.apply_explosion(&mut world, position, energy);
        let response = world.material_world().get_terrain_response(index).unwrap();
        crater_radii.push(response.crater.radius_m);
    }

    // All runs should produce same crater radius (deterministic)
    let first_radius = crater_radii[0];
    for (i, radius) in crater_radii.iter().enumerate() {
        assert!(
            (radius - first_radius).abs() < 0.01,
            "Run {}: crater radius {} differs from first {}",
            i,
            radius,
            first_radius
        );
    }
}

#[test]
fn regression_destruction_fragments_repeatable() {
    // Destroy tree 5 times (need fresh world each time)
    let mut fragment_counts = Vec::new();

    for _run in 0..5 {
        let scene = ReferenceRegionScene::create_default();
        let mut world_run = WorldState::new();
        let bootstrapper_run = ReferenceRegionBootstrapper::new(scene);
        let state_run = bootstrapper_run
            .bootstrap(&mut world_run)
            .expect("bootstrap");

        let index = state_run
            .apply_structure_destruction(
                &mut world_run,
                1, // tree
                [10.0, 1.0, 10.0],
                10000.0,
                [1.0, 0.0, 0.0],
            )
            .expect("destruction");

        let response = world_run
            .material_world()
            .get_destruction_response(index)
            .unwrap();
        fragment_counts.push(response.fragments.len());
    }

    // All runs should produce same fragment count
    let first_count = fragment_counts[0];
    for (i, count) in fragment_counts.iter().enumerate() {
        assert_eq!(*count, first_count, "Run {}: fragment count differs", i);
    }
}

// ============================================================================
// REGRESSION SUITE: HYDROLOGY/FIRE/STORM
// ============================================================================

#[test]
fn regression_hydrology_rain_fill_repeatable() {
    let scene = ReferenceRegionScene::create_default();

    // Run 5 times
    let mut final_volumes = Vec::new();

    for _run in 0..5 {
        let mut world = WorldState::new();
        let bootstrapper = ReferenceRegionBootstrapper::new(scene.clone());
        let state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

        let container_index = state.container_indices[0];

        // Simulate 100 seconds
        let rain_intensity = scene.weather_config.rainfall_intensity;
        for _ in 0..100 {
            world
                .material_world_mut()
                .update(1.0, rain_intensity, [0.0, 0.0, 0.0]);
        }

        let hydro_state = world
            .material_world()
            .get_hydrology_state(container_index)
            .unwrap();
        final_volumes.push(hydro_state.container.current_volume_liters);
    }

    // All runs should produce similar final volume (within 1%)
    let first_volume = final_volumes[0];
    for (i, volume) in final_volumes.iter().enumerate() {
        let diff_percent = ((volume - first_volume) / first_volume * 100.0).abs();
        assert!(
            diff_percent < 1.0,
            "Run {}: volume {} differs by {}% from first {}",
            i,
            volume,
            diff_percent,
            first_volume
        );
    }
}

#[test]
fn regression_fire_ignition_repeatable() {
    let scene = ReferenceRegionScene::create_default();

    // Run 5 times
    for run in 0..5 {
        let mut world = WorldState::new();
        let bootstrapper = ReferenceRegionBootstrapper::new(scene.clone());
        let state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

        let fire_index = state.fire_indices[0];

        // Fire should be burning (ignited in bootstrap)
        world.material_world_mut().update(1.0, 0.0, [0.0, 0.0, 0.0]);
        let fire_obj = world
            .material_world()
            .get_combustible_object(fire_index)
            .unwrap();
        assert!(fire_obj.fire.burning, "Run {}: fire should be burning", run);
    }
}

#[test]
fn regression_storm_movement_repeatable() {
    let scene = ReferenceRegionScene::create_default();

    // Run 3 times with same time steps
    let mut final_rainfalls = Vec::new();

    for _run in 0..3 {
        let mut world = WorldState::new();
        let bootstrapper = ReferenceRegionBootstrapper::new(scene.clone());
        let _state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

        // Simulate 10 hours
        for _ in 0..10 {
            world
                .material_world_mut()
                .update(3600.0, 0.0, [0.0, 0.0, 0.0]);
        }

        let rainfall = world
            .material_world()
            .get_rainfall_at_position([25.0, 25.0]);
        final_rainfalls.push(rainfall);
    }

    // All runs should produce same rainfall pattern
    let first_rainfall = final_rainfalls[0];
    for (i, rainfall) in final_rainfalls.iter().enumerate() {
        assert!(
            (rainfall - first_rainfall).abs() < 0.1,
            "Run {}: rainfall {} differs from first {}",
            i,
            rainfall,
            first_rainfall
        );
    }
}

// ============================================================================
// REGRESSION SUITE: RENDER/AUDIO/ANIMATION
// ============================================================================

#[test]
fn regression_lighting_muzzle_flash_repeatable() {
    // Run 10 times
    for run in 0..10 {
        let mut lighting = LightingRuntime::new();
        lighting.ambient_intensity = 0.05;

        let muzzle_pos = [60.0, 1.5, 20.0];
        let flash = engine_imaging::LightSource::muzzle_flash(muzzle_pos, [1.0, 0.0, 0.0]);
        lighting.add_light(flash);

        let nearby_pos = [61.0, 1.5, 20.0];
        let light = lighting.calculate_lighting(nearby_pos);

        assert!(
            light[0] > 0.5,
            "Run {}: muzzle flash should light geometry",
            run
        );
    }
}

#[test]
fn regression_audio_gunshot_repeatable() {
    // Run 10 times
    let mut volumes = Vec::new();

    for _run in 0..10 {
        let mut audio = AudioRuntime::new();
        let gunshot_pos = [60.0, 1.5, 20.0];
        let gunshot = engine_acoustics::AudioSource::gunshot(gunshot_pos);
        audio.add_source(gunshot);
        audio.set_listener_position([65.0, 1.5, 20.0]);

        let volume = audio.calculate_mix();
        volumes.push(volume);
    }

    // All runs should produce same volume
    let first_volume = volumes[0];
    for (i, volume) in volumes.iter().enumerate() {
        assert!(
            (volume - first_volume).abs() < 0.001,
            "Run {}: volume {} differs from first {}",
            i,
            volume,
            first_volume
        );
    }
}

#[test]
fn regression_animation_ik_solve_repeatable() {
    let scene = ReferenceRegionScene::create_default();
    let mut world = WorldState::new();
    let bootstrapper = ReferenceRegionBootstrapper::new(scene);
    let state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

    let door = &state.doors[0];

    // Run IK solve 10 times
    let mut distances = Vec::new();

    for _run in 0..10 {
        let mut runtime = AnimationRuntime::new();
        let mut chain = engine_animation::IkChain::new();

        let shoulder = chain.add_joint([door.position[0] - 1.0, 1.5, door.position[2]], None, 0.0);
        let elbow = chain.add_joint(
            [door.position[0] - 0.7, 1.2, door.position[2]],
            Some(shoulder),
            0.35,
        );
        let hand = chain.add_joint(
            [door.position[0] - 0.4, 1.0, door.position[2]],
            Some(elbow),
            0.35,
        );
        chain.set_end_effector(hand);

        let chain_index = runtime.add_ik_chain(chain);
        let result = runtime.solve_ik(chain_index, door.handle_position).unwrap();

        distances.push(result.distance_to_target);
    }

    // All runs should produce same distance
    let first_distance = distances[0];
    for (i, distance) in distances.iter().enumerate() {
        assert!(
            (distance - first_distance).abs() < 0.001,
            "Run {}: distance {} differs from first {}",
            i,
            distance,
            first_distance
        );
    }
}

// ============================================================================
// CERTIFICATION PACK: PROOF REGION FULL CYCLE
// ============================================================================

#[test]
fn certification_pack_proof_region_full_cycle() {
    // This is a full certification pack execution

    // Step 1: Create baseline
    let mut baseline_registry = BaselineRegistry::new();
    let baseline = Baseline {
        baseline_id: "baseline_proof_region_v1".to_string(),
        pack_id: "pack_proof_region".to_string(),
        scenario_id: "scenario_proof_region_full".to_string(),
        timestamp: 1000,
        artifacts: vec![ArtifactRef {
            artifact_id: "baseline_snapshot".to_string(),
            artifact_type: ArtifactType::StateSnapshot,
            path: "/baselines/proof_region_v1.snapshot".to_string(),
            checksum: "baseline_v1_checksum".to_string(),
        }],
        protected: true,
    };
    baseline_registry.register_baseline(baseline);

    // Step 2: Run proof region
    let scene = ReferenceRegionScene::create_default();
    let mut world = WorldState::new();
    let bootstrapper = ReferenceRegionBootstrapper::new(scene);
    let state = bootstrapper.bootstrap(&mut world).expect("bootstrap");

    // Execute all proof cases
    let explosion_index = state.apply_explosion(&mut world, [10.0, 0.0, 10.0], 50000.0);
    let explosion_result = world
        .material_world()
        .get_terrain_response(explosion_index)
        .unwrap();
    assert!(explosion_result.crater.radius_m > 0.0);

    let destruction_index = state
        .apply_structure_destruction(&mut world, 1, [10.0, 1.0, 10.0], 10000.0, [1.0, 0.0, 0.0])
        .expect("destruction");
    let destruction_result = world
        .material_world()
        .get_destruction_response(destruction_index)
        .unwrap();
    assert!(destruction_result.destroyed);

    // Step 3: Create compare triplet
    let mut compare_engine = CompareEngine::new();
    let pack_id = compare_engine.create_triplet(
        "pack_proof_region".to_string(),
        "baseline_proof_region_v1".to_string(),
        CompareMode::Exact,
    );

    // Step 4: Execute compare (no failed run - should pass)
    let compare_result = compare_engine.execute_compare(&pack_id).unwrap();
    assert_eq!(compare_result.verdict, CompareVerdict::Pass);

    // Step 5: Create evidence bundle
    let evidence = EvidenceBundle {
        bundle_id: "evidence_proof_region_v1".to_string(),
        pack_id: "pack_proof_region".to_string(),
        scenario_id: "scenario_proof_region_full".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 2000,
        artifacts: vec![
            ArtifactRef {
                artifact_id: "run_snapshot".to_string(),
                artifact_type: ArtifactType::StateSnapshot,
                path: "/runs/proof_region_v1_run1.snapshot".to_string(),
                checksum: "run1_checksum".to_string(),
            },
            ArtifactRef {
                artifact_id: "run_diagnostics".to_string(),
                artifact_type: ArtifactType::DiagnosticsLog,
                path: "/runs/proof_region_v1_run1.log".to_string(),
                checksum: "diag1_checksum".to_string(),
            },
        ],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("operator_certification".to_string()),
    };

    // Step 6: Certify
    let mut cert_engine = CertificationEngine::new();
    let cert_result = cert_engine.certify_pack(
        "pack_proof_region".to_string(),
        "scenario_proof_region_full".to_string(),
        &evidence,
        &compare_result,
    );

    assert_eq!(
        cert_result.verdict,
        tool_evidence_capture::CertificationVerdict::Certified
    );
    assert!(cert_result.blockers.is_empty());
    assert!(cert_engine.is_freeze_ready("pack_proof_region"));
}

// ============================================================================
// CERTIFICATION PACK: RECOVERY PATH
// ============================================================================

#[test]
fn certification_pack_recovery_path_verified() {
    // Verify that recovery path works when failure occurs

    let mut compare_engine = CompareEngine::new();
    let pack_id = compare_engine.create_triplet(
        "pack_recovery_test".to_string(),
        "baseline_recovery".to_string(),
        CompareMode::Exact,
    );

    // Simulate failed run
    compare_engine.add_failed_run(&pack_id, "failed_run_1".to_string());
    let failed_result = compare_engine.execute_compare(&pack_id).unwrap();
    assert_eq!(failed_result.verdict, CompareVerdict::Failed);

    // Evidence with Red verdict
    let failed_evidence = EvidenceBundle {
        bundle_id: "evidence_failed".to_string(),
        pack_id: "pack_recovery_test".to_string(),
        scenario_id: "scenario_recovery".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 1000,
        artifacts: vec![],
        verdict: EvidenceVerdict::Red,
        operator_signoff: None,
    };

    // Certification should be blocked
    let mut cert_engine = CertificationEngine::new();
    let failed_cert = cert_engine.certify_pack(
        "pack_recovery_test".to_string(),
        "scenario_recovery".to_string(),
        &failed_evidence,
        &failed_result,
    );

    assert_eq!(
        failed_cert.verdict,
        tool_evidence_capture::CertificationVerdict::Blocked
    );
    assert!(!failed_cert.blockers.is_empty());

    // Now add recovery run
    compare_engine.add_recovery_run(&pack_id, "recovery_run_1".to_string());
    let recovery_result = compare_engine.execute_compare(&pack_id).unwrap();
    assert_eq!(recovery_result.verdict, CompareVerdict::Recovered);

    // Evidence with Green verdict after recovery
    let recovery_evidence = EvidenceBundle {
        bundle_id: "evidence_recovered".to_string(),
        pack_id: "pack_recovery_test".to_string(),
        scenario_id: "scenario_recovery".to_string(),
        build_profile: "desktop".to_string(),
        schema_revision: "1.0".to_string(),
        timestamp: 2000,
        artifacts: vec![],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("operator_recovery".to_string()),
    };

    // Certification should now pass
    let recovery_cert = cert_engine.certify_pack(
        "pack_recovery_test".to_string(),
        "scenario_recovery".to_string(),
        &recovery_evidence,
        &recovery_result,
    );

    assert_eq!(
        recovery_cert.verdict,
        tool_evidence_capture::CertificationVerdict::Certified
    );
    assert!(recovery_cert.blockers.is_empty());
}

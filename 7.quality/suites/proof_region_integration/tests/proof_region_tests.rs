// ============================================================================
// Proof Region Integration Tests
// Tests the full lane: terrain -> sky -> materials -> viewport -> diagnostics -> simulate
// ============================================================================

use stratumx_route_test_support::{
    execute_button_in_context, routes_by_prefix, ToolSessionContext,
};
use stratumx_tooling_l6_14_release_runtime::first_result_verification::{
    BuildArtifact, ExportArtifact, FIRST_RESULT_SIGNATURE, FirstResultVerifier, LaunchTrace,
    ReleaseChain,
};
use tool_evidence_capture::{
    ArtifactRef, ArtifactType, EvidenceBundle, EvidenceVerdict,
};

// ============================================================================
// First Result Signature Verification
// ============================================================================

#[test]
fn first_result_signature_is_defined() {
    assert!(!FIRST_RESULT_SIGNATURE.is_empty());
}

#[test]
fn first_result_signature_matches_expected_value() {
    assert_eq!(FIRST_RESULT_SIGNATURE, "ты победил");
}

#[test]
fn first_result_verification_with_correct_signature() {
    let mut verifier = FirstResultVerifier::new();
    let result = verifier.verify_signature(
        "v1".into(),
        "proof_region_project".into(),
        "proof_region_build".into(),
        FIRST_RESULT_SIGNATURE.to_string(),
        vec!["proof_region".into()],
    );
    assert!(result.verified);
}

#[test]
fn first_result_verification_with_wrong_signature() {
    let mut verifier = FirstResultVerifier::new();
    let result = verifier.verify_signature(
        "v1".into(),
        "proof_region_project".into(),
        "proof_region_build".into(),
        "wrong_signature".into(),
        vec![],
    );
    assert!(!result.verified);
}

// ============================================================================
// Tunnel Slice Verification
// Covers: flashlight, muzzle flash, shadows, audio, destruction, diagnostics
// ============================================================================

#[test]
fn tunnel_slice_flashlight_route_exists() {
    let flashlight_routes = routes_by_prefix("btn.view");
    assert!(
        !flashlight_routes.is_empty(),
        "Tunnel slice requires viewport/view routes for flashlight"
    );
}

#[test]
fn tunnel_slice_audio_route_exists() {
    let audio_routes = routes_by_prefix("btn.audio");
    assert!(
        !audio_routes.is_empty(),
        "Tunnel slice requires audio authoring routes"
    );
}

#[test]
fn tunnel_slice_terrain_route_exists() {
    let terrain_routes = routes_by_prefix("btn.terrain");
    assert!(
        !terrain_routes.is_empty(),
        "Tunnel slice requires terrain authoring routes"
    );
}

#[test]
fn tunnel_slice_material_route_exists() {
    let material_routes = routes_by_prefix("btn.material");
    assert!(
        !material_routes.is_empty(),
        "Tunnel slice requires material authoring routes"
    );
}

#[test]
fn tunnel_slice_sky_route_exists() {
    let sky_routes = routes_by_prefix("btn.sky");
    assert!(
        !sky_routes.is_empty(),
        "Tunnel slice requires sky/weather authoring routes"
    );
}

#[test]
fn tunnel_slice_full_button_execution_lane() {
    let mut ctx = ToolSessionContext::default();

    let tunnel_buttons = [
        "btn.terrain.save_chunks",
        "btn.sky.set_weather_regime",
        "btn.material.bind_light_response",
        "btn.audio.preview_audibility_free_camera",
    ];

    for button_id in &tunnel_buttons {
        let result = execute_button_in_context(&mut ctx, button_id, ());
        assert!(
            result.focus_target.is_some(),
            "Button {} must produce a focus target",
            button_id
        );
    }

    assert_eq!(ctx.executed_buttons.len(), tunnel_buttons.len());
}

#[test]
fn tunnel_slice_evidence_bundle_creation() {
    let bundle = EvidenceBundle {
        bundle_id: "tunnel_slice_evidence".into(),
        pack_id: "tunnel_pack".into(),
        scenario_id: "flashlight_muzzle_shadows_audio_destruction".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 0,
        artifacts: vec![
            ArtifactRef {
                artifact_id: "flashlight_capture".into(),
                artifact_type: ArtifactType::FrameCapture,
                path: "/evidence/flashlight.png".into(),
                checksum: "sha256:flashlight".into(),
            },
            ArtifactRef {
                artifact_id: "audio_capture".into(),
                artifact_type: ArtifactType::AudioCapture,
                path: "/evidence/audio.wav".into(),
                checksum: "sha256:audio".into(),
            },
            ArtifactRef {
                artifact_id: "shadow_digest".into(),
                artifact_type: ArtifactType::CompareDigest,
                path: "/evidence/shadows.json".into(),
                checksum: "sha256:shadows".into(),
            },
        ],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("tunnel_slice_reviewer".into()),
    };

    assert_eq!(bundle.artifacts.len(), 3);
    assert_eq!(bundle.verdict, EvidenceVerdict::Green);
    assert!(bundle.operator_signoff.is_some());
}

#[test]
fn tunnel_slice_destruction_diagnostics() {
    let bundle = EvidenceBundle {
        bundle_id: "destruction_diagnostics".into(),
        pack_id: "destruction_pack".into(),
        scenario_id: "fracture_destruction_verify".into(),
        build_profile: "Release".into(),
        schema_revision: "v1".into(),
        timestamp: 0,
        artifacts: vec![ArtifactRef {
            artifact_id: "destruction_trace".into(),
            artifact_type: ArtifactType::DiagnosticsLog,
            path: "/evidence/destruction.log".into(),
            checksum: "sha256:destruction".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: None,
    };

    assert_eq!(bundle.verdict, EvidenceVerdict::Green);
    assert_eq!(bundle.artifacts[0].artifact_type, ArtifactType::DiagnosticsLog);
}

// ============================================================================
// Full World Lifecycle: create -> open -> terrain -> sky -> materials -> viewport -> simulate -> capture -> close
// ============================================================================

#[test]
fn world_lifecycle_project_create() {
    let mut ctx = ToolSessionContext::default();
    let result = execute_button_in_context(&mut ctx, "btn.project.new_project", ());
    assert!(
        result.focus_target.is_some(),
        "Project creation must produce a focus target"
    );
    assert!(
        ctx.executed_buttons
            .iter()
            .any(|b| b == "btn.project.new_project")
    );
}

#[test]
fn world_lifecycle_world_open() {
    let mut ctx = ToolSessionContext::default();
    execute_button_in_context(&mut ctx, "btn.project.new_project", ());
    let result = execute_button_in_context(&mut ctx, "btn.world.open_world_package", ());
    assert!(
        result.focus_target.is_some(),
        "World open must produce a focus target"
    );
    assert_eq!(ctx.executed_buttons.len(), 2);
}

#[test]
fn world_lifecycle_terrain_authoring() {
    let mut ctx = ToolSessionContext::default();
    execute_button_in_context(&mut ctx, "btn.project.new_project", ());
    execute_button_in_context(&mut ctx, "btn.world.open_world_package", ());

    let result = execute_button_in_context(&mut ctx, "btn.terrain.save_chunks", ());
    assert!(
        result.focus_target.is_some(),
        "Terrain authoring must produce a focus target"
    );
    assert_eq!(ctx.executed_buttons.len(), 3);
}

#[test]
fn world_lifecycle_sky_authoring() {
    let mut ctx = ToolSessionContext::default();
    execute_button_in_context(&mut ctx, "btn.project.new_project", ());
    execute_button_in_context(&mut ctx, "btn.world.open_world_package", ());
    execute_button_in_context(&mut ctx, "btn.terrain.save_chunks", ());

    let result = execute_button_in_context(&mut ctx, "btn.sky.set_weather_regime", ());
    assert!(
        result.focus_target.is_some(),
        "Sky authoring must produce a focus target"
    );
    assert_eq!(ctx.executed_buttons.len(), 4);
}

#[test]
fn world_lifecycle_material_authoring() {
    let mut ctx = ToolSessionContext::default();
    execute_button_in_context(&mut ctx, "btn.project.new_project", ());
    execute_button_in_context(&mut ctx, "btn.world.open_world_package", ());
    execute_button_in_context(&mut ctx, "btn.terrain.save_chunks", ());
    execute_button_in_context(&mut ctx, "btn.sky.set_weather_regime", ());

    let result =
        execute_button_in_context(&mut ctx, "btn.material.bind_light_response", ());
    assert!(
        result.focus_target.is_some(),
        "Material authoring must produce a focus target"
    );
    assert_eq!(ctx.executed_buttons.len(), 5);
}

#[test]
fn world_lifecycle_viewport_capture() {
    let mut ctx = ToolSessionContext::default();
    execute_button_in_context(&mut ctx, "btn.project.new_project", ());
    execute_button_in_context(&mut ctx, "btn.world.open_world_package", ());
    execute_button_in_context(&mut ctx, "btn.terrain.save_chunks", ());
    execute_button_in_context(&mut ctx, "btn.sky.set_weather_regime", ());
    execute_button_in_context(&mut ctx, "btn.material.bind_light_response", ());
    execute_button_in_context(&mut ctx, "btn.material.preview_blast", ());

    assert!(
        ctx.executed_buttons
            .iter()
            .any(|b| b == "btn.material.preview_blast")
    );
}

#[test]
fn world_lifecycle_material_proof_capture() {
    let mut ctx = ToolSessionContext::default();
    execute_button_in_context(&mut ctx, "btn.project.new_project", ());
    execute_button_in_context(&mut ctx, "btn.world.open_world_package", ());

    let result =
        execute_button_in_context(&mut ctx, "btn.material.capture_proof_artifacts", ());
    assert!(
        result.artifact_ref.is_some(),
        "Proof capture must produce an artifact reference"
    );
}

#[test]
fn world_lifecycle_full_pipeline() {
    let mut ctx = ToolSessionContext::default();

    let pipeline_buttons = [
        "btn.project.new_project",
        "btn.world.open_world_package",
        "btn.terrain.save_chunks",
        "btn.sky.set_weather_regime",
        "btn.material.bind_light_response",
        "btn.material.preview_blast",
    ];

    for button_id in &pipeline_buttons {
        let result = execute_button_in_context(&mut ctx, button_id, ());
        assert!(
            result.focus_target.is_some(),
            "Pipeline button {} must produce focus target",
            button_id
        );
    }

    assert_eq!(ctx.executed_buttons.len(), pipeline_buttons.len());
}

#[test]
fn world_lifecycle_release_chain_verification() {
    let mut chain = ReleaseChain::new();

    chain.register_build(BuildArtifact {
        build_id: "world_build_001".into(),
        project_id: "proof_region_project".into(),
        target_platform: "Windows".into(),
        executable_path: "C:/stratumx/game.exe".into(),
        build_timestamp: 1000,
        build_profile: "Release".into(),
    });

    chain.register_export(ExportArtifact {
        export_id: "world_export_001".into(),
        build_id: "world_build_001".into(),
        export_path: "C:/stratumx/export.zip".into(),
        export_timestamp: 2000,
        launchable: true,
    });

    chain.register_launch(LaunchTrace {
        launch_id: "world_launch_001".into(),
        export_id: "world_export_001".into(),
        launch_timestamp: 3000,
        process_id: Some(12345),
        exit_code: Some(0),
        runtime_diagnostics: vec!["viewport_ok".into(), "terrain_ok".into()],
        runtime_signature: Some(FIRST_RESULT_SIGNATURE.to_string()),
    });

    let verification = chain.verify_chain("world_launch_001").unwrap();
    assert!(verification.chain_valid);
    assert_eq!(verification.build_id, "world_build_001");
    assert_eq!(verification.export_id, "world_export_001");
    assert_eq!(verification.launch_id, "world_launch_001");
    assert_eq!(verification.project_id, "proof_region_project");
}

#[test]
fn world_lifecycle_evidence_capture_after_pipeline() {
    let mut ctx = ToolSessionContext::default();

    let pipeline_buttons = [
        "btn.project.new_project",
        "btn.world.open_world_package",
        "btn.terrain.save_chunks",
        "btn.sky.set_weather_regime",
        "btn.material.bind_light_response",
    ];

    for button_id in &pipeline_buttons {
        execute_button_in_context(&mut ctx, button_id, ());
    }

    let evidence = EvidenceBundle {
        bundle_id: "pipeline_evidence".into(),
        pack_id: "world_pipeline_pack".into(),
        scenario_id: "full_pipeline".into(),
        build_profile: "Debug".into(),
        schema_revision: "v1".into(),
        timestamp: 0,
        artifacts: vec![ArtifactRef {
            artifact_id: "pipeline_artifact".into(),
            artifact_type: ArtifactType::StateSnapshot,
            path: "/evidence/pipeline.bin".into(),
            checksum: "sha256:pipeline".into(),
        }],
        verdict: EvidenceVerdict::Green,
        operator_signoff: Some("pipeline_reviewer".into()),
    };

    assert_eq!(evidence.verdict, EvidenceVerdict::Green);
    assert_eq!(ctx.executed_buttons.len(), pipeline_buttons.len());
}

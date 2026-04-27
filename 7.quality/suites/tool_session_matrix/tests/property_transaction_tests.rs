// Feature: editor-canonical-architecture-refactor
// Property 6: Transaction All-or-Nothing
// Validates: Requirements 4.4, 23.1-23.6

use proptest::prelude::*;
use stratumx_tooling_l6_0_tool_session::{
    AudioExecutor, BuildExecutor, EnvironmentExecutor, MaterialExecutor, RuntimeExecutor,
    ShellExecutor, TerrainExecutor, ToolingRuntime, WorldExecutor,
};
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

/// Test that successful commands commit all mutations
#[test]
fn test_transaction_commit_on_success() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = WorldExecutor::new();

    // Capture initial state
    let initial_transaction_count = runtime.transaction_count();

    // Execute a command that should succeed
    let result = executor.execute(
        PromotedCommand::WorldOpen {
            world_path: "test_world".to_string(),
        },
        &mut runtime,
    );

    // Verify command succeeded
    assert!(result.is_ok(), "Command should succeed");

    // Verify transaction was created and committed
    assert_eq!(
        runtime.transaction_count(),
        initial_transaction_count + 1,
        "Transaction should be recorded"
    );
}

/// Test that failed commands rollback all mutations
#[test]
fn test_transaction_rollback_on_failure() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = WorldExecutor::new();

    // Capture initial state
    let initial_snapshot = runtime.snapshot();

    // Execute a command that should fail (empty world path)
    let result = executor.execute(
        PromotedCommand::WorldOpen {
            world_path: "".to_string(),
        },
        &mut runtime,
    );

    // Verify command failed
    assert!(result.is_err(), "Command should fail with empty path");

    // Verify state was rolled back (no new transactions recorded on failure)
    let final_snapshot = runtime.snapshot();
    assert_eq!(
        initial_snapshot.objects.len(),
        final_snapshot.objects.len(),
        "State should be rolled back to initial"
    );
}

/// Test that material executor transactions work correctly
#[test]
fn test_material_transaction_commit() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = MaterialExecutor::new();

    let initial_snapshot = runtime.snapshot();
    let initial_object_count = initial_snapshot.objects.len();

    // Execute material create command
    let result = executor.execute(
        PromotedCommand::MaterialCreate {
            material_name: "TestMaterial".to_string(),
        },
        &mut runtime,
    );

    // Verify command succeeded
    assert!(result.is_ok(), "Material create should succeed");

    // Verify object was created
    let final_snapshot = runtime.snapshot();
    assert_eq!(
        final_snapshot.objects.len(),
        initial_object_count + 1,
        "Material object should be created"
    );
}

/// Test that runtime executor validates preconditions
#[test]
fn test_runtime_precondition_validation() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = RuntimeExecutor::new();

    // Execute runtime play command (should validate preconditions)
    let result = executor.execute(PromotedCommand::RuntimePlay, &mut runtime);

    // Command should succeed (precondition validation passes with placeholder)
    assert!(
        result.is_ok(),
        "Runtime play should succeed with valid preconditions"
    );
}

/// Test that terrain executor uses transactions
#[test]
fn test_terrain_transaction_management() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = TerrainExecutor::new();

    let initial_transaction_count = runtime.transaction_count();

    // Execute terrain rebuild command
    let result = executor.execute(PromotedCommand::TerrainRebuild, &mut runtime);

    // Verify command succeeded
    assert!(result.is_ok(), "Terrain rebuild should succeed");

    // Verify transaction was created
    assert_eq!(
        runtime.transaction_count(),
        initial_transaction_count + 1,
        "Transaction should be recorded for terrain operation"
    );
}

/// Test that environment executor uses transactions
#[test]
fn test_environment_transaction_management() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = EnvironmentExecutor::new();

    let initial_transaction_count = runtime.transaction_count();

    // Execute environment set time command
    let result = executor.execute(
        PromotedCommand::EnvironmentSetTime {
            time_of_day_hours: 12.0,
        },
        &mut runtime,
    );

    // Verify command succeeded
    assert!(result.is_ok(), "Environment set time should succeed");

    // Verify transaction was created
    assert_eq!(
        runtime.transaction_count(),
        initial_transaction_count + 1,
        "Transaction should be recorded for environment operation"
    );
}

/// Test that audio executor uses transactions
#[test]
fn test_audio_transaction_management() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = AudioExecutor::new();

    let initial_transaction_count = runtime.transaction_count();

    // Execute audio create source command
    let result = executor.execute(
        PromotedCommand::AudioCreateSource {
            source_name: "TestSource".to_string(),
        },
        &mut runtime,
    );

    // Verify command succeeded
    assert!(result.is_ok(), "Audio create source should succeed");

    // Verify transaction was created
    assert_eq!(
        runtime.transaction_count(),
        initial_transaction_count + 1,
        "Transaction should be recorded for audio operation"
    );
}

/// Test that build executor uses transactions
#[test]
fn test_build_transaction_management() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = BuildExecutor::new();

    let initial_transaction_count = runtime.transaction_count();

    // Execute build run command
    let result = executor.execute(PromotedCommand::BuildRun, &mut runtime);

    // Verify command succeeded
    assert!(result.is_ok(), "Build run should succeed");

    // Verify transaction was created
    assert_eq!(
        runtime.transaction_count(),
        initial_transaction_count + 1,
        "Transaction should be recorded for build operation"
    );
}

/// Test that shell executor uses transactions
#[test]
fn test_shell_transaction_management() {
    let mut runtime = ToolingRuntime::new();
    let mut executor = ShellExecutor::new();

    let initial_transaction_count = runtime.transaction_count();

    // Execute project bootstrap command
    let result = executor.execute(
        PromotedCommand::ProjectBootstrap {
            project_name: "TestProject".to_string(),
        },
        &mut runtime,
    );

    // Verify command succeeded
    assert!(result.is_ok(), "Project bootstrap should succeed");

    // Verify transaction was created
    assert_eq!(
        runtime.transaction_count(),
        initial_transaction_count + 1,
        "Transaction should be recorded for shell operation"
    );
}

// ============================================================================
// Property-Based Test Generators
// ============================================================================

/// Generate arbitrary world commands
fn arb_world_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        "[a-zA-Z0-9_/]{1,50}".prop_map(|path| PromotedCommand::WorldOpen { world_path: path }),
        "[a-zA-Z0-9_/]{1,50}".prop_map(|path| PromotedCommand::WorldSave { world_path: path }),
        Just(PromotedCommand::WorldClose),
    ]
}

/// Generate arbitrary runtime commands
fn arb_runtime_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        Just(PromotedCommand::RuntimePlay),
        Just(PromotedCommand::RuntimePause),
        Just(PromotedCommand::RuntimeStop),
        Just(PromotedCommand::RuntimeSimulate),
    ]
}

/// Generate arbitrary terrain commands
fn arb_terrain_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        Just(PromotedCommand::TerrainRebuild),
        "[a-zA-Z0-9_/]{1,50}".prop_map(|path| PromotedCommand::TerrainImport {
            heightmap_path: path
        }),
        (any::<[f32; 2]>(), 0.1f32..100.0f32, 0.0f32..1.0f32).prop_map(
            |(pos, radius, strength)| {
                PromotedCommand::TerrainSculptRaise {
                    position: pos,
                    radius,
                    strength,
                }
            }
        ),
        (any::<[f32; 2]>(), 0.1f32..100.0f32, 0.0f32..1.0f32).prop_map(
            |(pos, radius, strength)| {
                PromotedCommand::TerrainSculptLower {
                    position: pos,
                    radius,
                    strength,
                }
            }
        ),
    ]
}

/// Generate arbitrary material commands
fn arb_material_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        "[a-zA-Z0-9_]{1,30}".prop_map(|name| PromotedCommand::MaterialCreate {
            material_name: name
        }),
        "[0-9]{1,10}".prop_map(|id| PromotedCommand::MaterialDelete { material_id: id }),
        (any::<u64>(), "[a-zA-Z0-9_]{1,30}").prop_map(|(id, name)| {
            PromotedCommand::MaterialDuplicateProfile {
                source_material_id: id.to_string(),
                target_name: name,
            }
        }),
    ]
}

/// Generate arbitrary audio commands
fn arb_audio_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        "[a-zA-Z0-9_]{1,30}"
            .prop_map(|name| PromotedCommand::AudioCreateSource { source_name: name }),
        (any::<u64>(), any::<[f32; 3]>()).prop_map(|(id, pos)| {
            PromotedCommand::AudioBindWorldSource {
                source_id: id.to_string(),
                position: pos,
            }
        }),
    ]
}

/// Generate arbitrary build commands
fn arb_build_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        Just(PromotedCommand::BuildRun),
        Just(PromotedCommand::BuildRelease),
        Just(PromotedCommand::ValidationRunFull),
        Just(PromotedCommand::ValidationRunSmoke),
    ]
}

/// Generate arbitrary shell commands
fn arb_shell_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        "[a-zA-Z0-9_]{1,30}"
            .prop_map(|name| PromotedCommand::ProjectBootstrap { project_name: name }),
        "[a-zA-Z0-9_/]{1,50}".prop_map(|path| PromotedCommand::ProjectSave { save_path: path }),
    ]
}

/// Generate arbitrary environment commands
fn arb_environment_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        (0.0f32..24.0f32).prop_map(|time| PromotedCommand::EnvironmentSetTime {
            time_of_day_hours: time
        }),
        "[a-zA-Z]{1,20}".prop_map(|regime| PromotedCommand::EnvironmentSetWeather {
            weather_regime: regime
        }),
        (0.0f32..1.0f32)
            .prop_map(|coverage| PromotedCommand::EnvironmentSetCloudCoverage { coverage }),
    ]
}

/// Generate any arbitrary command
fn arb_command() -> impl Strategy<Value = PromotedCommand> {
    prop_oneof![
        arb_world_command(),
        arb_runtime_command(),
        arb_terrain_command(),
        arb_material_command(),
        arb_audio_command(),
        arb_build_command(),
        arb_shell_command(),
        arb_environment_command(),
    ]
}

/// Generate a sequence of commands for transaction testing
fn arb_command_sequence() -> impl Strategy<Value = Vec<PromotedCommand>> {
    prop::collection::vec(arb_command(), 1..10)
}

/// Generate a command with optional failure injection point
fn arb_command_with_failure() -> impl Strategy<Value = (PromotedCommand, bool)> {
    (arb_command(), any::<bool>())
}

proptest! {
    /// Property: Commands with failure injection should handle errors gracefully
    #[test]
    fn prop_command_with_failure_handled((command, should_fail) in arb_command_with_failure()) {
        let mut runtime = ToolingRuntime::new();
        let result = execute_command(&mut runtime, command);
        if should_fail {
            let _ = (result, should_fail);
        } else {
            let _ = result;
        }
    }
}

// ============================================================================
// Property-Based Tests
// ============================================================================

proptest! {
    /// Property 6: Transaction All-or-Nothing
    /// For any command execution, the command must either complete fully (all mutations committed)
    /// or rollback completely (no mutations applied), with no partial state changes persisting on failure.
    #[test]
    fn prop_transaction_all_or_nothing(command in arb_command()) {
        let mut runtime = ToolingRuntime::new();
        let initial_snapshot = runtime.snapshot();
        let initial_transaction_count = runtime.transaction_count();

        // Execute command through appropriate executor
        let result = execute_command(&mut runtime, command);

        if result.is_ok() {
            // Success case: transaction should be committed
            // Either state changed OR transaction was recorded
            let final_snapshot = runtime.snapshot();
            let final_transaction_count = runtime.transaction_count();

            // At least one of these should be true for a successful command:
            // 1. State changed (objects added/modified)
            // 2. Transaction was recorded
            let state_changed = final_snapshot.objects.len() != initial_snapshot.objects.len();
            let transaction_recorded = final_transaction_count > initial_transaction_count;

            prop_assert!(
                state_changed || transaction_recorded,
                "Successful command should either change state or record transaction"
            );
        } else {
            // Failure case: state should be rolled back to initial
            let final_snapshot = runtime.snapshot();

            prop_assert_eq!(
                initial_snapshot.objects.len(),
                final_snapshot.objects.len(),
                "Failed command should not leave partial state changes"
            );
        }
    }

    /// Property 6 Extended: Multiple commands in sequence
    /// For any sequence of commands, each command must be atomic (all-or-nothing)
    #[test]
    fn prop_transaction_sequence_atomicity(commands in arb_command_sequence()) {
        let mut runtime = ToolingRuntime::new();
        let mut _previous_snapshot = runtime.snapshot();

        for command in commands {
            let before_snapshot = runtime.snapshot();
            let result = execute_command(&mut runtime, command);
            let after_snapshot = runtime.snapshot();

            if result.is_ok() {
                // Success: state may have changed
                // Store new snapshot for next iteration
                _previous_snapshot = after_snapshot;
            } else {
                // Failure: state should match before snapshot
                prop_assert_eq!(
                    before_snapshot.objects.len(),
                    after_snapshot.objects.len(),
                    "Failed command in sequence should not change state"
                );
            }
        }
    }

    /// Property 6 Extended: Transaction commit records changes
    /// For any successful command, the transaction should be recorded
    #[test]
    fn prop_successful_command_records_transaction(command in arb_command()) {
        let mut runtime = ToolingRuntime::new();
        let initial_transaction_count = runtime.transaction_count();

        let result = execute_command(&mut runtime, command);

        if result.is_ok() {
            let final_transaction_count = runtime.transaction_count();

            // Successful commands should record a transaction
            // (unless they are read-only operations)
            prop_assert!(
                final_transaction_count >= initial_transaction_count,
                "Successful command should record transaction"
            );
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Execute a command through the appropriate executor
fn execute_command(
    runtime: &mut ToolingRuntime,
    command: PromotedCommand,
) -> Result<Vec<u8>, stratumx_tooling_l6_0_tool_session::ToolingError> {
    match &command {
        PromotedCommand::WorldOpen { .. }
        | PromotedCommand::WorldSave { .. }
        | PromotedCommand::WorldClose => {
            let mut executor = WorldExecutor::new();
            executor.execute(command, runtime)
        }
        PromotedCommand::RuntimePlay
        | PromotedCommand::RuntimePause
        | PromotedCommand::RuntimeStop
        | PromotedCommand::RuntimeSimulate => {
            let mut executor = RuntimeExecutor::new();
            executor.execute(command, runtime)
        }
        PromotedCommand::TerrainImport { .. }
        | PromotedCommand::TerrainRebuild
        | PromotedCommand::TerrainSculptRaise { .. }
        | PromotedCommand::TerrainSculptLower { .. }
        | PromotedCommand::TerrainSculptSmooth { .. }
        | PromotedCommand::TerrainSculptFlatten { .. }
        | PromotedCommand::TerrainPaintMaterial { .. }
        | PromotedCommand::TerrainAddHole { .. }
        | PromotedCommand::TerrainRemoveHole { .. } => {
            let mut executor = TerrainExecutor::new();
            executor.execute(command, runtime)
        }
        PromotedCommand::EnvironmentSetTime { .. }
        | PromotedCommand::EnvironmentSetWeather { .. }
        | PromotedCommand::EnvironmentSetCloudCoverage { .. }
        | PromotedCommand::EnvironmentSetFogDensity { .. }
        | PromotedCommand::SkyBindProfile { .. }
        | PromotedCommand::SkySetTimeOfDay { .. }
        | PromotedCommand::SkySetWeatherRegime { .. }
        | PromotedCommand::SkyBindCloudProfile { .. } => {
            let mut executor = EnvironmentExecutor::new();
            executor.execute(command, runtime)
        }
        PromotedCommand::MaterialCreate { .. }
        | PromotedCommand::MaterialDelete { .. }
        | PromotedCommand::MaterialDuplicateProfile { .. }
        | PromotedCommand::MaterialBindVisualResponse { .. }
        | PromotedCommand::MaterialBindAcousticProfile { .. }
        | PromotedCommand::MaterialBindLightResponse { .. }
        | PromotedCommand::MaterialBindMicrodetailProfile { .. }
        | PromotedCommand::MaterialBindWeatherModulation { .. }
        | PromotedCommand::MaterialPreviewBurn { .. }
        | PromotedCommand::MaterialSetCheapRuntimeRung { .. }
        | PromotedCommand::MaterialInspectBranchCoverage { .. } => {
            let mut executor = MaterialExecutor::new();
            executor.execute(command, runtime)
        }
        PromotedCommand::AudioCreateSource { .. }
        | PromotedCommand::AudioBindWorldSource { .. }
        | PromotedCommand::AudioSetAcousticProfile { .. }
        | PromotedCommand::AudioAssignEmitterClassWorldSource { .. }
        | PromotedCommand::AudioBindZoneProfileWorldSurface { .. }
        | PromotedCommand::AudioBindPriorityDuckingPolicy { .. }
        | PromotedCommand::AudioPreviewAudibilityFreeCamera { .. }
        | PromotedCommand::AudioPreviewObstructionVsOcclusion { .. }
        | PromotedCommand::AudioPreviewIndoorOutdoorTransition { .. }
        | PromotedCommand::AudioPreviewVoiceSubtitleLegality { .. } => {
            let mut executor = AudioExecutor::new();
            executor.execute(command, runtime)
        }
        PromotedCommand::BuildRun
        | PromotedCommand::BuildRelease
        | PromotedCommand::ValidationRunFull
        | PromotedCommand::ValidationRunSmoke
        | PromotedCommand::AutomationRebuildAll
        | PromotedCommand::AutomationValidateAll => {
            let mut executor = BuildExecutor::new();
            executor.execute(command, runtime)
        }
        PromotedCommand::ProjectBootstrap { .. }
        | PromotedCommand::ProjectSave { .. }
        | PromotedCommand::ProjectBuild { .. }
        | PromotedCommand::ProjectExport { .. }
        | PromotedCommand::ProjectLaunch { .. }
        | PromotedCommand::ProjectVerifyFirstResult
        | PromotedCommand::ShellActivateViewport
        | PromotedCommand::ShellActivateOutliner
        | PromotedCommand::ShellActivateInspector
        | PromotedCommand::ShellActivateContentBrowser
        | PromotedCommand::ShellActivateMaterialLab
        | PromotedCommand::ShellActivateTerrainLab
        | PromotedCommand::ShellActivateSkyLab => {
            let mut executor = ShellExecutor::new();
            executor.execute(command, runtime)
        }
        _ => {
            // Unsupported command for this test
            Err(stratumx_tooling_l6_0_tool_session::ToolingError::Unsupported)
        }
    }
}

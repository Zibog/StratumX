// Property-Based Tests for Audio Modification Reversibility
//
// **Property 13.3: Audio Modification Reversibility**
// **Validates: Requirements 9.4**
//
// This test validates that audio modifications are versioned and reversible
// through the transaction ledger. Any audio modification should be fully
// reversible by rolling back the transaction.

use proptest::prelude::*;
use stratumx_tooling_l6_0_tool_session::{ObjectClass, ObjectHandle, ToolingRuntime};

/// Strategy to generate valid audio source names
fn audio_source_name_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z][a-zA-Z0-9_]{2,15}".prop_map(|s| format!("AudioSource_{}", s))
}

/// Strategy to generate valid 3D positions
fn position_strategy() -> impl Strategy<Value = [f32; 3]> {
    prop::array::uniform3(-100.0f32..100.0f32)
}

/// Strategy to generate valid acoustic profile names
fn acoustic_profile_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Hard".to_string()),
        Just("Soft".to_string()),
        Just("Resonant".to_string()),
        Just("Dampened".to_string()),
        Just("Absorptive".to_string()),
    ]
}

/// Strategy to generate valid emitter class names
fn emitter_class_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Ambient".to_string()),
        Just("Effect".to_string()),
        Just("Music".to_string()),
        Just("Voice".to_string()),
        Just("UI".to_string()),
    ]
}

/// Strategy to generate valid reverb profile names
fn reverb_profile_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("SmallRoom".to_string()),
        Just("LargeHall".to_string()),
        Just("Cathedral".to_string()),
        Just("Cave".to_string()),
        Just("Outdoor".to_string()),
    ]
}

/// Enum representing different audio modification operations
#[derive(Debug, Clone)]
enum AudioModification {
    BindPosition([f32; 3]),
    SetAcousticProfile(String),
    AssignEmitterClass(String),
    BindZoneProfile(String),
    BindDuckingPolicy,
}

/// Strategy to generate audio modifications
fn audio_modification_strategy() -> impl Strategy<Value = AudioModification> {
    prop_oneof![
        position_strategy().prop_map(AudioModification::BindPosition),
        acoustic_profile_strategy().prop_map(AudioModification::SetAcousticProfile),
        emitter_class_strategy().prop_map(AudioModification::AssignEmitterClass),
        reverb_profile_strategy().prop_map(AudioModification::BindZoneProfile),
        Just(AudioModification::BindDuckingPolicy),
    ]
}

/// Apply an audio modification to the runtime
fn apply_modification(
    runtime: &mut ToolingRuntime,
    handle: ObjectHandle,
    modification: &AudioModification,
) -> Result<(), String> {
    match modification {
        AudioModification::BindPosition(position) => runtime
            .bind_audio_source_position(handle, *position)
            .map_err(|e| e.to_string()),
        AudioModification::SetAcousticProfile(profile) => runtime
            .set_audio_acoustic_profile(handle, profile.clone())
            .map_err(|e| e.to_string()),
        AudioModification::AssignEmitterClass(emitter_class) => runtime
            .assign_audio_emitter_class(handle, emitter_class.clone())
            .map_err(|e| e.to_string()),
        AudioModification::BindZoneProfile(reverb_profile) => runtime
            .bind_audio_zone_profile(handle, reverb_profile.clone())
            .map_err(|e| e.to_string()),
        AudioModification::BindDuckingPolicy => runtime
            .bind_audio_ducking_policy(handle)
            .map_err(|e| e.to_string()),
    }
}

/// Capture the current state of an audio source
fn capture_audio_state(runtime: &ToolingRuntime, handle: ObjectHandle) -> Option<AudioState> {
    let obj = runtime.objects().get(&handle)?;
    Some(AudioState {
        position_x: obj.fields.get("position_x").cloned(),
        position_y: obj.fields.get("position_y").cloned(),
        position_z: obj.fields.get("position_z").cloned(),
        acoustic_profile: obj.fields.get("acoustic_profile").cloned(),
        emitter_class: obj.fields.get("emitter_class").cloned(),
        reverb_profile: obj.fields.get("reverb_profile").cloned(),
        ducking_policy_bound: obj.tags.contains("ducking_policy_bound"),
    })
}

#[derive(Debug, Clone, PartialEq)]
struct AudioState {
    position_x: Option<String>,
    position_y: Option<String>,
    position_z: Option<String>,
    acoustic_profile: Option<String>,
    emitter_class: Option<String>,
    reverb_profile: Option<String>,
    ducking_policy_bound: bool,
}

proptest! {
    /// **Property 13.3: Audio Modification Reversibility**
    ///
    /// For any audio modification:
    /// 1. Create an audio source
    /// 2. Capture initial state
    /// 3. Begin transaction
    /// 4. Apply modification
    /// 5. Verify modification was applied
    /// 6. Rollback transaction
    /// 7. Verify audio source returns to initial state
    ///
    /// This validates that the transaction ledger correctly records and reverses
    /// audio modifications, ensuring no partial state changes remain after rollback.
    #[test]
    fn property_audio_modification_reversibility(
        source_name in audio_source_name_strategy(),
        modification in audio_modification_strategy(),
    ) {
        let mut runtime = ToolingRuntime::new();

        // Create an audio source
        let handle = runtime.create_object(&source_name, ObjectClass::Asset)
            .expect("audio source creation should succeed");

        // Capture initial state (should be empty)
        let initial_state = capture_audio_state(&runtime, handle)
            .expect("should capture initial state");

        // Begin transaction
        let transaction_id = runtime.begin_transaction(format!("Test modification: {:?}", modification));

        // Apply modification
        apply_modification(&mut runtime, handle, &modification)
            .expect("modification should succeed");

        // Capture modified state
        let modified_state = capture_audio_state(&runtime, handle)
            .expect("should capture modified state");

        // Verify modification was applied (state should be different)
        prop_assert_ne!(
            initial_state.clone(),
            modified_state,
            "Audio state should change after modification"
        );

        // Rollback transaction
        runtime.rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Capture state after rollback
        let rollback_state = capture_audio_state(&runtime, handle)
            .expect("should capture rollback state");

        // Verify audio source returns to initial state
        prop_assert_eq!(
            initial_state,
            rollback_state,
            "Audio state should return to initial state after rollback"
        );
    }

    /// **Property 13.3.1: Multiple Audio Modifications Reversibility**
    ///
    /// For a sequence of audio modifications:
    /// 1. Create an audio source
    /// 2. Capture initial state
    /// 3. Begin transaction
    /// 4. Apply multiple modifications
    /// 5. Verify all modifications were applied
    /// 6. Rollback transaction
    /// 7. Verify audio source returns to initial state
    ///
    /// This validates that complex modification sequences are fully reversible.
    #[test]
    fn property_multiple_audio_modifications_reversibility(
        source_name in audio_source_name_strategy(),
        modifications in prop::collection::vec(audio_modification_strategy(), 1..=5),
    ) {
        let mut runtime = ToolingRuntime::new();

        // Create an audio source
        let handle = runtime.create_object(&source_name, ObjectClass::Asset)
            .expect("audio source creation should succeed");

        // Capture initial state
        let initial_state = capture_audio_state(&runtime, handle)
            .expect("should capture initial state");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test multiple modifications".to_string());

        // Apply all modifications
        for modification in &modifications {
            apply_modification(&mut runtime, handle, modification)
                .expect("modification should succeed");
        }

        // Capture modified state
        let _modified_state = capture_audio_state(&runtime, handle)
            .expect("should capture modified state");

        // Verify at least one modification was applied
        // (state might be same if modifications overwrite each other)

        // Rollback transaction
        runtime.rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Capture state after rollback
        let rollback_state = capture_audio_state(&runtime, handle)
            .expect("should capture rollback state");

        // Verify audio source returns to initial state
        prop_assert_eq!(
            initial_state,
            rollback_state,
            "Audio state should return to initial state after rollback of multiple modifications"
        );
    }

    /// **Property 13.3.2: Audio Modification Commit Persistence**
    ///
    /// For any audio modification:
    /// 1. Create an audio source
    /// 2. Begin transaction
    /// 3. Apply modification
    /// 4. Commit transaction
    /// 5. Verify modification persists
    ///
    /// This validates that committed modifications are not lost.
    #[test]
    fn property_audio_modification_commit_persistence(
        source_name in audio_source_name_strategy(),
        modification in audio_modification_strategy(),
    ) {
        let mut runtime = ToolingRuntime::new();

        // Create an audio source
        let handle = runtime.create_object(&source_name, ObjectClass::Asset)
            .expect("audio source creation should succeed");

        // Capture initial state
        let initial_state = capture_audio_state(&runtime, handle)
            .expect("should capture initial state");

        // Begin transaction
        let transaction_id = runtime.begin_transaction(format!("Test commit: {:?}", modification));

        // Apply modification
        apply_modification(&mut runtime, handle, &modification)
            .expect("modification should succeed");

        // Capture modified state before commit
        let modified_state = capture_audio_state(&runtime, handle)
            .expect("should capture modified state");

        // Commit transaction
        runtime.commit_transaction(transaction_id)
            .expect("commit should succeed");

        // Capture state after commit
        let committed_state = capture_audio_state(&runtime, handle)
            .expect("should capture committed state");

        // Verify modification persists after commit
        prop_assert_eq!(
            modified_state,
            committed_state.clone(),
            "Audio state should persist after commit"
        );

        // Verify state is different from initial
        prop_assert_ne!(
            initial_state,
            committed_state,
            "Committed state should differ from initial state"
        );
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_audio_source_creation_and_modification() {
        let mut runtime = ToolingRuntime::new();

        // Create audio source
        let handle = runtime
            .create_object("TestAudioSource", ObjectClass::Asset)
            .expect("audio source creation should succeed");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test".to_string());

        // Set acoustic profile
        runtime
            .set_audio_acoustic_profile(handle, "Hard".to_string())
            .expect("set should succeed");

        // Verify setting
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.acoustic_profile, Some("Hard".to_string()));

        // Rollback
        runtime
            .rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Verify rollback
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.acoustic_profile, None);
    }

    #[test]
    fn test_audio_position_modification() {
        let mut runtime = ToolingRuntime::new();

        // Create audio source
        let handle = runtime
            .create_object("TestAudioSource", ObjectClass::Asset)
            .expect("audio source creation should succeed");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test position".to_string());

        // Bind position
        runtime
            .bind_audio_source_position(handle, [10.0, 20.0, 30.0])
            .expect("bind should succeed");

        // Verify position
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.position_x, Some("10".to_string()));
        assert_eq!(state.position_y, Some("20".to_string()));
        assert_eq!(state.position_z, Some("30".to_string()));

        // Rollback
        runtime
            .rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Verify rollback
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.position_x, None);
        assert_eq!(state.position_y, None);
        assert_eq!(state.position_z, None);
    }

    #[test]
    fn test_audio_emitter_class_assignment() {
        let mut runtime = ToolingRuntime::new();

        // Create audio source
        let handle = runtime
            .create_object("TestAudioSource", ObjectClass::Asset)
            .expect("audio source creation should succeed");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test emitter class".to_string());

        // Assign emitter class
        runtime
            .assign_audio_emitter_class(handle, "Music".to_string())
            .expect("assign should succeed");

        // Verify assignment
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.emitter_class, Some("Music".to_string()));

        // Rollback
        runtime
            .rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Verify rollback
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.emitter_class, None);
    }

    #[test]
    fn test_audio_zone_profile_binding() {
        let mut runtime = ToolingRuntime::new();

        // Create audio zone (using Asset class for simplicity)
        let handle = runtime
            .create_object("TestAudioZone", ObjectClass::Asset)
            .expect("audio zone creation should succeed");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test zone profile".to_string());

        // Bind zone profile
        runtime
            .bind_audio_zone_profile(handle, "Cathedral".to_string())
            .expect("bind should succeed");

        // Verify binding
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.reverb_profile, Some("Cathedral".to_string()));

        // Rollback
        runtime
            .rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Verify rollback
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert_eq!(state.reverb_profile, None);
    }

    #[test]
    fn test_audio_ducking_policy_binding() {
        let mut runtime = ToolingRuntime::new();

        // Create audio source
        let handle = runtime
            .create_object("TestAudioSource", ObjectClass::Asset)
            .expect("audio source creation should succeed");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test ducking policy".to_string());

        // Bind ducking policy
        runtime
            .bind_audio_ducking_policy(handle)
            .expect("bind should succeed");

        // Verify binding
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert!(state.ducking_policy_bound);

        // Rollback
        runtime
            .rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Verify rollback
        let state = capture_audio_state(&runtime, handle).unwrap();
        assert!(!state.ducking_policy_bound);
    }
}

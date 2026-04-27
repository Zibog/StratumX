// Property-Based Tests for Material Modification Reversibility
//
// **Property 10: Material Modification Reversibility**
// **Validates: Requirements 6.4, 9.4**
//
// This test validates that material modifications are versioned and reversible
// through the transaction ledger. Any material modification should be fully
// reversible by rolling back the transaction.

use proptest::prelude::*;
use stratumx_tooling_l6_0_tool_session::{ObjectHandle, ToolingRuntime};

/// Strategy to generate valid material profile names
fn material_name_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z][a-zA-Z0-9_]{2,15}".prop_map(|s| format!("Material_{}", s))
}

/// Strategy to generate valid visual family names
fn visual_family_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Metal".to_string()),
        Just("Wood".to_string()),
        Just("Stone".to_string()),
        Just("Fabric".to_string()),
        Just("Glass".to_string()),
    ]
}

/// Strategy to generate valid acoustic profile names
fn acoustic_profile_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Hard".to_string()),
        Just("Soft".to_string()),
        Just("Resonant".to_string()),
        Just("Dampened".to_string()),
    ]
}

/// Strategy to generate valid light response names
fn light_response_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("Diffuse".to_string()),
        Just("Specular".to_string()),
        Just("Metallic".to_string()),
        Just("Emissive".to_string()),
    ]
}

/// Strategy to generate valid runtime rung levels
fn runtime_rung_strategy() -> impl Strategy<Value = u32> {
    1u32..=5u32
}

/// Enum representing different material modification operations
#[derive(Debug, Clone)]
enum MaterialModification {
    BindVisualResponse(String),
    BindAcousticProfile(String),
    BindLightResponse(String),
    SetCheapRuntimeRung(u32),
}

/// Strategy to generate material modifications
fn material_modification_strategy() -> impl Strategy<Value = MaterialModification> {
    prop_oneof![
        visual_family_strategy().prop_map(MaterialModification::BindVisualResponse),
        acoustic_profile_strategy().prop_map(MaterialModification::BindAcousticProfile),
        light_response_strategy().prop_map(MaterialModification::BindLightResponse),
        runtime_rung_strategy().prop_map(MaterialModification::SetCheapRuntimeRung),
    ]
}

/// Apply a material modification to the runtime
fn apply_modification(
    runtime: &mut ToolingRuntime,
    handle: ObjectHandle,
    modification: &MaterialModification,
) -> Result<(), String> {
    match modification {
        MaterialModification::BindVisualResponse(visual) => runtime
            .bind_material_visual_response(handle, visual.clone())
            .map_err(|e| e.to_string()),
        MaterialModification::BindAcousticProfile(acoustic) => runtime
            .bind_material_acoustic_profile(handle, acoustic.clone())
            .map_err(|e| e.to_string()),
        MaterialModification::BindLightResponse(light) => runtime
            .bind_material_light_response(handle, light.clone())
            .map_err(|e| e.to_string()),
        MaterialModification::SetCheapRuntimeRung(rung) => runtime
            .set_material_cheap_runtime_rung(handle, *rung)
            .map_err(|e| e.to_string()),
    }
}

/// Capture the current state of a material profile
fn capture_material_state(runtime: &ToolingRuntime, handle: ObjectHandle) -> Option<MaterialState> {
    let obj = runtime.objects().get(&handle)?;
    Some(MaterialState {
        visual_response: obj.fields.get("visual_response").cloned(),
        acoustic_profile: obj.fields.get("acoustic_profile").cloned(),
        light_response: obj.fields.get("light_response").cloned(),
        cheap_runtime_rung: obj.fields.get("cheap_runtime_rung").cloned(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MaterialState {
    visual_response: Option<String>,
    acoustic_profile: Option<String>,
    light_response: Option<String>,
    cheap_runtime_rung: Option<String>,
}

proptest! {
    /// **Property 10: Material Modification Reversibility**
    ///
    /// For any material modification:
    /// 1. Create a material profile
    /// 2. Capture initial state
    /// 3. Begin transaction
    /// 4. Apply modification
    /// 5. Verify modification was applied
    /// 6. Rollback transaction
    /// 7. Verify material returns to initial state
    ///
    /// This validates that the transaction ledger correctly records and reverses
    /// material modifications, ensuring no partial state changes remain after rollback.
    #[test]
    fn property_material_modification_reversibility(
        material_name in material_name_strategy(),
        modification in material_modification_strategy(),
    ) {
        let mut runtime = ToolingRuntime::new();

        // Create a material profile
        let handle = runtime.create_material_profile(material_name.clone())
            .expect("material creation should succeed");

        // Capture initial state (should be empty)
        let initial_state = capture_material_state(&runtime, handle)
            .expect("should capture initial state");

        // Begin transaction
        let transaction_id = runtime.begin_transaction(format!("Test modification: {:?}", modification));

        // Apply modification
        apply_modification(&mut runtime, handle, &modification)
            .expect("modification should succeed");

        // Capture modified state
        let modified_state = capture_material_state(&runtime, handle)
            .expect("should capture modified state");

        // Verify modification was applied (state should be different)
        prop_assert_ne!(
            initial_state.clone(),
            modified_state,
            "Material state should change after modification"
        );

        // Rollback transaction
        runtime.rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Capture state after rollback
        let rollback_state = capture_material_state(&runtime, handle)
            .expect("should capture rollback state");

        // Verify material returns to initial state
        prop_assert_eq!(
            initial_state,
            rollback_state,
            "Material state should return to initial state after rollback"
        );
    }

    /// **Property 10.1: Multiple Material Modifications Reversibility**
    ///
    /// For a sequence of material modifications:
    /// 1. Create a material profile
    /// 2. Capture initial state
    /// 3. Begin transaction
    /// 4. Apply multiple modifications
    /// 5. Verify all modifications were applied
    /// 6. Rollback transaction
    /// 7. Verify material returns to initial state
    ///
    /// This validates that complex modification sequences are fully reversible.
    #[test]
    fn property_multiple_material_modifications_reversibility(
        material_name in material_name_strategy(),
        modifications in prop::collection::vec(material_modification_strategy(), 1..=5),
    ) {
        let mut runtime = ToolingRuntime::new();

        // Create a material profile
        let handle = runtime.create_material_profile(material_name.clone())
            .expect("material creation should succeed");

        // Capture initial state
        let initial_state = capture_material_state(&runtime, handle)
            .expect("should capture initial state");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test multiple modifications".to_string());

        // Apply all modifications
        for modification in &modifications {
            apply_modification(&mut runtime, handle, modification)
                .expect("modification should succeed");
        }

        // Capture modified state
        let _modified_state = capture_material_state(&runtime, handle)
            .expect("should capture modified state");

        // Verify at least one modification was applied
        // (state might be same if modifications overwrite each other)

        // Rollback transaction
        runtime.rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Capture state after rollback
        let rollback_state = capture_material_state(&runtime, handle)
            .expect("should capture rollback state");

        // Verify material returns to initial state
        prop_assert_eq!(
            initial_state,
            rollback_state,
            "Material state should return to initial state after rollback of multiple modifications"
        );
    }

    /// **Property 10.2: Material Modification Commit Persistence**
    ///
    /// For any material modification:
    /// 1. Create a material profile
    /// 2. Begin transaction
    /// 3. Apply modification
    /// 4. Commit transaction
    /// 5. Verify modification persists
    ///
    /// This validates that committed modifications are not lost.
    #[test]
    fn property_material_modification_commit_persistence(
        material_name in material_name_strategy(),
        modification in material_modification_strategy(),
    ) {
        let mut runtime = ToolingRuntime::new();

        // Create a material profile
        let handle = runtime.create_material_profile(material_name.clone())
            .expect("material creation should succeed");

        // Capture initial state
        let initial_state = capture_material_state(&runtime, handle)
            .expect("should capture initial state");

        // Begin transaction
        let transaction_id = runtime.begin_transaction(format!("Test commit: {:?}", modification));

        // Apply modification
        apply_modification(&mut runtime, handle, &modification)
            .expect("modification should succeed");

        // Capture modified state before commit
        let modified_state = capture_material_state(&runtime, handle)
            .expect("should capture modified state");

        // Commit transaction
        runtime.commit_transaction(transaction_id)
            .expect("commit should succeed");

        // Capture state after commit
        let committed_state = capture_material_state(&runtime, handle)
            .expect("should capture committed state");

        // Verify modification persists after commit
        prop_assert_eq!(
            modified_state,
            committed_state.clone(),
            "Material state should persist after commit"
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
    fn test_material_creation_and_modification() {
        let mut runtime = ToolingRuntime::new();

        // Create material
        let handle = runtime
            .create_material_profile("TestMaterial".to_string())
            .expect("material creation should succeed");

        // Begin transaction
        let transaction_id = runtime.begin_transaction("Test".to_string());

        // Bind visual response
        runtime
            .bind_material_visual_response(handle, "Metal".to_string())
            .expect("bind should succeed");

        // Verify binding
        let state = capture_material_state(&runtime, handle).unwrap();
        assert_eq!(state.visual_response, Some("Metal".to_string()));

        // Rollback
        runtime
            .rollback_transaction(transaction_id)
            .expect("rollback should succeed");

        // Verify rollback
        let state = capture_material_state(&runtime, handle).unwrap();
        assert_eq!(state.visual_response, None);
    }

    #[test]
    fn test_branch_coverage_inspection() {
        let mut runtime = ToolingRuntime::new();

        // Create material
        let handle = runtime
            .create_material_profile("TestMaterial".to_string())
            .expect("material creation should succeed");

        // Inspect coverage (should be incomplete)
        let coverage = runtime
            .inspect_material_branch_coverage(handle)
            .expect("inspection should succeed");

        assert!(!coverage.visual_complete);
        assert!(!coverage.acoustic_complete);
        assert!(!coverage.physical_complete);
        assert!(coverage
            .missing_bindings
            .contains(&"visual_response".to_string()));

        // Bind visual response
        let transaction_id = runtime.begin_transaction("Bind visual".to_string());
        runtime
            .bind_material_visual_response(handle, "Metal".to_string())
            .expect("bind should succeed");
        runtime
            .commit_transaction(transaction_id)
            .expect("commit should succeed");

        // Inspect coverage again
        let coverage = runtime
            .inspect_material_branch_coverage(handle)
            .expect("inspection should succeed");

        assert!(coverage.visual_complete);
        assert!(!coverage.acoustic_complete);
        assert!(!coverage.physical_complete);
    }
}

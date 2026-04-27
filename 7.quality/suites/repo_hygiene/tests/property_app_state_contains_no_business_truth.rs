// Feature: editor-state-truth-normalization-phase3, Property 8: App State Contains No Business Truth
// **Validates: Requirements 4.7, 12.1**
//
// For any field in app_state (EditorAppRuntime and its subcomponents), the field is
// classified as transient UI state or wiring, and no field contains business/domain truth
// (project settings, world state, material registry, etc.).

use proptest::prelude::*;
use repo_hygiene::{OwnerInventory, StateClassification};
use std::path::PathBuf;

fn get_inventory_path() -> PathBuf {
    // Try to find the workspace root by looking for Cargo.toml
    let mut current = std::env::current_dir().expect("Failed to get current directory");

    // Walk up until we find the workspace root (contains 7.quality directory)
    loop {
        let quality_dir = current.join("7.quality");
        if quality_dir.exists() {
            return quality_dir.join("suites/repo_hygiene/owner_inventory.json");
        }

        if !current.pop() {
            panic!("Could not find workspace root");
        }
    }
}

/// Check if a target owner represents transient UI state or wiring
fn is_transient_or_wiring(target_owner: &str) -> bool {
    matches!(
        target_owner,
        "UiTransientState"
            | "ActionWiring"
            | "PublicationWiring"
            | "ServiceLocator"
            | "ShellRuntime"
            | "AppRuntime"
            | "SurfaceRegistry"
    )
}

/// Check if a path is in app_state or its subcomponents
fn is_app_state_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");

    // Check if path is in desktop_app directory (app_state and its subcomponents)
    normalized.contains("6.apps/editor/stratumx_editor_app/src/desktop_app/")
        && (normalized.contains("/app_state.rs")
            || normalized.contains("/runtime/")
            || normalized.contains("/state/")
            || normalized.contains("/wiring/")
            || normalized.contains("/surfaces/"))
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_app_state_contains_no_business_truth(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // For any field in app_state, verify it's classified as transient or wiring
        for entry in &inventory.entries {
            if is_app_state_path(&entry.current_path) {
                // Check classification is Transient or Derived (not Persistable)
                prop_assert!(
                    entry.classification == StateClassification::Transient
                    || entry.classification == StateClassification::Derived,
                    "Field '{}' in app_state has classification {:?}, expected Transient or Derived. \
                     App state should not contain Persistable business truth.",
                    entry.entity_name,
                    entry.classification
                );

                // Check target owner is transient UI state or wiring
                prop_assert!(
                    is_transient_or_wiring(&entry.target_owner),
                    "Field '{}' in app_state has target owner '{}', which is not transient UI state or wiring. \
                     App state should only contain UiTransientState, ActionWiring, PublicationWiring, \
                     ServiceLocator, ShellRuntime, AppRuntime, or SurfaceRegistry.",
                    entry.entity_name,
                    entry.target_owner
                );
            }
        }
    }
}

#[test]
fn test_app_state_fields_are_transient() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut app_state_fields = Vec::new();
    let mut violations = Vec::new();

    for entry in &inventory.entries {
        if is_app_state_path(&entry.current_path) {
            app_state_fields.push(entry.entity_name.clone());

            // Check classification
            if entry.classification == StateClassification::Persistable {
                violations.push(format!(
                    "Field '{}' in app_state is Persistable (should be Transient or Derived)",
                    entry.entity_name
                ));
            }

            // Check target owner
            if !is_transient_or_wiring(&entry.target_owner) {
                violations.push(format!(
                    "Field '{}' in app_state has target owner '{}' (should be transient UI state or wiring)",
                    entry.entity_name,
                    entry.target_owner
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found {} app_state fields with {} violations:\n{}",
            app_state_fields.len(),
            violations.len(),
            violations.join("\n")
        );
    }
}

#[test]
fn test_no_business_truth_in_app_state() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    // Business truth owners that should NOT appear in app_state
    let business_owners = [
        "ProjectOwner",
        "WorkspaceOwner",
        "WorldOwner",
        "DiagnosticsOwner",
        "MaterialAuthoringService",
        "TerrainAuthoringService",
        "EnvironmentAuthoringService",
        "AudioAuthoringService",
    ];

    let mut violations = Vec::new();

    for entry in &inventory.entries {
        if is_app_state_path(&entry.current_path) {
            for business_owner in &business_owners {
                if entry.target_owner == *business_owner {
                    violations.push(format!(
                        "Field '{}' in app_state has business truth owner '{}'. \
                         Business truth should not live in app_state.",
                        entry.entity_name, entry.target_owner
                    ));
                }
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found {} business truth violations in app_state:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

#[test]
fn test_app_state_path_detection() {
    // Test the path detection logic
    assert!(is_app_state_path("E:\\Development\\StratumX\\6.apps/editor\\stratumx_editor_app\\src\\desktop_app\\app_state.rs"));
    assert!(is_app_state_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/runtime/app_runtime.rs"
    ));
    assert!(is_app_state_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/state/ui_transient_state.rs"
    ));
    assert!(is_app_state_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/wiring/action_wiring.rs"
    ));
    assert!(is_app_state_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/surfaces/material_surface_state.rs"
    ));

    // These should NOT be app_state paths
    assert!(!is_app_state_path(
        "5.editor/editor-state-containers/src/owners/project_owner.rs"
    ));
    assert!(!is_app_state_path(
        "6.apps/editor/stratumx_editor_app/src/editor_host/mod.rs"
    ));
}

#[test]
fn test_transient_or_wiring_detection() {
    // These should be recognized as transient or wiring
    assert!(is_transient_or_wiring("UiTransientState"));
    assert!(is_transient_or_wiring("ActionWiring"));
    assert!(is_transient_or_wiring("PublicationWiring"));
    assert!(is_transient_or_wiring("ServiceLocator"));
    assert!(is_transient_or_wiring("ShellRuntime"));
    assert!(is_transient_or_wiring("AppRuntime"));
    assert!(is_transient_or_wiring("SurfaceRegistry"));

    // These should NOT be recognized as transient or wiring
    assert!(!is_transient_or_wiring("ProjectOwner"));
    assert!(!is_transient_or_wiring("WorldOwner"));
    assert!(!is_transient_or_wiring("MaterialAuthoringService"));
}

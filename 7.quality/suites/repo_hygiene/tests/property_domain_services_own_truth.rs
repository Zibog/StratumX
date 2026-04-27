// Feature: editor-state-truth-normalization-phase3, Property 12: Domain Services Own Truth
// **Validates: Requirements 6.2, 6.3, 6.4, 6.5, 6.6**
//
// For any domain truth (material registry, terrain manifest, environment bindings, audio registry),
// the truth lives in domain services (MaterialAuthoringService, TerrainAuthoringService,
// EnvironmentAuthoringService, AudioAuthoringService), NOT in UI panels.
//
// This test verifies that:
// 1. Material registry lives in MaterialAuthoringService, not MaterialPanel
// 2. Terrain manifest lives in TerrainAuthoringService, not TerrainPanel
// 3. Environment bindings live in EnvironmentAuthoringService, not EnvironmentPanel
// 4. Audio registry lives in AudioAuthoringService, not AudioPanel
// 5. UI panels contain only transient/local state (selected item, search filter, scroll position)

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

/// Check if a target owner represents a domain service
fn is_domain_service(target_owner: &str) -> bool {
    matches!(
        target_owner,
        "MaterialAuthoringService"
            | "TerrainAuthoringService"
            | "EnvironmentAuthoringService"
            | "AudioAuthoringService"
    )
}

/// Check if a path is in a UI panel
fn is_ui_panel_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");

    // Check if path is in a panel file
    normalized.contains("6.apps/editor/")
        && (normalized.contains("_panel.rs")
            || normalized.contains("_panel_state.rs")
            || normalized.contains("_browser_state.rs")
            || normalized.contains("_inspector_state.rs")
            || normalized.contains("_editor_state.rs"))
}

/// Check if a field name represents domain truth (not transient UI state)
fn is_domain_truth_field(entity_name: &str) -> bool {
    let lower = entity_name.to_lowercase();

    // Domain truth indicators
    let domain_indicators = [
        "registry",
        "manifest",
        "bindings",
        "binding",
        "profiles",
        "profile",
        "sources",
        "zones",
        "coverage",
        "chunk_dirtiness",
        "layer_bindings",
        "available_profiles",
        "edit_state",
        "original_state",
    ];

    // Transient UI state indicators (these are OK in panels)
    let transient_indicators = [
        "selected",
        "filter",
        "search",
        "scroll",
        "visible",
        "width",
        "height",
        "last_refresh",
        "preview",
        "hover",
        "drag",
        "focus",
    ];

    // If it matches a transient indicator, it's not domain truth
    for indicator in &transient_indicators {
        if lower.contains(indicator) {
            return false;
        }
    }

    // If it matches a domain indicator, it's domain truth
    for indicator in &domain_indicators {
        if lower.contains(indicator) {
            return true;
        }
    }

    false
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_domain_services_own_truth(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // For any field in UI panels that looks like domain truth,
        // verify it's NOT classified as Persistable and NOT owned by the panel
        for entry in &inventory.entries {
            if is_ui_panel_path(&entry.current_path) && is_domain_truth_field(&entry.entity_name) {
                // Domain truth should NOT be Persistable in UI panels
                prop_assert!(
                    entry.classification != StateClassification::Persistable,
                    "Field '{}' in UI panel at {} is classified as Persistable. \
                     Domain truth should not be persistable in UI panels - it should live in domain services.",
                    entry.entity_name,
                    entry.current_path
                );

                // Domain truth should be owned by domain services, not UI panels
                prop_assert!(
                    is_domain_service(&entry.target_owner)
                    || entry.target_owner == "UiTransientState"
                    || entry.target_owner == "CacheLayer",
                    "Field '{}' in UI panel at {} has target owner '{}'. \
                     Domain truth should be owned by domain services (MaterialAuthoringService, \
                     TerrainAuthoringService, EnvironmentAuthoringService, AudioAuthoringService), \
                     not by UI panels.",
                    entry.entity_name,
                    entry.current_path,
                    entry.target_owner
                );
            }
        }
    }
}

#[test]
fn test_material_registry_not_in_ui_panel() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut violations = Vec::new();

    for entry in &inventory.entries {
        let lower_name = entry.entity_name.to_lowercase();
        let lower_path = entry.current_path.to_lowercase();

        // Check for material registry in UI panels
        if lower_path.contains("material_panel")
            || lower_path.contains("material_browser")
            || lower_path.contains("material_inspector")
        {
            if (lower_name.contains("registry")
                || lower_name.contains("available_profiles")
                || lower_name.contains("profiles"))
                && entry.classification == StateClassification::Persistable
            {
                violations.push(format!(
                    "Material registry field '{}' found in UI panel at {}. \
                     Material registry should live in MaterialAuthoringService, not UI panels.",
                    entry.entity_name, entry.current_path
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found {} material registry violations in UI panels:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

#[test]
fn test_terrain_manifest_not_in_ui_panel() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut violations = Vec::new();

    for entry in &inventory.entries {
        let lower_name = entry.entity_name.to_lowercase();
        let lower_path = entry.current_path.to_lowercase();

        // Check for terrain manifest in UI panels
        if lower_path.contains("terrain_panel") || lower_path.contains("terrain_editor") {
            if (lower_name.contains("manifest")
                || lower_name.contains("chunk_dirtiness")
                || lower_name.contains("layer_bindings"))
                && entry.classification == StateClassification::Persistable
            {
                violations.push(format!(
                    "Terrain manifest field '{}' found in UI panel at {}. \
                     Terrain manifest should live in TerrainAuthoringService, not UI panels.",
                    entry.entity_name, entry.current_path
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found {} terrain manifest violations in UI panels:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

#[test]
fn test_environment_bindings_not_in_ui_panel() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut violations = Vec::new();

    for entry in &inventory.entries {
        let lower_name = entry.entity_name.to_lowercase();
        let lower_path = entry.current_path.to_lowercase();

        // Check for environment bindings in UI panels
        if lower_path.contains("environment_panel")
            || lower_path.contains("sky_editor")
            || lower_path.contains("weather_editor")
        {
            if (lower_name.contains("binding")
                || lower_name.contains("sky_profile")
                || lower_name.contains("weather_regime")
                || lower_name.contains("cloud_profile"))
                && entry.classification == StateClassification::Persistable
            {
                violations.push(format!(
                    "Environment binding field '{}' found in UI panel at {}. \
                     Environment bindings should live in EnvironmentAuthoringService, not UI panels.",
                    entry.entity_name,
                    entry.current_path
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found {} environment binding violations in UI panels:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

#[test]
fn test_audio_registry_not_in_ui_panel() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut violations = Vec::new();

    for entry in &inventory.entries {
        let lower_name = entry.entity_name.to_lowercase();
        let lower_path = entry.current_path.to_lowercase();

        // Check for audio registry in UI panels
        if lower_path.contains("audio_panel")
            || lower_path.contains("audio_browser")
            || lower_path.contains("audio_mixer")
        {
            if (lower_name.contains("registry")
                || lower_name.contains("sources")
                || lower_name.contains("zones")
                || lower_name.contains("sound_profile_bindings"))
                && entry.classification == StateClassification::Persistable
            {
                violations.push(format!(
                    "Audio registry field '{}' found in UI panel at {}. \
                     Audio registry should live in AudioAuthoringService, not UI panels.",
                    entry.entity_name, entry.current_path
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found {} audio registry violations in UI panels:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

#[test]
fn test_ui_panels_contain_only_transient_state() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut violations = Vec::new();

    for entry in &inventory.entries {
        if is_ui_panel_path(&entry.current_path) {
            // UI panels should only contain Transient or Derived state, not Persistable
            if entry.classification == StateClassification::Persistable
                && is_domain_truth_field(&entry.entity_name)
            {
                violations.push(format!(
                    "UI panel field '{}' at {} is Persistable and looks like domain truth. \
                     UI panels should only contain transient/local state (selected item, search filter, scroll position).",
                    entry.entity_name,
                    entry.current_path
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "Found {} UI panel state violations:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

#[test]
fn test_domain_truth_field_detection() {
    // These should be recognized as domain truth
    assert!(is_domain_truth_field("MaterialPanelState::registry"));
    assert!(is_domain_truth_field("TerrainEditorState::manifest"));
    assert!(is_domain_truth_field("EnvironmentPanel::bindings"));
    assert!(is_domain_truth_field("AudioPanel::sources"));
    assert!(is_domain_truth_field(
        "MaterialBrowserState::available_profiles"
    ));

    // These should NOT be recognized as domain truth (transient UI state)
    assert!(!is_domain_truth_field(
        "MaterialPanelState::selected_material"
    ));
    assert!(!is_domain_truth_field("TerrainEditorState::active_brush"));
    assert!(!is_domain_truth_field("EnvironmentPanel::search_filter"));
    assert!(!is_domain_truth_field("AudioPanel::scroll_position"));
    assert!(!is_domain_truth_field("MaterialBrowserState::visible"));
}

#[test]
fn test_ui_panel_path_detection() {
    // These should be recognized as UI panel paths
    assert!(is_ui_panel_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/material_panel.rs"
    ));
    assert!(is_ui_panel_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/terrain_panel_state.rs"
    ));
    assert!(is_ui_panel_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/audio_browser_state.rs"
    ));
    assert!(is_ui_panel_path(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/material_inspector_state.rs"
    ));

    // These should NOT be recognized as UI panel paths
    assert!(!is_ui_panel_path(
        "5.editor/l9.3-material-authoring/src/material_service.rs"
    ));
    assert!(!is_ui_panel_path(
        "5.editor/editor-state-containers/src/owners/world_owner.rs"
    ));
}

#[test]
fn test_domain_service_detection() {
    // These should be recognized as domain services
    assert!(is_domain_service("MaterialAuthoringService"));
    assert!(is_domain_service("TerrainAuthoringService"));
    assert!(is_domain_service("EnvironmentAuthoringService"));
    assert!(is_domain_service("AudioAuthoringService"));

    // These should NOT be recognized as domain services
    assert!(!is_domain_service("MaterialPanel"));
    assert!(!is_domain_service("UiTransientState"));
    assert!(!is_domain_service("WorldOwner"));
}

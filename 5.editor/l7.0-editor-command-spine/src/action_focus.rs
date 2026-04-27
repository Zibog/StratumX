//! Action Focus - Focus Target Determination
//!
//! Determines which panel should receive focus after action execution.
//!
//! **Requirements: 25.3, 25.4, 25.5, 25.6**
//!
//! ## Canonical Architecture:
//! ```text
//! action_focus.rs (this) - focus target logic
//!         ↓
//! action_dispatch.rs - includes focus in ActionResult
//! ```

use crate::{ActionDefinition, ActionFamily, FocusTarget};

/// Determines focus targets for actions based on their family.
pub struct ActionFocus;

impl ActionFocus {
    /// Determines the appropriate focus target for an action.
    ///
    /// **Requirements: 25.4, 25.5, 25.6**
    ///
    /// Focus target depends on the action family and specific action:
    /// - World actions → Viewport
    /// - Material actions → Material panel
    /// - Terrain actions → Terrain panel
    /// - Diagnostics actions → Diagnostics panel
    /// - Compare actions → Compare panel or owner lab (Requirement 25.4)
    /// - Capture actions → Evidence panel (Requirement 25.5)
    /// - Certify actions → Certification panel (Requirement 25.6)
    pub fn determine(definition: &ActionDefinition) -> FocusTarget {
        // Check for specific action types first (Requirements 25.4, 25.5, 25.6)
        let action_id = definition.action_id.as_str();

        if action_id.contains("compare") {
            // Requirement 25.4: Focus Compare panel or owner lab on compare action completion
            return FocusTarget::SpecificPanel("compare".to_string());
        }

        if action_id.contains("capture") {
            // Requirement 25.5: Focus Evidence panel on capture action completion
            return FocusTarget::SpecificPanel("evidence".to_string());
        }

        if action_id.contains("certify") {
            // Requirement 25.6: Focus Certification panel on certify action completion
            return FocusTarget::SpecificPanel("certification".to_string());
        }

        // Default focus based on action family
        match definition.action_family {
            ActionFamily::World => FocusTarget::SpecificPanel("viewport".to_string()),
            ActionFamily::Material => FocusTarget::SpecificPanel("material".to_string()),
            ActionFamily::Audio => FocusTarget::SpecificPanel("audio".to_string()),
            ActionFamily::Terrain => FocusTarget::SpecificPanel("terrain".to_string()),
            ActionFamily::SkyEnvironment => FocusTarget::SpecificPanel("sky_lighting".to_string()),
            ActionFamily::Runtime => FocusTarget::SpecificPanel("viewport".to_string()),
            ActionFamily::DiagnosticsProof => FocusTarget::Diagnostics,
            ActionFamily::BuildRelease => FocusTarget::SpecificPanel("build_release".to_string()),
            ActionFamily::ViewPanel => FocusTarget::NoChange,
            ActionFamily::ProjectFile => FocusTarget::NoChange,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ActionId, FocusTarget, MutationClass};

    fn make_def(family: ActionFamily) -> ActionDefinition {
        ActionDefinition {
            action_id: ActionId::new("test"),
            display_label: "Test".to_string(),
            action_family: family,
            tooling_route: "route.test.v1".to_string(),
            sdk_packet_family: "packet.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            possible_denial_families: vec![],
        }
    }

    fn make_def_with_id(family: ActionFamily, action_id: &str) -> ActionDefinition {
        ActionDefinition {
            action_id: ActionId::new(action_id),
            display_label: "Test".to_string(),
            action_family: family,
            tooling_route: "route.test.v1".to_string(),
            sdk_packet_family: "packet.*".to_string(),
            engine_truth_owner: "engine/test".to_string(),
            mutation_class: MutationClass::Mutate,
            possible_denial_families: vec![],
        }
    }

    #[test]
    fn test_focus_world() {
        assert_eq!(
            ActionFocus::determine(&make_def(ActionFamily::World)),
            FocusTarget::SpecificPanel("viewport".to_string())
        );
    }

    #[test]
    fn test_focus_material() {
        assert_eq!(
            ActionFocus::determine(&make_def(ActionFamily::Material)),
            FocusTarget::SpecificPanel("material".to_string())
        );
    }

    #[test]
    fn test_focus_audio() {
        assert_eq!(
            ActionFocus::determine(&make_def(ActionFamily::Audio)),
            FocusTarget::SpecificPanel("audio".to_string())
        );
    }

    #[test]
    fn test_focus_terrain() {
        assert_eq!(
            ActionFocus::determine(&make_def(ActionFamily::Terrain)),
            FocusTarget::SpecificPanel("terrain".to_string())
        );
    }

    #[test]
    fn test_focus_view_panel() {
        assert_eq!(
            ActionFocus::determine(&make_def(ActionFamily::ViewPanel)),
            FocusTarget::NoChange
        );
    }

    #[test]
    fn test_focus_project_file() {
        assert_eq!(
            ActionFocus::determine(&make_def(ActionFamily::ProjectFile)),
            FocusTarget::NoChange
        );
    }

    #[test]
    fn test_focus_compare_action() {
        // Requirement 25.4: Compare actions focus Compare panel
        let def = make_def_with_id(ActionFamily::Material, "material.compare");
        assert_eq!(
            ActionFocus::determine(&def),
            FocusTarget::SpecificPanel("compare".to_string())
        );
    }

    #[test]
    fn test_focus_capture_action() {
        // Requirement 25.5: Capture actions focus Evidence panel
        let def = make_def_with_id(ActionFamily::Material, "material.capture");
        assert_eq!(
            ActionFocus::determine(&def),
            FocusTarget::SpecificPanel("evidence".to_string())
        );
    }

    #[test]
    fn test_focus_certify_action() {
        // Requirement 25.6: Certify actions focus Certification panel
        let def = make_def_with_id(ActionFamily::Material, "material.certify");
        assert_eq!(
            ActionFocus::determine(&def),
            FocusTarget::SpecificPanel("certification".to_string())
        );
    }
}

use super::{UiStateClassificationValidator, UiStateViolation, UiStateViolationType};
use crate::{CodebaseState, StateClassification};

impl UiStateClassificationValidator {
    pub(super) fn check_app_state_no_business_truth(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<UiStateViolation>> {
        let mut violations = Vec::new();
        let business_truth_patterns = [
            "project_identity",
            "workspace_identity",
            "world_identity",
            "registry",
            "manifest",
            "bindings",
            "coverage",
            "terrain_state",
            "environment_state",
            "material_state",
            "audio_state",
            "save_generation",
            "chunk_dirtiness",
            "layer_bindings",
        ];

        for field in &codebase_state.state_fields {
            let is_in_app_state = field.file_path.contains("app_state.rs")
                || field.file_path.contains("desktop_app/state/")
                || field.owner_type.contains("AppState")
                || field.owner_type.contains("EditorAppRuntime");
            if !is_in_app_state {
                continue;
            }

            let is_business_truth = business_truth_patterns
                .iter()
                .any(|pattern| field.name.contains(pattern));
            let is_persistable = self
                .inventory
                .find_entry(&field.name)
                .map(|entry| matches!(entry.classification, StateClassification::Persistable))
                .unwrap_or(false);

            if is_business_truth || is_persistable {
                violations.push(UiStateViolation {
                    component_name: field.owner_type.clone(),
                    file_path: field.file_path.clone(),
                    field_name: field.name.clone(),
                    violation_type: UiStateViolationType::BusinessTruthInAppState,
                    message: format!(
                        "Business truth field '{}' found in app_state at {}. app_state should contain only transient UI state and wiring.",
                        field.name, field.file_path
                    ),
                });
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

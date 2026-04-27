use super::{UiStateClassificationValidator, UiStateViolation, UiStateViolationType};
use crate::{CodebaseState, StateClassification};

impl UiStateClassificationValidator {
    pub(super) fn check_panel_state_transient_only(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<UiStateViolation>> {
        let mut violations = Vec::new();
        let panel_patterns = ["Panel", "panel", "Surface", "surface"];
        let non_transient_patterns = [
            "registry",
            "manifest",
            "bindings",
            "coverage",
            "truth",
            "authoritative",
            "canonical",
        ];

        for field in &codebase_state.state_fields {
            let is_panel = panel_patterns.iter().any(|pattern| {
                field.owner_type.contains(pattern) || field.file_path.contains(pattern)
            });
            let is_in_ui = field.file_path.contains("6.apps/editor");
            if !is_panel || !is_in_ui {
                continue;
            }

            let is_non_transient = non_transient_patterns
                .iter()
                .any(|pattern| field.name.contains(pattern));
            let is_persistable_or_derived = self
                .inventory
                .find_entry(&field.name)
                .map(|entry| {
                    matches!(
                        entry.classification,
                        StateClassification::Persistable | StateClassification::Derived
                    )
                })
                .unwrap_or(false);

            if is_non_transient || is_persistable_or_derived {
                violations.push(UiStateViolation {
                    component_name: field.owner_type.clone(),
                    file_path: field.file_path.clone(),
                    field_name: field.name.clone(),
                    violation_type: UiStateViolationType::NonTransientInPanel,
                    message: format!(
                        "Non-transient state field '{}' found in panel '{}' at {}. Panels should contain only transient/UI-local state (hover, drag, focus, selection, search filters).",
                        field.name, field.owner_type, field.file_path
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

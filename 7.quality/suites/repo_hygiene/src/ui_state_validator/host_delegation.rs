use super::{UiStateClassificationValidator, UiStateViolation, UiStateViolationType};
use crate::{CodebaseState, StateClassification};

impl UiStateClassificationValidator {
    pub(super) fn check_editor_host_delegates(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<UiStateViolation>> {
        let mut violations = Vec::new();
        let business_logic_patterns = [
            "create_",
            "update_",
            "delete_",
            "modify_",
            "process_",
            "compute_",
            "calculate_",
            "validate_",
            "transform_",
            "apply_",
        ];
        let service_patterns = ["Service", "service"];

        for field in &codebase_state.state_fields {
            let is_editor_host =
                field.owner_type.contains("EditorHost") || field.file_path.contains("editor_host");
            if !is_editor_host {
                continue;
            }

            let is_service_ref = service_patterns
                .iter()
                .any(|pattern| field.field_type.contains(pattern));
            let is_business_logic = business_logic_patterns
                .iter()
                .any(|pattern| field.name.contains(pattern));
            let is_persistable = self
                .inventory
                .find_entry(&field.name)
                .map(|entry| matches!(entry.classification, StateClassification::Persistable))
                .unwrap_or(false);

            if !is_service_ref && (is_business_logic || is_persistable) {
                violations.push(UiStateViolation {
                    component_name: field.owner_type.clone(),
                    file_path: field.file_path.clone(),
                    field_name: field.name.clone(),
                    violation_type: UiStateViolationType::EditorHostBusinessLogic,
                    message: format!(
                        "Business logic or state field '{}' found in EditorHost at {}. EditorHost should be a thin facade that delegates to services.",
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

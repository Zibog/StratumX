use super::{PersistenceSeparationValidator, PersistenceViolation, PersistenceViolationType};
use crate::CodebaseState;

impl PersistenceSeparationValidator {
    pub(super) fn check_owner_not_equal_persistence_view(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<PersistenceViolation>> {
        let mut violations = Vec::new();
        let owner_types = [
            "ProjectOwner",
            "WorkspaceOwner",
            "WorldOwner",
            "DiagnosticsOwner",
        ];

        for field in &codebase_state.state_fields {
            if !owner_types.contains(&field.owner_type.as_str()) {
                continue;
            }
            if !field.file_path.contains("owners/") || field.file_path.contains("persistence/") {
                continue;
            }

            let expected_persistence_view =
                format!("{}PersistenceView", field.owner_type.replace("Owner", ""));
            let has_persistence_view = codebase_state.state_fields.iter().any(|candidate| {
                candidate.owner_type == expected_persistence_view
                    && candidate.file_path.contains("persistence/")
            });

            if !has_persistence_view {
                violations.push(PersistenceViolation {
                    file_path: field.file_path.clone(),
                    violation_type: PersistenceViolationType::OwnerEqualsPersistenceView,
                    message: format!(
                        "Owner type '{}' in {} may not have a separate persistence view. Expected to find '{}'",
                        field.owner_type, field.file_path, expected_persistence_view
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

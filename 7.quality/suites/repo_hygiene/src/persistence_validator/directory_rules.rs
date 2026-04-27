use super::{PersistenceSeparationValidator, PersistenceViolation, PersistenceViolationType};
use crate::CodebaseState;

impl PersistenceSeparationValidator {
    pub(super) fn check_persistence_in_correct_directory(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<PersistenceViolation>> {
        let mut violations = Vec::new();
        let persistence_view_suffixes = ["PersistenceView", "PersistedView", "SaveView"];

        for field in &codebase_state.state_fields {
            let is_persistence_view = persistence_view_suffixes
                .iter()
                .any(|suffix| field.owner_type.ends_with(suffix));
            if !is_persistence_view {
                continue;
            }

            if !field.file_path.contains("persistence/") {
                violations.push(PersistenceViolation {
                    file_path: field.file_path.clone(),
                    violation_type: PersistenceViolationType::WrongDirectory,
                    message: format!(
                        "Persistence view type '{}' found in {} but should be in persistence/ directory",
                        field.owner_type, field.file_path
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

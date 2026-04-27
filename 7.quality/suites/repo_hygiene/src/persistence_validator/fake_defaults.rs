use super::{PersistenceSeparationValidator, PersistenceViolation, PersistenceViolationType};
use crate::{CodebaseState, StateClassification};

impl PersistenceSeparationValidator {
    pub(super) fn check_no_fake_defaults(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<PersistenceViolation>> {
        let mut violations = Vec::new();
        let fake_default_patterns = [
            "unwrap_or_default",
            "get_or_insert",
            "or_insert_with",
            ".unwrap_or(",
        ];

        for field in &codebase_state.state_fields {
            if !field.file_path.contains("persistence/") {
                continue;
            }

            let is_persistable = self
                .inventory
                .find_entry(&field.name)
                .map(|entry| entry.classification == StateClassification::Persistable)
                .unwrap_or(false);
            if !is_persistable {
                continue;
            }

            for pattern in &fake_default_patterns {
                if field.field_type.contains(pattern) {
                    violations.push(PersistenceViolation {
                        file_path: field.file_path.clone(),
                        violation_type: PersistenceViolationType::FakeDefaultsInPersistence,
                        message: format!(
                            "Persistence field '{}' in {} contains fake default pattern '{}'. Persistence should not have hardcoded defaults.",
                            field.name, field.file_path, pattern
                        ),
                    });
                }
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

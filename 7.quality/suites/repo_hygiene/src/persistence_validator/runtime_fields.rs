use super::{PersistenceSeparationValidator, PersistenceViolation, PersistenceViolationType};
use crate::{CodebaseState, StateClassification};

impl PersistenceSeparationValidator {
    pub(super) fn check_no_runtime_fields_in_persistence(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<PersistenceViolation>> {
        let mut violations = Vec::new();
        let runtime_field_patterns = [
            "event_bus",
            "cache",
            "handle",
            "runtime",
            "transient",
            "temporary",
            "buffer",
        ];

        for field in &codebase_state.state_fields {
            if !field.file_path.contains("persistence/") || !field.file_path.contains("_view.rs") {
                continue;
            }

            for pattern in &runtime_field_patterns {
                if field.name.to_lowercase().contains(pattern) {
                    violations.push(PersistenceViolation {
                        file_path: field.file_path.clone(),
                        violation_type: PersistenceViolationType::RuntimeFieldInPersistence,
                        message: format!(
                            "Runtime-only field '{}' found in persistence view at {}. Runtime fields should not be persisted.",
                            field.name, field.file_path
                        ),
                    });
                }
            }

            let is_transient = self
                .inventory
                .find_entry(&field.name)
                .map(|entry| entry.classification == StateClassification::Transient)
                .unwrap_or(false);
            if is_transient {
                violations.push(PersistenceViolation {
                    file_path: field.file_path.clone(),
                    violation_type: PersistenceViolationType::RuntimeFieldInPersistence,
                    message: format!(
                        "Transient field '{}' found in persistence view at {}. Transient state should not be persisted.",
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

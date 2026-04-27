use crate::{CodebaseState, OwnerInventory};

mod directory_rules;
mod fake_defaults;
mod owner_view_separation;
mod runtime_fields;

/// Validator for persistence separation rules.
pub struct PersistenceSeparationValidator {
    inventory: OwnerInventory,
}

impl PersistenceSeparationValidator {
    pub fn new(inventory: OwnerInventory) -> Self {
        Self { inventory }
    }

    /// Validate persistence separation rules.
    pub fn validate(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<PersistenceViolation>> {
        let mut violations = Vec::new();

        if let Err(mut found) = self.check_owner_not_equal_persistence_view(codebase_state) {
            violations.append(&mut found);
        }

        if let Err(mut found) = self.check_no_fake_defaults(codebase_state) {
            violations.append(&mut found);
        }

        if let Err(mut found) = self.check_no_runtime_fields_in_persistence(codebase_state) {
            violations.append(&mut found);
        }

        if let Err(mut found) = self.check_persistence_in_correct_directory(codebase_state) {
            violations.append(&mut found);
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

/// Represents a violation of persistence separation rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceViolation {
    pub file_path: String,
    pub violation_type: PersistenceViolationType,
    pub message: String,
}

/// Types of persistence violations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceViolationType {
    /// Owner type is the same as persistence view (not separated).
    OwnerEqualsPersistenceView,
    /// Fake defaults found in persistence (hardcoded fallback values).
    FakeDefaultsInPersistence,
    /// Runtime-only field found in persistence view.
    RuntimeFieldInPersistence,
    /// Persistence view in wrong directory.
    WrongDirectory,
}

#[cfg(test)]
mod tests;

use crate::{CodebaseState, OwnerInventory};

mod app_state_truth;
mod host_delegation;
mod panel_state;

/// Validator for UI state classification rules.
pub struct UiStateClassificationValidator {
    inventory: OwnerInventory,
}

impl UiStateClassificationValidator {
    pub fn new(inventory: OwnerInventory) -> Self {
        Self { inventory }
    }

    /// Validate UI state classification rules.
    pub fn validate(&self, codebase_state: &CodebaseState) -> Result<(), Vec<UiStateViolation>> {
        let mut violations = Vec::new();

        if let Err(mut found) = self.check_app_state_no_business_truth(codebase_state) {
            violations.append(&mut found);
        }

        if let Err(mut found) = self.check_panel_state_transient_only(codebase_state) {
            violations.append(&mut found);
        }

        if let Err(mut found) = self.check_editor_host_delegates(codebase_state) {
            violations.append(&mut found);
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

/// Represents a violation of UI state classification rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiStateViolation {
    pub component_name: String,
    pub file_path: String,
    pub field_name: String,
    pub violation_type: UiStateViolationType,
    pub message: String,
}

/// Types of UI state violations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiStateViolationType {
    /// Business/domain truth found in app_state.
    BusinessTruthInAppState,
    /// Non-transient state found in UI panel.
    NonTransientInPanel,
    /// Business logic found in EditorHost (should delegate to services).
    EditorHostBusinessLogic,
}

#[cfg(test)]
mod tests;

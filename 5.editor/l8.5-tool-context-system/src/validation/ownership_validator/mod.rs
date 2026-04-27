//! Ownership Validator
//!
//! Validates state ownership uniqueness, completeness, and acyclicity.

mod acyclic;
mod completeness;

use std::sync::Arc;

use crate::runtime::state_container_system::{OwnershipViolation, StateContainerSystem};
use crate::state_graph::StateGraph;

/// Ownership validator
///
/// Validates that all state has exactly one owner and no circular dependencies.
pub struct OwnershipValidator {
    state_system: Arc<StateContainerSystem>,
    state_graph: Option<Arc<StateGraph>>,
}

impl OwnershipValidator {
    /// Creates a new ownership validator
    pub fn new(state_system: Arc<StateContainerSystem>) -> Self {
        Self {
            state_system,
            state_graph: None,
        }
    }

    /// Sets the state graph for acyclicity validation
    pub fn with_state_graph(mut self, state_graph: Arc<StateGraph>) -> Self {
        self.state_graph = Some(state_graph);
        self
    }

    /// Validates all ownership constraints
    ///
    /// Returns Ok(()) if all validations pass, or Err with all violations
    pub fn validate(&self) -> Result<(), Vec<OwnershipViolation>> {
        let mut all_violations = Vec::new();

        if let Err(mut violations) = self.check_uniqueness() {
            all_violations.append(&mut violations);
        }

        if let Err(mut violations) = self.check_completeness() {
            all_violations.append(&mut violations);
        }

        if let Err(mut violations) = self.check_acyclic() {
            all_violations.append(&mut violations);
        }

        if all_violations.is_empty() {
            Ok(())
        } else {
            Err(all_violations)
        }
    }

    /// Checks that no state has multiple owners
    pub fn check_uniqueness(&self) -> Result<(), Vec<OwnershipViolation>> {
        self.state_system.validate_ownership_uniqueness()
    }
}

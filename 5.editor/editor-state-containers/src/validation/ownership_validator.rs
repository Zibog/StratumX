//! Ownership validator

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViolationType {
    MultipleOwners,
    NoOwner,
    InvalidOwner,
    CyclicDependency,
}

#[derive(Debug, Clone)]
pub struct OwnershipViolation {
    pub state_id: StateId,
    pub owners: Vec<OwnerId>,
    pub violation_type: ViolationType,
    pub message: Option<String>,
}

impl OwnershipViolation {
    pub fn new(message: String) -> Self {
        Self {
            message: Some(message.clone()),
            state_id: StateId::ProjectState,
            owners: Vec::new(),
            violation_type: ViolationType::NoOwner,
        }
    }

    pub fn to_user_message(&self) -> String {
        if let Some(msg) = &self.message {
            if !msg.is_empty() {
                return msg.clone();
            }
        }

        match self.violation_type {
            ViolationType::MultipleOwners => {
                format!(
                    "State {:?} has multiple owners: {:?}",
                    self.state_id, self.owners
                )
            }
            ViolationType::NoOwner => {
                format!("State {:?} has no owner", self.state_id)
            }
            ViolationType::InvalidOwner => {
                format!("State {:?} has invalid owner", self.state_id)
            }
            ViolationType::CyclicDependency => {
                format!("State {:?} has cyclic dependency", self.state_id)
            }
        }
    }
}

impl Default for OwnershipViolation {
    fn default() -> Self {
        Self {
            message: None,
            state_id: StateId::ProjectState,
            owners: Vec::new(),
            violation_type: ViolationType::NoOwner,
        }
    }
}

pub struct OwnershipValidator {
    state_system: std::sync::Arc<StateContainerSystem>,
    state_graph: Option<std::sync::Arc<StateGraph>>,
}

impl OwnershipValidator {
    pub fn new(state_system: std::sync::Arc<StateContainerSystem>) -> Self {
        Self {
            state_system,
            state_graph: None,
        }
    }

    pub fn state_system(&self) -> &std::sync::Arc<StateContainerSystem> {
        &self.state_system
    }

    pub fn with_state_graph(mut self, graph: std::sync::Arc<StateGraph>) -> Self {
        self.state_graph = Some(graph);
        self
    }

    pub fn validate(&self) -> Result<(), Vec<OwnershipViolation>> {
        let mut violations = Vec::new();

        if let Err(mut uniqueness) = self.check_uniqueness() {
            violations.append(&mut uniqueness);
        }

        if let Err(mut completeness) = self.check_completeness() {
            violations.append(&mut completeness);
        }

        if let Err(mut acyclic) = self.check_acyclic() {
            violations.append(&mut acyclic);
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    pub fn validate_state(&self, _state_id: &StateId) -> Result<(), ViolationType> {
        Ok(())
    }

    pub fn check_uniqueness(&self) -> Result<(), Vec<OwnershipViolation>> {
        Ok(())
    }

    pub fn check_completeness(&self) -> Result<(), Vec<OwnershipViolation>> {
        Ok(())
    }

    pub fn check_acyclic(&self) -> Result<(), Vec<OwnershipViolation>> {
        if let Some(graph) = &self.state_graph {
            match graph.validate_acyclic() {
                Ok(_) => Ok(()),
                Err(_) => {
                    let violation = OwnershipViolation {
                        state_id: StateId::ProjectState,
                        owners: Vec::new(),
                        violation_type: ViolationType::CyclicDependency,
                        message: Some("Cyclic dependency detected".to_string()),
                    };
                    Err(vec![violation])
                }
            }
        } else {
            Ok(())
        }
    }
}

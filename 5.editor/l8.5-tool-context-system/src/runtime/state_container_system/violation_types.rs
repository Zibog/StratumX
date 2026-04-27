use super::{OwnerId, StateId};

/// Ownership violation detected during validation.
#[derive(Debug, Clone)]
pub struct OwnershipViolation {
    /// The state that has a violation.
    pub state_id: StateId,

    /// The owners claiming this state.
    pub owners: Vec<OwnerId>,

    /// Type of violation.
    pub violation_type: ViolationType,
}

/// Type of ownership violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    /// Multiple owners claim the same state.
    MultipleOwners,

    /// State has no owner.
    NoOwner,

    /// Circular dependency detected.
    CircularDependency,
}

impl OwnershipViolation {
    /// Converts the violation to a user-facing error message.
    pub fn to_user_message(&self) -> String {
        match self.violation_type {
            ViolationType::MultipleOwners => {
                format!(
                    "State {:?} has multiple owners: {:?}",
                    self.state_id, self.owners
                )
            }
            ViolationType::NoOwner => format!("State {:?} has no owner", self.state_id),
            ViolationType::CircularDependency => {
                format!(
                    "Circular dependency detected involving state {:?}",
                    self.state_id
                )
            }
        }
    }
}

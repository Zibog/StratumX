use super::OwnershipValidator;
use crate::runtime::state_container_system::{
    OwnershipViolation, StateContainerSystem, ViolationType,
};

impl OwnershipValidator {
    /// Checks that all expected state has an owner (no orphaned state)
    pub fn check_completeness(&self) -> Result<(), Vec<OwnershipViolation>> {
        let mut violations = Vec::new();

        violations
            .extend(self.missing_owner_violations(StateContainerSystem::always_required_states()));

        if self.state_system.has_world_state() {
            violations.extend(
                self.missing_owner_violations(StateContainerSystem::world_required_states()),
            );
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    fn missing_owner_violations(
        &self,
        required_states: &[crate::StateId],
    ) -> Vec<OwnershipViolation> {
        required_states
            .iter()
            .filter(|state_id| self.state_system.get_owner(state_id).is_none())
            .map(|state_id| OwnershipViolation {
                state_id: *state_id,
                owners: Vec::new(),
                violation_type: ViolationType::NoOwner,
            })
            .collect()
    }
}

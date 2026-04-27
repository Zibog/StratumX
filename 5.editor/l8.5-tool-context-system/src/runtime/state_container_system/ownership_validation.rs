use std::collections::HashMap;

use super::{OwnershipViolation, StateContainerSystem, StateId, ViolationType};

impl StateContainerSystem {
    /// Validates that all state has exactly one owner.
    ///
    /// Returns Ok(()) if validation passes, or Err with a list of violations.
    pub fn validate_ownership_uniqueness(&self) -> Result<(), Vec<OwnershipViolation>> {
        let mut violations = Vec::new();
        let mut state_owner_counts: HashMap<StateId, Vec<_>> = HashMap::new();

        for (state_id, owner_id) in &self.ownership_map {
            state_owner_counts
                .entry(*state_id)
                .or_default()
                .push(*owner_id);
        }

        for (state_id, owners) in state_owner_counts {
            if owners.len() > 1 {
                violations.push(OwnershipViolation {
                    state_id,
                    owners,
                    violation_type: ViolationType::MultipleOwners,
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

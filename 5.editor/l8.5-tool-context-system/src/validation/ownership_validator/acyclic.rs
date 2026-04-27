use super::OwnershipValidator;
use crate::runtime::state_container_system::{OwnershipViolation, ViolationType};

impl OwnershipValidator {
    /// Checks that the state graph has no circular dependencies
    pub fn check_acyclic(&self) -> Result<(), Vec<OwnershipViolation>> {
        if let Some(graph) = &self.state_graph {
            match graph.validate_acyclic() {
                Ok(()) => Ok(()),
                Err(cycle) => Err(cycle
                    .into_iter()
                    .map(|state_id| OwnershipViolation {
                        state_id,
                        owners: Vec::new(),
                        violation_type: ViolationType::CircularDependency,
                    })
                    .collect()),
            }
        } else {
            Ok(())
        }
    }
}

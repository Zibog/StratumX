use crate::types::UpdateOrderGraph;

use super::SubstrateValidationError;

/// Validate update order graph has no cycles.
pub fn validate_update_order(graph: &UpdateOrderGraph) -> Result<(), SubstrateValidationError> {
    graph
        .topological_sort()
        .map(|_| ())
        .map_err(|_| SubstrateValidationError::CycleDetected)
}

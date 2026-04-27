use engine_core::Generation;
use engine_identity::{ComponentId, EntityId};
use serde::{Deserialize, Serialize};

/// Context for handle validation operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationContext {
    /// Entity enters system boundary.
    BoundaryEntry,
    /// Diagnostic inspection pass.
    Diagnostics,
    /// Query plan compilation phase.
    PlanBuild,
    /// Steady-state traversal (restrictive context).
    SteadyTraversal,
}

/// Current state of a handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvalidationState {
    /// Handle points to active entity/component.
    Active,
    /// Handle has been explicitly invalidated.
    Invalidated,
}

/// Result of validating a handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationResult {
    /// Handle is valid and current.
    Valid,
    /// Handle was explicitly invalidated.
    Invalidated,
    /// Handle points to stale or reclaimed slot.
    Stale,
    /// Validation context does not allow this operation.
    IllegalContext,
}

/// Stable handle for entity identity with generation tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableEntityHandle {
    /// Entity identity.
    pub id: EntityId,
    /// Generation observed when handle was created.
    pub observed_generation: Generation,
    /// Current invalidation state.
    pub state: InvalidationState,
}

/// Stable handle for component identity with generation tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableComponentHandle {
    /// Component identity.
    pub id: ComponentId,
    /// Generation observed when handle was created.
    pub observed_generation: Generation,
    /// Current invalidation state.
    pub state: InvalidationState,
}

/// Opaque dense execution handle (slot-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DenseExecutionHandle(pub(crate) u32);

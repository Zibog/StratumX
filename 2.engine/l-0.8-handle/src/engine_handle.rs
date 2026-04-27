use engine_core::Generation;
use engine_identity::{ComponentId, EntityId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationContext {
    BoundaryEntry,
    Diagnostics,
    PlanBuild,
    SteadyTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvalidationState {
    Active,
    Invalidated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationResult {
    Valid,
    Invalidated,
    Stale,
    IllegalContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableEntityHandle {
    pub id: EntityId,
    pub observed_generation: Generation,
    pub state: InvalidationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableComponentHandle {
    pub id: ComponentId,
    pub observed_generation: Generation,
    pub state: InvalidationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DenseExecutionHandle(pub(crate) u32);

impl StableEntityHandle {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            observed_generation: id.generation,
            state: InvalidationState::Active,
        }
    }

    pub fn invalidate(&mut self) {
        self.state = InvalidationState::Invalidated;
    }

    pub fn validate(&self, current: EntityId, context: ValidationContext) -> ValidationResult {
        if matches!(context, ValidationContext::SteadyTraversal) {
            return ValidationResult::IllegalContext;
        }
        if self.state == InvalidationState::Invalidated {
            return ValidationResult::Invalidated;
        }
        if self.id.slot != current.slot || self.observed_generation != current.generation {
            return ValidationResult::Stale;
        }
        ValidationResult::Valid
    }
}

impl StableComponentHandle {
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            observed_generation: id.generation,
            state: InvalidationState::Active,
        }
    }

    pub fn invalidate(&mut self) {
        self.state = InvalidationState::Invalidated;
    }

    pub fn validate(&self, current: ComponentId, context: ValidationContext) -> ValidationResult {
        if matches!(context, ValidationContext::SteadyTraversal) {
            return ValidationResult::IllegalContext;
        }
        if self.state == InvalidationState::Invalidated {
            return ValidationResult::Invalidated;
        }
        if self.id.slot != current.slot || self.observed_generation != current.generation {
            return ValidationResult::Stale;
        }
        ValidationResult::Valid
    }
}

use crate::types::{
    InvalidationState, StableComponentHandle, StableEntityHandle, ValidationContext,
    ValidationResult,
};
use engine_identity::{ComponentId, EntityId};

impl StableEntityHandle {
    /// Create a new entity handle from an entity identity.
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            observed_generation: id.generation,
            state: InvalidationState::Active,
        }
    }

    /// Mark this handle as invalidated.
    pub fn invalidate(&mut self) {
        self.state = InvalidationState::Invalidated;
    }

    /// Validate this handle against current entity state.
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

    /// Get the entity ID this handle points to.
    pub fn entity_id(&self) -> EntityId {
        self.id
    }

    /// Check if this handle is active.
    pub fn is_active(&self) -> bool {
        self.state == InvalidationState::Active
    }
}

impl StableComponentHandle {
    /// Create a new component handle from a component identity.
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            observed_generation: id.generation,
            state: InvalidationState::Active,
        }
    }

    /// Mark this handle as invalidated.
    pub fn invalidate(&mut self) {
        self.state = InvalidationState::Invalidated;
    }

    /// Validate this handle against current component state.
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

    /// Get the component ID this handle points to.
    pub fn component_id(&self) -> ComponentId {
        self.id
    }

    /// Check if this handle is active.
    pub fn is_active(&self) -> bool {
        self.state == InvalidationState::Active
    }
}

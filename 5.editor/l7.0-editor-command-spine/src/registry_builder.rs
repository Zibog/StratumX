//! Registry Builder - Action Registration Builder
//!
//! The RegistryBuilder provides a builder pattern for constructing an ActionRegistry.
//! It validates action definitions during registration and builds an immutable
//! registry once all actions are registered.

use crate::registry_core::ActionRegistry;
use crate::{ActionDefinition, ActionId};
use std::collections::HashMap;

/// Builder for constructing an ActionRegistry.
///
/// The RegistryBuilder validates action definitions during registration
/// and constructs an immutable ActionRegistry when build() is called.
///
/// # Example
///
/// ```ignore
/// let mut builder = RegistryBuilder::new();
/// builder.register_action(definition1)?;
/// builder.register_action(definition2)?;
/// let registry = builder.build();
/// ```
#[derive(Debug, Default)]
pub struct RegistryBuilder {
    /// Pending action definitions to be registered
    actions: HashMap<ActionId, ActionDefinition>,
}

/// Error type for registration validation failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationError {
    /// Action ID is empty or invalid
    InvalidActionId(String),
    /// Display label is empty
    EmptyDisplayLabel(String),
    /// Tooling route is empty
    EmptyToolingRoute(String),
    /// SDK packet family is empty
    EmptySdkPacketFamily(String),
    /// Engine truth owner is empty
    EmptyEngineTruthOwner(String),
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationError::InvalidActionId(id) => {
                write!(f, "Invalid action ID: '{}'", id)
            }
            RegistrationError::EmptyDisplayLabel(id) => {
                write!(f, "Empty display label for action '{}'", id)
            }
            RegistrationError::EmptyToolingRoute(id) => {
                write!(f, "Empty tooling route for action '{}'", id)
            }
            RegistrationError::EmptySdkPacketFamily(id) => {
                write!(f, "Empty SDK packet family for action '{}'", id)
            }
            RegistrationError::EmptyEngineTruthOwner(id) => {
                write!(f, "Empty engine truth owner for action '{}'", id)
            }
        }
    }
}

impl std::error::Error for RegistrationError {}

impl RegistryBuilder {
    /// Creates a new empty RegistryBuilder.
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    /// Registers an action with validation.
    ///
    /// Validates that all required metadata fields are present and non-empty.
    /// If an action with the same ActionId already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `definition` - Complete ActionDefinition to register
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the definition is valid and registered
    /// * `Err(RegistrationError)` - If validation fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut builder = RegistryBuilder::new();
    /// builder.register_action(ActionDefinition {
    ///     action_id: ActionId::new("world.open"),
    ///     display_label: "Open World".to_string(),
    ///     action_family: ActionFamily::World,
    ///     tooling_route: "route.world.open.v1".to_string(),
    ///     sdk_packet_family: "packet.world.*".to_string(),
    ///     engine_truth_owner: "engine/50".to_string(),
    ///     mutation_class: MutationClass::Mutate,
    ///     possible_denial_families: vec![],
    ///         /// })?;
    /// ```
    pub fn register_action(
        &mut self,
        definition: ActionDefinition,
    ) -> Result<(), RegistrationError> {
        // Validate the definition
        self.validate_definition(&definition)?;

        // Register the action (replaces existing if present)
        self.actions
            .insert(definition.action_id.clone(), definition);

        Ok(())
    }

    /// Builds an immutable ActionRegistry from registered actions.
    ///
    /// Consumes the builder and returns an ActionRegistry.
    /// After calling build(), the builder cannot be used again.
    ///
    /// # Returns
    ///
    /// An immutable ActionRegistry containing all registered actions.
    pub fn build(self) -> ActionRegistry {
        ActionRegistry::new(self.actions)
    }

    /// Validates an action definition.
    ///
    /// Checks that all required metadata fields are present and non-empty:
    /// - action_id must be non-empty
    /// - display_label must be non-empty
    /// - tooling_route must be non-empty
    /// - sdk_packet_family must be non-empty
    /// - engine_truth_owner must be non-empty
    ///
    /// # Arguments
    ///
    /// * `definition` - The ActionDefinition to validate
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If all required fields are valid
    /// * `Err(RegistrationError)` - If any required field is missing or invalid
    fn validate_definition(&self, definition: &ActionDefinition) -> Result<(), RegistrationError> {
        // Validate action_id is non-empty
        if definition.action_id.as_str().trim().is_empty() {
            return Err(RegistrationError::InvalidActionId(
                definition.action_id.as_str().to_string(),
            ));
        }

        // Validate display_label is non-empty
        if definition.display_label.trim().is_empty() {
            return Err(RegistrationError::EmptyDisplayLabel(
                definition.action_id.as_str().to_string(),
            ));
        }

        // Validate tooling_route is non-empty
        if definition.tooling_route.trim().is_empty() {
            return Err(RegistrationError::EmptyToolingRoute(
                definition.action_id.as_str().to_string(),
            ));
        }

        // Validate sdk_packet_family is non-empty
        if definition.sdk_packet_family.trim().is_empty() {
            return Err(RegistrationError::EmptySdkPacketFamily(
                definition.action_id.as_str().to_string(),
            ));
        }

        // Validate engine_truth_owner is non-empty
        if definition.engine_truth_owner.trim().is_empty() {
            return Err(RegistrationError::EmptyEngineTruthOwner(
                definition.action_id.as_str().to_string(),
            ));
        }

        Ok(())
    }

    /// Returns the number of actions currently registered in the builder.
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Returns true if no actions have been registered yet.
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}

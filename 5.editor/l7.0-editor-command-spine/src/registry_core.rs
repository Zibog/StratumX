//! Registry Core - Action Registry Logic
//!
//! The ActionRegistry maintains a HashMap of all available editor actions,
//! mapping ActionId to ActionDefinition. It provides methods to look up
//! handlers and query available actions based on context.

use crate::{ActionContext, ActionDefinition, ActionId};
use std::collections::HashMap;

/// Centralized registry of all editor action identifiers and their routing configurations.
///
/// The ActionRegistry stores ActionDefinitions in a HashMap keyed by ActionId
/// and provides methods to look up handlers and query available actions based on context.
///
/// The registry is immutable after initialization - all actions must be registered
/// during the build phase using RegistryBuilder.
#[derive(Debug)]
pub struct ActionRegistry {
    /// Map of action IDs to their complete definitions
    actions: HashMap<ActionId, ActionDefinition>,
    /// Domain index for efficient filtering by action family
    domain_index: HashMap<crate::ActionFamily, Vec<ActionId>>,
}

impl ActionRegistry {
    /// Creates a new ActionRegistry from a HashMap of actions.
    /// This is an internal constructor used by RegistryBuilder.
    pub(crate) fn new(actions: HashMap<ActionId, ActionDefinition>) -> Self {
        let mut domain_index: HashMap<crate::ActionFamily, Vec<ActionId>> = HashMap::new();
        for (action_id, definition) in &actions {
            domain_index
                .entry(definition.action_family)
                .or_default()
                .push(action_id.clone());
        }
        Self {
            actions,
            domain_index,
        }
    }

    /// Resolves an action by ActionId.
    /// Returns a reference to the ActionDefinition if registered, or None if not found.
    pub fn resolve(&self, action_id: &ActionId) -> Option<&ActionDefinition> {
        self.actions.get(action_id)
    }

    /// Queries actions for a specific domain (action family).
    /// Returns all action IDs that belong to the specified action family.
    pub fn query_actions_for_domain(&self, family: crate::ActionFamily) -> &[ActionId] {
        self.domain_index
            .get(&family)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Queries available actions for a given ActionContext.
    /// This method filters registered actions based on the provided context,
    /// returning only actions that could potentially execute given the current state.
    /// Note: Full validation (including disabled reasons) is performed during action dispatch.
    pub fn query_available_actions(&self, context: &ActionContext) -> Vec<ActionId> {
        self.actions
            .values()
            .filter(|definition| self.is_action_potentially_available(definition, context))
            .map(|definition| definition.action_id.clone())
            .collect()
    }

    /// Checks if an action is potentially available given the context.
    /// This is a basic filter that checks project, world, and selection requirements.
    fn is_action_potentially_available(
        &self,
        definition: &ActionDefinition,
        context: &ActionContext,
    ) -> bool {
        use crate::ActionFamily;
        match definition.action_family {
            ActionFamily::ProjectFile => {
                if definition.action_id.as_str().contains("new") {
                    return true;
                }
                context.has_project()
            }
            ActionFamily::World => {
                if definition.action_id.as_str().contains("open") {
                    return context.has_project();
                }
                context.has_project() && context.has_active_world()
            }
            ActionFamily::Terrain | ActionFamily::Material | ActionFamily::SkyEnvironment => {
                context.has_project() && context.has_active_world()
            }
            ActionFamily::Audio => {
                context.has_project() && context.has_active_world() && context.has_audio_registry()
            }
            ActionFamily::Runtime => context.has_project() && context.has_active_world(),
            ActionFamily::DiagnosticsProof | ActionFamily::BuildRelease => context.has_project(),
            ActionFamily::ViewPanel => true,
        }
    }

    /// Returns the total number of registered actions.
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Returns true if the registry contains no actions.
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Returns an iterator over all registered action IDs.
    pub fn action_ids(&self) -> impl Iterator<Item = &ActionId> {
        self.actions.keys()
    }

    /// Returns an iterator over all registered action definitions.
    pub fn definitions(&self) -> impl Iterator<Item = &ActionDefinition> {
        self.actions.values()
    }
}

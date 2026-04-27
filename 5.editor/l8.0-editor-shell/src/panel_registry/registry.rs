//! Panel Registry Service
//!
//! Manages available panels, their lifecycle, and instantiation.

use super::types::{PanelDefinition, PanelLifecycle};
use crate::panel_trait::{ActionContext, Panel, PanelId};
use std::collections::HashMap;

/// Panel Registry
///
/// Manages available panels, their lifecycle, and instantiation.
pub struct PanelRegistry {
    /// Registered panel definitions
    panels: HashMap<PanelId, PanelDefinition>,
    /// Current lifecycle state for each panel
    lifecycle: HashMap<PanelId, PanelLifecycle>,
    /// Active panel instances (Warm or Hot state)
    instances: HashMap<PanelId, Box<dyn Panel>>,
}

impl PanelRegistry {
    /// Creates a new empty panel registry
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
            lifecycle: HashMap::new(),
            instances: HashMap::new(),
        }
    }

    /// Registers a panel with the registry
    pub fn register_panel(&mut self, definition: PanelDefinition) {
        let panel_id = definition.panel_id.clone();
        self.lifecycle
            .insert(panel_id.clone(), PanelLifecycle::Cold);
        self.panels.insert(panel_id, definition);
    }

    /// Instantiates a panel using its factory function
    pub fn instantiate_panel(&mut self, panel_id: &PanelId) -> Result<(), String> {
        let definition = self
            .panels
            .get(panel_id)
            .ok_or_else(|| format!("Panel not found: {:?}", panel_id))?;

        let lifecycle = self
            .lifecycle
            .get(panel_id)
            .ok_or_else(|| format!("Panel lifecycle not tracked: {:?}", panel_id))?;

        if *lifecycle != PanelLifecycle::Cold {
            return Err(format!("Panel already instantiated: {:?}", panel_id));
        }

        let instance = (definition.factory)();
        self.lifecycle
            .insert(panel_id.clone(), PanelLifecycle::Warm);
        self.instances.insert(panel_id.clone(), instance);

        Ok(())
    }

    /// Gets the current lifecycle state of a panel
    pub fn get_lifecycle(&self, panel_id: &PanelId) -> PanelLifecycle {
        self.lifecycle
            .get(panel_id)
            .copied()
            .unwrap_or(PanelLifecycle::Cold)
    }

    /// Sets the lifecycle state of a panel
    pub fn set_lifecycle(&mut self, panel_id: &PanelId, lifecycle: PanelLifecycle) {
        self.lifecycle.insert(panel_id.clone(), lifecycle);
    }

    /// Queries all available panels
    pub fn query_available_panels(&self) -> Vec<PanelId> {
        self.panels.keys().cloned().collect()
    }

    /// Gets the panel definition for a given panel ID
    pub fn get_panel_definition(&self, panel_id: &PanelId) -> Option<&PanelDefinition> {
        self.panels.get(panel_id)
    }

    /// Gets a mutable reference to a panel instance
    pub fn get_panel_mut(&mut self, panel_id: &PanelId) -> Option<&mut Box<dyn Panel>> {
        self.instances.get_mut(panel_id)
    }

    /// Checks if a panel's dependencies are met
    pub fn check_dependencies(
        &self,
        panel_id: &PanelId,
        context: &ActionContext,
    ) -> Result<(), String> {
        let definition = self
            .panels
            .get(panel_id)
            .ok_or_else(|| format!("Panel not found: {:?}", panel_id))?;

        for dependency in &definition.dependencies {
            match dependency {
                super::types::PanelDependency::RequiresSelection => {
                    if !context.has_selection() {
                        return Err("Panel requires a selection".to_string());
                    }
                }
                super::types::PanelDependency::RequiresProject => {
                    if !context.has_project() {
                        return Err("Panel requires a project to be open".to_string());
                    }
                }
                super::types::PanelDependency::RequiresWorld => {
                    if !context.has_active_world() {
                        return Err("Panel requires a world to be open".to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Destroys a panel instance
    pub fn destroy_panel(&mut self, panel_id: &PanelId) {
        self.instances.remove(panel_id);
        self.lifecycle
            .insert(panel_id.clone(), PanelLifecycle::Cold);
    }
}

impl Default for PanelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

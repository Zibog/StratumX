//! Product Shell
//!
//! The Product Shell manages the top-level editor layout, panel lifecycle, and workspace
//! persistence. Owned by l8.0-editor-shell as the canonical shell orchestration.

use crate::panel_registry::{PanelLifecycle, PanelRegistry, WorkspaceState};
use crate::panel_trait::{ActionContext, PanelEvent, PanelId};
use std::sync::{Arc, Mutex};

/// Product Shell
///
/// The main product shell that manages panel lifecycle, layout, and workspace persistence.
pub struct ProductShell {
    /// Panel registry managing available panels
    panel_registry: PanelRegistry,
    /// Workspace state for layout persistence
    workspace_state: Arc<Mutex<WorkspaceState>>,
}

impl ProductShell {
    /// Creates a new product shell
    pub fn new(panel_registry: PanelRegistry, workspace_state: Arc<Mutex<WorkspaceState>>) -> Self {
        Self {
            panel_registry,
            workspace_state,
        }
    }

    /// Restores workspace layout from workspace state
    pub fn restore_workspace_layout(&mut self) -> Result<(), String> {
        let workspace_state = self
            .workspace_state
            .lock()
            .map_err(|e| format!("Failed to lock workspace state: {}", e))?;

        let open_panel_ids = workspace_state.get_open_panels();

        for panel_id in open_panel_ids {
            if self.panel_registry.get_panel_definition(panel_id).is_none() {
                eprintln!("Warning: Panel not registered: {:?}", panel_id);
                continue;
            }

            let lifecycle = self.panel_registry.get_lifecycle(panel_id);
            if lifecycle == PanelLifecycle::Cold {
                self.panel_registry
                    .instantiate_panel(panel_id)
                    .map_err(|e| format!("Failed to instantiate panel {:?}: {}", panel_id, e))?;
            }

            self.panel_registry
                .set_lifecycle(panel_id, PanelLifecycle::Hot);
        }

        Ok(())
    }

    /// Opens a panel
    pub fn open_panel(&mut self, panel_id: &PanelId) -> Result<(), String> {
        if self.panel_registry.get_panel_definition(panel_id).is_none() {
            return Err(format!("Panel not registered: {:?}", panel_id));
        }

        let lifecycle = self.panel_registry.get_lifecycle(panel_id);
        if lifecycle == PanelLifecycle::Cold {
            self.panel_registry.instantiate_panel(panel_id)?;
        }

        self.panel_registry
            .set_lifecycle(panel_id, PanelLifecycle::Hot);

        let mut workspace_state = self
            .workspace_state
            .lock()
            .map_err(|e| format!("Failed to lock workspace state: {}", e))?;
        workspace_state.add_open_panel(panel_id.clone());

        Ok(())
    }

    /// Closes a panel
    pub fn close_panel(&mut self, panel_id: &PanelId) -> Result<(), String> {
        self.panel_registry
            .set_lifecycle(panel_id, PanelLifecycle::Warm);

        let mut workspace_state = self
            .workspace_state
            .lock()
            .map_err(|e| format!("Failed to lock workspace state: {}", e))?;
        workspace_state.remove_open_panel(panel_id);

        Ok(())
    }

    /// Focuses a panel
    pub fn focus_panel(&mut self, panel_id: &PanelId) -> Result<(), String> {
        if self.panel_registry.get_panel_definition(panel_id).is_none() {
            return Err(format!("Panel not registered: {:?}", panel_id));
        }

        let lifecycle = self.panel_registry.get_lifecycle(panel_id);
        if lifecycle != PanelLifecycle::Hot {
            return Err(format!("Panel not open: {:?}", panel_id));
        }

        let mut workspace_state = self
            .workspace_state
            .lock()
            .map_err(|e| format!("Failed to lock workspace state: {}", e))?;
        workspace_state.set_focused_panel(Some(panel_id.clone()));

        Ok(())
    }

    /// Persists the current layout to workspace state
    pub fn persist_layout(&self) -> Result<(), String> {
        let workspace_state = self
            .workspace_state
            .lock()
            .map_err(|e| format!("Failed to lock workspace state: {}", e))?;

        let config_path = std::path::Path::new(".editor_workspace.json");
        workspace_state
            .serialize_to_file(config_path)
            .map_err(|e| format!("Failed to serialize workspace state: {}", e))
    }

    /// Gets a reference to the panel registry
    pub fn panel_registry(&self) -> &PanelRegistry {
        &self.panel_registry
    }

    /// Gets a mutable reference to the panel registry
    pub fn panel_registry_mut(&mut self) -> &mut PanelRegistry {
        &mut self.panel_registry
    }

    /// Gets a reference to the workspace state
    pub fn workspace_state(&self) -> Arc<Mutex<WorkspaceState>> {
        self.workspace_state.clone()
    }

    /// Renders all hot panels
    pub fn render_panels(&mut self, context: &ActionContext) {
        let hot_panels: Vec<PanelId> = self
            .panel_registry
            .query_available_panels()
            .into_iter()
            .filter(|panel_id| self.panel_registry.get_lifecycle(panel_id) == PanelLifecycle::Hot)
            .collect();

        for panel_id in hot_panels {
            if let Some(panel) = self.panel_registry.get_panel_mut(&panel_id) {
                panel.render(context);
            }
        }
    }

    /// Sends an event to all panels
    pub fn send_event_to_panels(&mut self, event: &PanelEvent) {
        let instantiated_panels: Vec<PanelId> = self
            .panel_registry
            .query_available_panels()
            .into_iter()
            .filter(|panel_id| {
                let lifecycle = self.panel_registry.get_lifecycle(panel_id);
                lifecycle == PanelLifecycle::Warm || lifecycle == PanelLifecycle::Hot
            })
            .collect();

        for panel_id in instantiated_panels {
            if let Some(panel) = self.panel_registry.get_panel_mut(&panel_id) {
                panel.on_event(event);
            }
        }
    }
}

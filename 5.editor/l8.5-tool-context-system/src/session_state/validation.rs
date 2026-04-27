//! Session State Validation

use crate::session_errors::ApplyError;
use crate::state_modification::{StateModification, ValidationError, ValidationErrorCode};

use super::types::SessionState;

impl SessionState {
    pub fn validate_modification(
        &self,
        modification: &StateModification,
    ) -> Result<(), ValidationError> {
        match modification {
            StateModification::SetActiveWorld(world_opt) => {
                if let Some(w) = world_opt {
                    if w.world_id.is_nil() {
                        return Err(ValidationError::with_field(
                            "World ID cannot be nil UUID",
                            "world_id",
                            ValidationErrorCode::InvalidWorldIdentity,
                        ));
                    }
                    if w.world_name.is_empty() {
                        return Err(ValidationError::with_field(
                            "World name cannot be empty",
                            "world_name",
                            ValidationErrorCode::InvalidWorldIdentity,
                        ));
                    }
                }
                Ok(())
            }
            StateModification::AddOpenPanel(p) | StateModification::RemoveOpenPanel(p) => {
                if p.0.is_empty() {
                    Err(ValidationError::with_field(
                        "Panel ID cannot be empty",
                        "panel_id",
                        ValidationErrorCode::InvalidPanelId,
                    ))
                } else {
                    Ok(())
                }
            }
            StateModification::SetFocusedPanel(p_opt) => {
                if let Some(p) = p_opt {
                    if p.0.is_empty() {
                        return Err(ValidationError::with_field(
                            "Panel ID cannot be empty",
                            "panel_id",
                            ValidationErrorCode::InvalidPanelId,
                        ));
                    }
                }
                Ok(())
            }
            StateModification::SetSelection(entities)
            | StateModification::AddToSelection(entities)
            | StateModification::RemoveFromSelection(entities) => {
                if entities.iter().any(|e| e.is_nil()) {
                    Err(ValidationError::with_field(
                        "Entity ID cannot be nil UUID",
                        "entity_id",
                        ValidationErrorCode::InvalidEntityId,
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    pub fn apply_modification(
        &mut self,
        modification: StateModification,
    ) -> Result<(), ApplyError> {
        match modification {
            StateModification::SetActiveWorld(w) => {
                self.set_active_world(w);
                Ok(())
            }
            StateModification::AddOpenPanel(p) => {
                self.add_open_panel(p);
                Ok(())
            }
            StateModification::RemoveOpenPanel(p) => {
                self.remove_open_panel(&p);
                Ok(())
            }
            StateModification::SetFocusedPanel(p) => {
                self.set_focused_panel(p);
                Ok(())
            }
            StateModification::SetSelection(entities) => {
                self.selection_state.set_selected(entities);
                Ok(())
            }
            StateModification::AddToSelection(entities) => {
                for e in entities {
                    self.selection_state.add_entity(e);
                }
                Ok(())
            }
            StateModification::RemoveFromSelection(entities) => {
                for e in &entities {
                    self.selection_state.remove_entity(e);
                }
                Ok(())
            }
            StateModification::ClearSelection => {
                self.selection_state.clear();
                Ok(())
            }
            StateModification::SetSelectionMode(m) => {
                self.selection_state.selection_mode = m;
                Ok(())
            }
            StateModification::SetToolMode(m) => {
                self.set_tool_mode(m);
                Ok(())
            }
            StateModification::IncrementSaveGeneration => Err(ApplyError {
                reason: "IncrementSaveGeneration should be applied to ProjectState".into(),
            }),
        }
    }
}

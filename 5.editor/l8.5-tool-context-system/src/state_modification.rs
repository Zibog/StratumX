//! State modification types for session state

use crate::selection_state::SelectionMode;
use crate::tool_mode::ToolMode;
use crate::{EntityId, PanelId, WorldIdentity};
use serde::{Deserialize, Serialize};

/// State modification commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateModification {
    SetActiveWorld(Option<WorldIdentity>),
    AddOpenPanel(PanelId),
    RemoveOpenPanel(PanelId),
    SetFocusedPanel(Option<PanelId>),
    SetSelection(Vec<EntityId>),
    AddToSelection(Vec<EntityId>),
    RemoveFromSelection(Vec<EntityId>),
    ClearSelection,
    SetSelectionMode(SelectionMode),
    SetToolMode(ToolMode),
    IncrementSaveGeneration,
}

/// Validation error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationErrorCode {
    InvalidWorldIdentity,
    InvalidPanelId,
    InvalidEntityId,
    InvalidToolMode,
    InvalidSelectionMode,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub message: String,
    pub field: Option<String>,
    pub code: ValidationErrorCode,
}

impl ValidationError {
    pub fn new(message: impl Into<String>, code: ValidationErrorCode) -> Self {
        Self {
            message: message.into(),
            field: None,
            code,
        }
    }

    pub fn with_field(
        message: impl Into<String>,
        field: impl Into<String>,
        code: ValidationErrorCode,
    ) -> Self {
        Self {
            message: message.into(),
            field: Some(field.into()),
            code,
        }
    }
}

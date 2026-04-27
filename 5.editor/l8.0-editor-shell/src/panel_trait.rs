//! Panel Trait Definition
//!
//! Defines the `Panel` trait that all editor panels must implement.
//! This trait provides the interface for panel lifecycle, rendering, and event handling.
//!
//! Owned by l8.0-editor-shell as the canonical shell panel behavior.

use crate::{Deserialize, Serialize};

/// Panel trait that all editor panels must implement
///
/// This trait defines the interface for panel lifecycle, rendering, and event handling.
/// All panels in the editor must implement this trait to be managed by the Panel Registry.
pub trait Panel: Send + Sync {
    /// Returns the unique identifier for this panel
    fn id(&self) -> PanelId;

    /// Returns the human-readable display name for this panel
    fn display_name(&self) -> &str;

    /// Renders the panel content
    ///
    /// This method is called when the panel is in the Hot lifecycle state.
    fn render(&mut self, context: &ActionContext);

    /// Handles events sent to this panel
    fn on_event(&mut self, event: &PanelEvent);
}

/// Unique identifier for a panel
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelId(pub String);

/// Events that can be sent to panels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelEvent {
    /// Selection state changed
    SelectionChanged,
    /// World was opened
    WorldOpened,
    /// World was closed
    WorldClosed,
    /// World was saved
    WorldSaved,
    /// Generic state mutation occurred
    StateMutated,
    /// Panel focus changed
    FocusChanged(Option<PanelId>),
    /// Panel was opened
    PanelOpened(PanelId),
    /// Panel was closed
    PanelClosed(PanelId),
    /// Material state changed
    MaterialChanged,
}

/// Panel factory function type
pub type PanelFactory = Box<dyn Fn() -> Box<dyn Panel> + Send + Sync>;

/// Action context stub — actual type comes from command spine
#[derive(Debug, Clone)]
pub struct ActionContext {
    has_selection: bool,
    has_project: bool,
    has_active_world: bool,
}

impl ActionContext {
    pub fn new() -> Self {
        Self {
            has_selection: false,
            has_project: false,
            has_active_world: false,
        }
    }

    pub fn with_selection(mut self) -> Self {
        self.has_selection = true;
        self
    }

    pub fn with_project(mut self) -> Self {
        self.has_project = true;
        self
    }

    pub fn with_active_world(mut self) -> Self {
        self.has_active_world = true;
        self
    }

    pub fn has_selection(&self) -> bool {
        self.has_selection
    }

    pub fn has_project(&self) -> bool {
        self.has_project
    }

    pub fn has_active_world(&self) -> bool {
        self.has_active_world
    }
}

impl Default for ActionContext {
    fn default() -> Self {
        Self::new()
    }
}

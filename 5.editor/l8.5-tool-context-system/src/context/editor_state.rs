//! Editor State
//!
//! Defines the editor's operational state.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorState {
    Initializing,
    WorldOpen,
    Authoring,
    PlayMode,
    SimulateMode,
    Degraded,
}

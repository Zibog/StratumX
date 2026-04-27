//! Session State Container
//!
//! This module provides the session state management for the editor,
//! including active world, open panels, selection state, and tool mode.

mod mutations;
mod persistence;
mod queries;
mod types;
mod validation;

pub use types::SessionState;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection_state::SelectionState;
    use crate::PanelId;
    use std::path::PathBuf;
    use uuid::Uuid;

    #[test]
    fn test_session_state_creation() {
        let s = SessionState::new();
        assert!(s.active_world.is_none());
        assert!(s.open_panels.is_empty());
    }

    #[test]
    fn test_panel_management() {
        let mut s = SessionState::new();
        let p = PanelId("viewport".into());
        s.add_open_panel(p.clone());
        assert_eq!(s.open_panels.len(), 1);
        s.add_open_panel(p.clone());
        assert_eq!(s.open_panels.len(), 1);
        s.remove_open_panel(&p);
        assert!(s.open_panels.is_empty());
    }

    #[test]
    fn test_selection_state() {
        let mut sel = SelectionState::new();
        assert!(!sel.has_selection());
        let e1 = Uuid::new_v4();
        sel.add_entity(e1);
        assert!(sel.has_selection());
        sel.clear();
        assert!(!sel.has_selection());
    }

    #[test]
    fn test_recently_opened_worlds() {
        let mut s = SessionState::new();
        for i in 0..15 {
            s.add_recently_opened_world(PathBuf::from(format!("/w{}", i)));
        }
        assert_eq!(s.recently_opened_worlds.len(), 10);
    }
}

use crate::{SelectionState, WorldIdentity};

use super::CachedStateQueries;

impl<'a> CachedStateQueries<'a> {
    pub(crate) fn active_world_ref(&self) -> Option<&WorldIdentity> {
        self.session_state.active_world.as_ref()
    }

    pub(crate) fn selection_state_ref(&self) -> &SelectionState {
        &self.session_state.selection_state
    }

    pub(crate) fn has_active_world_flag(&self) -> bool {
        self.session_state.active_world.is_some()
    }

    pub(crate) fn has_selection_flag(&self) -> bool {
        self.session_state.selection_state.has_selection()
    }
}

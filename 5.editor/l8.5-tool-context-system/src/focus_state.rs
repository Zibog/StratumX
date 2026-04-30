//! Focus state type.

use crate::{EntityId, PanelId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusState {
    pub focused_panel: Option<PanelId>,
    pub focused_entity: Option<EntityId>,
}
impl FocusState {
    pub fn new() -> Self {
        Self {
            focused_panel: None,
            focused_entity: None,
        }
    }
}
impl Default for FocusState {
    fn default() -> Self {
        Self::new()
    }
}

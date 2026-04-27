//! Entity selection management

use crate::entity_state::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Selection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionState {
    selected: HashSet<EntityId>,
    active: Option<EntityId>,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selected: HashSet::new(),
            active: None,
        }
    }

    pub fn select(&mut self, entity_id: EntityId) {
        self.selected.insert(entity_id);
        self.active = Some(entity_id);
    }

    pub fn deselect(&mut self, entity_id: EntityId) {
        self.selected.remove(&entity_id);
        if self.active == Some(entity_id) {
            self.active = self.selected.iter().next().copied();
        }
    }

    pub fn clear(&mut self) {
        self.selected.clear();
        self.active = None;
    }

    pub fn is_selected(&self, entity_id: EntityId) -> bool {
        self.selected.contains(&entity_id)
    }

    pub fn selected_entities(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.selected.iter().copied()
    }

    pub fn active_entity(&self) -> Option<EntityId> {
        self.active
    }

    pub fn selection_count(&self) -> usize {
        self.selected.len()
    }
}

impl Default for SelectionState {
    fn default() -> Self {
        Self::new()
    }
}

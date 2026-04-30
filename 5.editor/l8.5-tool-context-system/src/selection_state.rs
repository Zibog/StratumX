//! Selection state types.

use crate::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionMode {
    Single,
    Multiple,
    Hierarchical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionState {
    pub selected_entities: Vec<EntityId>,
    pub selection_mode: SelectionMode,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selected_entities: Vec::new(),
            selection_mode: SelectionMode::Single,
        }
    }
    pub fn get_selected_entities(&self) -> &[EntityId] {
        &self.selected_entities
    }
    pub fn get_selection_mode(&self) -> SelectionMode {
        self.selection_mode
    }
    pub fn has_selection(&self) -> bool {
        !self.selected_entities.is_empty()
    }
    pub fn selection_count(&self) -> usize {
        self.selected_entities.len()
    }
    pub fn is_selected(&self, entity: &EntityId) -> bool {
        self.selected_entities.contains(entity)
    }
    pub(crate) fn clear(&mut self) {
        self.selected_entities.clear()
    }
    pub(crate) fn set_selected(&mut self, entities: Vec<EntityId>) {
        self.selected_entities = entities
    }
    pub(crate) fn add_entity(&mut self, entity: EntityId) {
        if !self.selected_entities.contains(&entity) {
            self.selected_entities.push(entity);
        }
    }
    pub(crate) fn remove_entity(&mut self, entity: &EntityId) {
        self.selected_entities.retain(|e| e != entity);
    }
}
impl Default for SelectionState {
    fn default() -> Self {
        Self::new()
    }
}

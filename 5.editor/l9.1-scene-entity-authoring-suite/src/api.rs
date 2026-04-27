//! Public API for scene entity authoring

use crate::entity_state::*;
use crate::hierarchy::*;
use crate::transform::*;
use crate::component_editor::*;
use crate::selection::*;

/// Scene authoring service
pub struct SceneAuthoringService {
    entity_registry: EntityRegistry,
    component_registry: ComponentRegistry,
    selection: SelectionState,
}

impl SceneAuthoringService {
    pub fn new() -> Self {
        Self {
            entity_registry: EntityRegistry::new(),
            component_registry: ComponentRegistry::new(),
            selection: SelectionState::new(),
        }
    }

    pub fn create_entity(&mut self, name: String) -> EntityId {
        let entity = Entity {
            id: EntityId::new(),
            name,
            enabled: true,
            parent: None,
            children: Vec::new(),
            components: Vec::new(),
        };
        let id = entity.id;
        self.entity_registry.add_entity(entity);
        id
    }

    pub fn delete_entity(&mut self, entity_id: EntityId) {
        self.entity_registry.remove_entity(entity_id);
        self.selection.deselect(entity_id);
    }

    pub fn set_parent(&mut self, child_id: EntityId, parent_id: Option<EntityId>) -> Result<(), HierarchyError> {
        HierarchyManager::set_parent(&mut self.entity_registry, child_id, parent_id)
    }

    pub fn add_component(&mut self, entity_id: EntityId, component_type: ComponentType) -> ComponentId {
        let component = ComponentData {
            id: ComponentId::new(),
            component_type,
            enabled: true,
            properties: Default::default(),
        };
        let id = component.id;
        self.component_registry.add_component(entity_id, component);
        id
    }

    pub fn select_entity(&mut self, entity_id: EntityId) {
        self.selection.select(entity_id);
    }

    pub fn entity_registry(&self) -> &EntityRegistry {
        &self.entity_registry
    }

    pub fn component_registry(&self) -> &ComponentRegistry {
        &self.component_registry
    }

    pub fn selection(&self) -> &SelectionState {
        &self.selection
    }
}

impl Default for SceneAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}

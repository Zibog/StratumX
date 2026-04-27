//! Entity state management for scene authoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Entity ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Entity metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub enabled: bool,
    pub parent: Option<EntityId>,
    pub children: Vec<EntityId>,
    pub components: Vec<ComponentId>,
}

/// Component ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentId(pub Uuid);

impl ComponentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Entity registry state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRegistry {
    entities: HashMap<EntityId, Entity>,
    root_entities: Vec<EntityId>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            root_entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let id = entity.id;
        let is_root = entity.parent.is_none();
        self.entities.insert(id, entity);
        if is_root {
            self.root_entities.push(id);
        }
    }

    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        if let Some(entity) = self.entities.remove(&id) {
            self.root_entities.retain(|&e| e != id);
            Some(entity)
        } else {
            None
        }
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn root_entities(&self) -> &[EntityId] {
        &self.root_entities
    }

    pub fn all_entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }
}

impl Default for EntityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

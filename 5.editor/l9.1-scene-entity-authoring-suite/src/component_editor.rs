//! Component editing functionality

use crate::entity_state::{ComponentId, EntityId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Component type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    Transform,
    MeshRenderer,
    Light,
    Camera,
    Collider,
    RigidBody,
    Script,
    AudioSource,
}

/// Component data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub id: ComponentId,
    pub component_type: ComponentType,
    pub enabled: bool,
    pub properties: HashMap<String, PropertyValue>,
}

/// Property value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Vec3([f32; 3]),
    Color([f32; 4]),
}

/// Component registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentRegistry {
    components: HashMap<ComponentId, ComponentData>,
    entity_components: HashMap<EntityId, Vec<ComponentId>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            entity_components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, entity_id: EntityId, component: ComponentData) {
        let component_id = component.id;
        self.components.insert(component_id, component);
        self.entity_components
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(component_id);
    }

    pub fn remove_component(&mut self, component_id: ComponentId) -> Option<ComponentData> {
        if let Some(component) = self.components.remove(&component_id) {
            for components in self.entity_components.values_mut() {
                components.retain(|&id| id != component_id);
            }
            Some(component)
        } else {
            None
        }
    }

    pub fn get_component(&self, component_id: ComponentId) -> Option<&ComponentData> {
        self.components.get(&component_id)
    }

    pub fn get_component_mut(&mut self, component_id: ComponentId) -> Option<&mut ComponentData> {
        self.components.get_mut(&component_id)
    }

    pub fn get_entity_components(&self, entity_id: EntityId) -> Vec<&ComponentData> {
        self.entity_components
            .get(&entity_id)
            .map(|ids| ids.iter().filter_map(|id| self.components.get(id)).collect())
            .unwrap_or_default()
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

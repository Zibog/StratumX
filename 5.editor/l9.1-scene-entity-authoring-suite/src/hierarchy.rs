//! Entity hierarchy management

use crate::entity_state::{Entity, EntityId, EntityRegistry};

/// Hierarchy operations
pub struct HierarchyManager;

impl HierarchyManager {
    /// Set parent for an entity
    pub fn set_parent(
        registry: &mut EntityRegistry,
        child_id: EntityId,
        parent_id: Option<EntityId>,
    ) -> Result<(), HierarchyError> {
        // Validate no circular dependency
        if let Some(pid) = parent_id {
            if Self::is_ancestor(registry, child_id, pid) {
                return Err(HierarchyError::CircularDependency);
            }
        }

        // Remove from old parent
        if let Some(child) = registry.get_entity(child_id) {
            if let Some(old_parent_id) = child.parent {
                if let Some(old_parent) = registry.get_entity_mut(old_parent_id) {
                    old_parent.children.retain(|&id| id != child_id);
                }
            }
        }

        // Update child's parent
        if let Some(child) = registry.get_entity_mut(child_id) {
            child.parent = parent_id;
        }

        // Add to new parent
        if let Some(pid) = parent_id {
            if let Some(parent) = registry.get_entity_mut(pid) {
                if !parent.children.contains(&child_id) {
                    parent.children.push(child_id);
                }
            }
        }

        Ok(())
    }

    /// Check if entity is ancestor of another
    pub fn is_ancestor(registry: &EntityRegistry, ancestor_id: EntityId, entity_id: EntityId) -> bool {
        let mut current = entity_id;
        while let Some(entity) = registry.get_entity(current) {
            if let Some(parent_id) = entity.parent {
                if parent_id == ancestor_id {
                    return true;
                }
                current = parent_id;
            } else {
                break;
            }
        }
        false
    }

    /// Get all descendants of an entity
    pub fn get_descendants(registry: &EntityRegistry, entity_id: EntityId) -> Vec<EntityId> {
        let mut descendants = Vec::new();
        if let Some(entity) = registry.get_entity(entity_id) {
            for &child_id in &entity.children {
                descendants.push(child_id);
                descendants.extend(Self::get_descendants(registry, child_id));
            }
        }
        descendants
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyError {
    CircularDependency,
    EntityNotFound,
}

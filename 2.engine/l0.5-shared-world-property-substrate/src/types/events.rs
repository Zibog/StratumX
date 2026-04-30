use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    CellField, ConflictRule, ObjectLocalField, PropertyType, SurfaceField, UpdateOrderGraph,
    VolumeField,
};

/// World property substrate - unified field storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPropertySubstrate {
    /// Cell-based fields.
    pub cell_fields: HashMap<PropertyType, CellField>,
    /// Object-local fields.
    pub object_local_fields: HashMap<u64, Vec<ObjectLocalField>>,
    /// Surface fields.
    pub surface_fields: HashMap<u64, Vec<SurfaceField>>,
    /// Volume fields.
    pub volume_fields: HashMap<u64, VolumeField>,
    /// Update order graph.
    pub update_order: UpdateOrderGraph,
    /// Conflict resolution rules.
    pub conflict_rules: Vec<ConflictRule>,
}

impl WorldPropertySubstrate {
    /// Create a new empty substrate.
    pub fn new() -> Self {
        Self {
            cell_fields: HashMap::new(),
            object_local_fields: HashMap::new(),
            surface_fields: HashMap::new(),
            volume_fields: HashMap::new(),
            update_order: UpdateOrderGraph::new(),
            conflict_rules: Vec::new(),
        }
    }

    /// Create default substrate with standard update order.
    pub fn with_default_update_order() -> Self {
        let mut substrate = Self::new();

        substrate
            .update_order
            .add_edge(PropertyType::Heat, PropertyType::Wetness);
        substrate.update_order.add_edge(
            PropertyType::SmokeDensity,
            PropertyType::VisibilityObscuration,
        );
        substrate
            .update_order
            .add_edge(PropertyType::WindHint, PropertyType::SmokeDensity);
        substrate.update_order.add_edge(
            PropertyType::ToxicContamination,
            PropertyType::AnomalyIntensity,
        );

        substrate
    }
}

impl Default for WorldPropertySubstrate {
    fn default() -> Self {
        Self::new()
    }
}

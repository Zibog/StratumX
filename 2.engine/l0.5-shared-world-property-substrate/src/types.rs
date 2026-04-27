/// World property types for unified field substrate
///
/// This module defines the core types for the shared world property substrate,
/// including field properties, storage types, and update ordering.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Property types supported by the world field substrate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// Water saturation level (0.0 = dry, 1.0 = fully saturated)
    Wetness,
    /// Temperature in Kelvin
    Heat,
    /// Smoke particle density (0.0 = clear, 1.0 = opaque)
    SmokeDensity,
    /// Toxic contamination level (0.0 = safe, 1.0 = lethal)
    ToxicContamination,
    /// Wind direction and strength hint for simulation
    WindHint,
    /// Visibility reduction factor (0.0 = clear, 1.0 = zero visibility)
    VisibilityObscuration,
    /// Sound pressure level hint for audio propagation
    SoundPressureHint,
    /// Anomaly intensity for special effects (0.0 = normal, 1.0 = maximum anomaly)
    AnomalyIntensity,
}

/// Storage location for field data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldStorage {
    /// Cell-based field (voxel grid)
    CellField,
    /// Object-local field (attached to entities)
    ObjectLocalField,
    /// Surface field (attached to surfaces/meshes)
    SurfaceField,
    /// Volume field (3D regions)
    VolumeField,
}

/// Field value with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue {
    /// The property type
    pub property: PropertyType,
    /// The numeric value
    pub value: f32,
    /// Storage location
    pub storage: FieldStorage,
    /// Last update timestamp (simulation tick)
    pub last_update: u64,
}

/// Cell field data structure (voxel-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellField {
    /// Property type
    pub property: PropertyType,
    /// 3D grid dimensions (x, y, z)
    pub dimensions: (usize, usize, usize),
    /// Flattened voxel data (row-major order)
    pub data: Vec<f32>,
}

/// Object-local field data (per-entity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectLocalField {
    /// Property type
    pub property: PropertyType,
    /// Entity ID
    pub entity_id: u64,
    /// Field value
    pub value: f32,
}

/// Surface field data (per-surface)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceField {
    /// Property type
    pub property: PropertyType,
    /// Surface ID
    pub surface_id: u64,
    /// Per-vertex or per-face values
    pub values: Vec<f32>,
}

/// Volume field data (3D region)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeField {
    /// Property type
    pub property: PropertyType,
    /// Volume ID
    pub volume_id: u64,
    /// Bounding box (min_x, min_y, min_z, max_x, max_y, max_z)
    pub bounds: (f32, f32, f32, f32, f32, f32),
    /// Uniform value or interpolated field
    pub value: f32,
}

/// Update order graph for conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderGraph {
    /// Nodes (property types)
    pub nodes: Vec<PropertyType>,
    /// Edges (from, to) indicating update dependencies
    pub edges: Vec<(PropertyType, PropertyType)>,
}

impl UpdateOrderGraph {
    /// Create a new empty update order graph
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Add a property node
    pub fn add_node(&mut self, property: PropertyType) {
        if !self.nodes.contains(&property) {
            self.nodes.push(property);
        }
    }

    /// Add an update dependency edge (from must update before to)
    pub fn add_edge(&mut self, from: PropertyType, to: PropertyType) {
        self.add_node(from);
        self.add_node(to);
        if !self.edges.contains(&(from, to)) {
            self.edges.push((from, to));
        }
    }

    /// Get topological sort order for updates
    pub fn topological_sort(&self) -> Result<Vec<PropertyType>, String> {
        let mut in_degree: HashMap<PropertyType, usize> = HashMap::new();
        let mut adj_list: HashMap<PropertyType, Vec<PropertyType>> = HashMap::new();

        // Initialize
        for &node in &self.nodes {
            in_degree.insert(node, 0);
            adj_list.insert(node, Vec::new());
        }

        // Build adjacency list and in-degrees
        for &(from, to) in &self.edges {
            adj_list.get_mut(&from).unwrap().push(to);
            *in_degree.get_mut(&to).unwrap() += 1;
        }

        // Kahn's algorithm
        let mut queue: Vec<PropertyType> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&node, _)| node)
            .collect();

        let mut result = Vec::new();

        while let Some(node) = queue.pop() {
            result.push(node);

            if let Some(neighbors) = adj_list.get(&node) {
                for &neighbor in neighbors {
                    let deg = in_degree.get_mut(&neighbor).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err("Cycle detected in update order graph".to_string());
        }

        Ok(result)
    }
}

impl Default for UpdateOrderGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Use priority order (first property wins)
    Priority,
    /// Blend values with weights
    Blend,
    /// Take maximum value
    Maximum,
    /// Take minimum value
    Minimum,
}

/// Conflict resolution rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRule {
    /// Properties involved in conflict
    pub properties: Vec<PropertyType>,
    /// Resolution strategy
    pub resolution: ConflictResolution,
    /// Weights for blending (if using Blend strategy)
    pub weights: HashMap<PropertyType, f32>,
}

/// World property substrate - unified field storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPropertySubstrate {
    /// Cell-based fields
    pub cell_fields: HashMap<PropertyType, CellField>,
    /// Object-local fields
    pub object_local_fields: HashMap<u64, Vec<ObjectLocalField>>,
    /// Surface fields
    pub surface_fields: HashMap<u64, Vec<SurfaceField>>,
    /// Volume fields
    pub volume_fields: HashMap<u64, VolumeField>,
    /// Update order graph
    pub update_order: UpdateOrderGraph,
    /// Conflict resolution rules
    pub conflict_rules: Vec<ConflictRule>,
}

impl WorldPropertySubstrate {
    /// Create a new empty substrate
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

    /// Create default substrate with standard update order
    pub fn with_default_update_order() -> Self {
        let mut substrate = Self::new();

        // Define standard update order
        // Heat affects wetness (evaporation)
        substrate
            .update_order
            .add_edge(PropertyType::Heat, PropertyType::Wetness);
        // Smoke affects visibility
        substrate.update_order.add_edge(
            PropertyType::SmokeDensity,
            PropertyType::VisibilityObscuration,
        );
        // Wind affects smoke
        substrate
            .update_order
            .add_edge(PropertyType::WindHint, PropertyType::SmokeDensity);
        // Toxic contamination affects anomaly intensity
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_order_graph_topological_sort() {
        let mut graph = UpdateOrderGraph::new();
        graph.add_edge(PropertyType::Heat, PropertyType::Wetness);
        graph.add_edge(PropertyType::WindHint, PropertyType::SmokeDensity);
        graph.add_edge(
            PropertyType::SmokeDensity,
            PropertyType::VisibilityObscuration,
        );

        let order = graph.topological_sort().unwrap();

        // Heat should come before Wetness
        let heat_idx = order.iter().position(|&p| p == PropertyType::Heat).unwrap();
        let wetness_idx = order
            .iter()
            .position(|&p| p == PropertyType::Wetness)
            .unwrap();
        assert!(heat_idx < wetness_idx);

        // WindHint -> SmokeDensity -> VisibilityObscuration
        let wind_idx = order
            .iter()
            .position(|&p| p == PropertyType::WindHint)
            .unwrap();
        let smoke_idx = order
            .iter()
            .position(|&p| p == PropertyType::SmokeDensity)
            .unwrap();
        let vis_idx = order
            .iter()
            .position(|&p| p == PropertyType::VisibilityObscuration)
            .unwrap();
        assert!(wind_idx < smoke_idx);
        assert!(smoke_idx < vis_idx);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = UpdateOrderGraph::new();
        graph.add_edge(PropertyType::Heat, PropertyType::Wetness);
        graph.add_edge(PropertyType::Wetness, PropertyType::Heat); // Cycle

        let result = graph.topological_sort();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cycle detected"));
    }

    #[test]
    fn test_world_property_substrate_creation() {
        let substrate = WorldPropertySubstrate::new();
        assert!(substrate.cell_fields.is_empty());
        assert!(substrate.object_local_fields.is_empty());
        assert!(substrate.surface_fields.is_empty());
        assert!(substrate.volume_fields.is_empty());
    }

    #[test]
    fn test_default_update_order() {
        let substrate = WorldPropertySubstrate::with_default_update_order();
        let order = substrate.update_order.topological_sort().unwrap();

        // Verify some expected orderings
        let heat_idx = order.iter().position(|&p| p == PropertyType::Heat);
        let wetness_idx = order.iter().position(|&p| p == PropertyType::Wetness);

        if let (Some(h), Some(w)) = (heat_idx, wetness_idx) {
            assert!(h < w, "Heat should update before Wetness");
        }
    }
}

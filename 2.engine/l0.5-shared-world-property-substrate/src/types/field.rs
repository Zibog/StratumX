use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Property types supported by the world field substrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// Water saturation level (0.0 = dry, 1.0 = fully saturated).
    Wetness,
    /// Temperature in Kelvin.
    Heat,
    /// Smoke particle density (0.0 = clear, 1.0 = opaque).
    SmokeDensity,
    /// Toxic contamination level (0.0 = safe, 1.0 = lethal).
    ToxicContamination,
    /// Wind direction and strength hint for simulation.
    WindHint,
    /// Visibility reduction factor (0.0 = clear, 1.0 = zero visibility).
    VisibilityObscuration,
    /// Sound pressure level hint for audio propagation.
    SoundPressureHint,
    /// Anomaly intensity for special effects (0.0 = normal, 1.0 = maximum anomaly).
    AnomalyIntensity,
}

/// Storage location for field data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldStorage {
    /// Cell-based field (voxel grid).
    CellField,
    /// Object-local field (attached to entities).
    ObjectLocalField,
    /// Surface field (attached to surfaces/meshes).
    SurfaceField,
    /// Volume field (3D regions).
    VolumeField,
}

/// Field value with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue {
    /// The property type.
    pub property: PropertyType,
    /// The numeric value.
    pub value: f32,
    /// Storage location.
    pub storage: FieldStorage,
    /// Last update timestamp (simulation tick).
    pub last_update: u64,
}

/// Cell field data structure (voxel-based).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellField {
    /// Property type.
    pub property: PropertyType,
    /// 3D grid dimensions (x, y, z).
    pub dimensions: (usize, usize, usize),
    /// Flattened voxel data (row-major order).
    pub data: Vec<f32>,
}

/// Update order graph for conflict resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderGraph {
    /// Nodes (property types).
    pub nodes: Vec<PropertyType>,
    /// Edges (from, to) indicating update dependencies.
    pub edges: Vec<(PropertyType, PropertyType)>,
}

impl UpdateOrderGraph {
    /// Create a new empty update order graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Add a property node.
    pub fn add_node(&mut self, property: PropertyType) {
        if !self.nodes.contains(&property) {
            self.nodes.push(property);
        }
    }

    /// Add an update dependency edge (from must update before to).
    pub fn add_edge(&mut self, from: PropertyType, to: PropertyType) {
        self.add_node(from);
        self.add_node(to);
        if !self.edges.contains(&(from, to)) {
            self.edges.push((from, to));
        }
    }

    /// Get topological sort order for updates.
    pub fn topological_sort(&self) -> Result<Vec<PropertyType>, String> {
        let mut in_degree: BTreeMap<PropertyType, usize> = BTreeMap::new();
        let mut adj_list: BTreeMap<PropertyType, BTreeSet<PropertyType>> = BTreeMap::new();

        for &node in &self.nodes {
            in_degree.insert(node, 0);
            adj_list.insert(node, BTreeSet::new());
        }

        for &(from, to) in &self.edges {
            if adj_list.get_mut(&from).unwrap().insert(to) {
                *in_degree.get_mut(&to).unwrap() += 1;
            }
        }

        let mut queue: BTreeSet<PropertyType> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&node, _)| node)
            .collect();
        let mut result = Vec::new();

        while let Some(node) = queue.pop_first() {
            result.push(node);

            if let Some(neighbors) = adj_list.get(&node) {
                for &neighbor in neighbors {
                    let deg = in_degree.get_mut(&neighbor).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.insert(neighbor);
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

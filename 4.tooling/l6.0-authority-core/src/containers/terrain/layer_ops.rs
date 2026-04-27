// Terrain Authority Container - Layer operations

use super::types::{TerrainAuthorityContainer, TerrainLayer};

impl TerrainAuthorityContainer {
    /// Query terrain layers (read-only)
    pub fn query_layers(&self) -> &[TerrainLayer] {
        &self.registry.layers
    }

    /// Add a terrain layer
    pub fn add_layer(&mut self, name: String, material_id: Option<String>) -> TerrainLayer {
        let id = self.registry.layers.len() as u32;
        let layer = TerrainLayer {
            id,
            name,
            material_id,
            visible: true,
        };
        self.registry.layers.push(layer.clone());
        layer
    }

    /// Remove a terrain layer
    pub fn remove_layer(&mut self, id: u32) -> bool {
        let len = self.registry.layers.len();
        self.registry.layers.retain(|l| l.id != id);
        self.registry.layers.len() != len
    }
}

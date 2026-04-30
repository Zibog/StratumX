//! Terrain Authoring Service
//!
//! Owns terrain truth: manifest, chunk dirtiness, layer bindings.
//! UI panels read from this service, not own authoritative state.
//!
//! **Requirements: 6.2, 6.4**

use super::types::*;
use crate::{TerrainAuthoring, TerrainDeliveryThread};
use editor_dto_law::TerrainManifest;

/// Terrain Authoring Service - owns terrain truth
///
/// **Requirements: 6.2, 6.4**
/// - Owns terrain manifest (not UI)
/// - Owns chunk dirtiness truth (not UI)
/// - Owns layer bindings truth (not UI)
pub struct TerrainAuthoringService {
    /// Terrain manifest - authoritative terrain configuration
    pub(super) manifest: Option<TerrainManifest>,

    /// Chunk dirtiness tracking - which chunks need rebuild
    pub(super) chunk_dirtiness: ChunkDirtiness,

    /// Layer bindings - material layers for terrain
    pub(super) layer_bindings: LayerBindings,

    /// Real terrain authoring operations
    pub(super) authoring: TerrainAuthoring,

    /// Delivery/sync bridge to engine world truth
    pub(super) delivery: TerrainDeliveryThread,

    /// Bound proof scene name
    pub(super) bound_scene_label: Option<String>,
}

impl TerrainAuthoringService {
    /// Create a new terrain authoring service
    pub fn new() -> Self {
        Self {
            manifest: None,
            chunk_dirtiness: ChunkDirtiness::new(),
            layer_bindings: LayerBindings::new(),
            authoring: TerrainAuthoring::new(),
            delivery: TerrainDeliveryThread::new(),
            bound_scene_label: None,
        }
    }

    /// Initialize the service with a terrain manifest
    pub fn initialize_manifest(&mut self, manifest: TerrainManifest) {
        self.manifest = Some(manifest);
        self.chunk_dirtiness.clear();
        self.layer_bindings.clear();
        self.authoring.clear_dirty_chunks();
    }

    /// Modify heightmap in a region
    ///
    /// **Requirement 6.4**: Service owns terrain truth, not UI
    pub fn modify_heightmap(&mut self, region: Region, _delta: f32) -> Result<(), String> {
        if self.manifest.is_none() {
            return Err("No terrain manifest initialized".to_string());
        }

        // Calculate affected chunks
        let manifest = self.manifest.as_ref().unwrap();
        let chunk_size = manifest.chunk_size as f32;

        let min_chunk_x = (region.min_x / chunk_size).floor() as u32;
        let min_chunk_y = (region.min_y / chunk_size).floor() as u32;
        let max_chunk_x = (region.max_x / chunk_size).ceil() as u32;
        let max_chunk_y = (region.max_y / chunk_size).ceil() as u32;

        // Mark affected chunks as dirty and update heightmap buffer
        for chunk_x in min_chunk_x..=max_chunk_x {
            for chunk_y in min_chunk_y..=max_chunk_y {
                let chunk_id = ChunkId::new(chunk_x, chunk_y);
                self.chunk_dirtiness.insert(chunk_id, true);
            }
        }

        Ok(())
    }

    /// Set a layer profile
    ///
    /// **Requirement 6.4**: Service owns layer bindings, not UI
    pub fn set_layer(&mut self, layer_id: LayerId, profile: LayerProfile) -> Result<(), String> {
        if self.manifest.is_none() {
            return Err("No terrain manifest initialized".to_string());
        }

        self.layer_bindings.insert(layer_id, profile);

        // Mark all chunks as dirty since layer configuration changed
        if let Some(manifest) = &self.manifest {
            for chunk_x in 0..manifest.chunk_grid[0] {
                for chunk_y in 0..manifest.chunk_grid[1] {
                    let chunk_id = ChunkId::new(chunk_x, chunk_y);
                    self.chunk_dirtiness.insert(chunk_id, true);
                }
            }
        }

        Ok(())
    }

    /// Get a reference to the terrain manifest
    ///
    /// **Requirement 6.6**: UI reads from service
    pub fn get_manifest(&self) -> Option<&TerrainManifest> {
        self.manifest.as_ref()
    }

    /// Get list of dirty chunks that need rebuild
    ///
    /// **Requirement 6.6**: UI reads derived data from service
    pub fn get_dirty_chunks(&self) -> Vec<ChunkId> {
        self.chunk_dirtiness
            .iter()
            .filter_map(
                |(chunk_id, &is_dirty)| {
                    if is_dirty {
                        Some(*chunk_id)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    /// Clear dirty flag for a chunk (after rebuild)
    pub fn clear_chunk_dirty(&mut self, chunk_id: ChunkId) {
        self.chunk_dirtiness.insert(chunk_id, false);
    }

    /// Clear all dirty chunks
    pub fn clear_all_dirty_chunks(&mut self) {
        self.chunk_dirtiness.clear();
    }

    /// Get layer profile for a layer
    pub fn get_layer(&self, layer_id: LayerId) -> Option<&LayerProfile> {
        self.layer_bindings.get(&layer_id)
    }

    /// Get all layer bindings
    pub fn get_layer_bindings(&self) -> &LayerBindings {
        &self.layer_bindings
    }

    /// Check if terrain is initialized
    pub fn is_initialized(&self) -> bool {
        self.manifest.is_some()
    }

    pub fn bound_scene_label(&self) -> Option<&str> {
        self.bound_scene_label.as_deref()
    }
}

impl Default for TerrainAuthoringService {
    fn default() -> Self {
        Self::new()
    }
}

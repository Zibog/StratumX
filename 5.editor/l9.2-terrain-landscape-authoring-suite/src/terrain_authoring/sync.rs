//! Terrain Sync and Rebuild Operations

use super::types::{ChunkId, LayerId, TerrainSyncReport};
use super::TerrainAuthoringService;
use editor_dto_law::{ChunkDescriptor, MaterialLayer, StableWorldId};
use engine_world::WorldState;
use uuid::Uuid;

impl TerrainAuthoringService {
    pub fn bind_terrain(
        &mut self,
        world_ref: StableWorldId,
        scene_label: impl Into<String>,
    ) -> Result<(), String> {
        self.delivery.terrain_root(world_ref)?;
        self.bound_scene_label = Some(scene_label.into());
        Ok(())
    }

    pub fn rebuild_terrain(&mut self, world: &mut WorldState) -> Result<(), String> {
        self.delivery.terrain_rebuild()?;
        self.delivery.rebuild_dirty_chunks(world)?;
        self.update_manifest_from_world(world)?;
        self.capture_world_dirtiness(world);
        Ok(())
    }

    pub fn sync_terrain(&mut self, world: &mut WorldState) -> Result<TerrainSyncReport, String> {
        self.delivery.sync_to_world(world)?;
        self.delivery.sync_from_world(world)?;
        self.update_manifest_from_world(world)?;
        self.capture_world_dirtiness(world);

        Ok(TerrainSyncReport {
            bound: self.delivery.is_bound(),
            dirty_chunk_count: self.get_dirty_chunks().len(),
            gpu_sync_required: self.delivery.needs_gpu_sync(),
        })
    }

    pub(super) fn capture_world_dirtiness(&mut self, world: &WorldState) {
        if let Some(scene) = world.vertical_slice_scene() {
            for region in &scene.terrain.dirty_regions {
                self.chunk_dirtiness
                    .insert(ChunkId::new(region.chunk_x, region.chunk_y), true);
            }
        }
    }

    pub(super) fn update_manifest_from_world(&mut self, world: &WorldState) -> Result<(), String> {
        let scene = world
            .vertical_slice_scene()
            .ok_or_else(|| "No active scene found".to_string())?;
        let terrain = &scene.terrain;

        let mut min_height = 0.0;
        let mut max_height = 0.0;
        if let Some(first) = terrain.height_samples.first().copied() {
            min_height = first;
            max_height = first;
            for sample in &terrain.height_samples {
                min_height = min_height.min(*sample);
                max_height = max_height.max(*sample);
            }
        }

        let terrain_id = self
            .manifest
            .as_ref()
            .map(|manifest| manifest.terrain_id)
            .unwrap_or_else(Uuid::new_v4);

        let material_layers = terrain
            .layer_materials
            .iter()
            .map(|layer| MaterialLayer {
                layer_id: layer.layer_id,
                material_family: layer.material_family.clone(),
                density_kg_m3: self
                    .layer_bindings
                    .get(&LayerId(layer.layer_id))
                    .map(|profile| profile.density_kg_m3)
                    .unwrap_or(1000.0),
                albedo_texture_ref: layer.albedo_texture_ref.clone(),
                normal_texture_ref: layer.normal_texture_ref.clone(),
                orm_texture_ref: layer.orm_texture_ref.clone(),
                uv_scale: layer.uv_scale,
                base_tint: layer.base_tint,
            })
            .collect();

        let chunks = terrain
            .chunks
            .iter()
            .map(|chunk| ChunkDescriptor {
                chunk_x: chunk.chunk_x,
                chunk_y: chunk.chunk_y,
                data_file: chunk.data_file.clone().unwrap_or_default(),
                revision: terrain.mesh_revision,
            })
            .collect();

        self.manifest = Some(editor_dto_law::TerrainManifest {
            terrain_id,
            origin: terrain.origin,
            world_size: terrain.world_size,
            resolution: terrain.resolution,
            chunk_grid: terrain.chunk_grid,
            chunk_size: terrain.chunk_size,
            height_range: [min_height, max_height],
            material_layers,
            chunks,
        });
        Ok(())
    }
}

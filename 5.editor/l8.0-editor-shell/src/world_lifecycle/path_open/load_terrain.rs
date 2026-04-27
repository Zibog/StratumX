// Terrain loading from package

use editor_dto_law::{TerrainManifest, WorldPackageManifest};
use engine_world::{EntityId, TerrainChunk, TerrainLayerMaterialState, TerrainPatchState};
use std::fs;
use std::path::Path;

pub(super) fn load_terrain_from_package(
    world_path: &Path,
    manifest: &WorldPackageManifest,
) -> Result<TerrainPatchState, String> {
    let mut terrain_state = TerrainPatchState {
        entity_id: EntityId(1),
        origin: [0.0, 0.0, 0.0],
        world_size: [1000.0, 1000.0],
        resolution: [256, 256],
        chunk_grid: [4, 4],
        chunk_size: 64,
        height_source_ref: None,
        height_samples: vec![0.0; 256 * 256],
        material_layer_ids: vec![0],
        layer_weights: vec![[1.0, 0.0, 0.0, 0.0]; 256 * 256],
        layer_materials: vec![],
        hole_mask: None,
        chunks: vec![],
        dirty_regions: vec![],
        mesh_revision: 1,
        collision_revision: 1,
    };

    if let Some(terrain_ref) = &manifest.terrain_root_ref {
        let terrain_manifest_path = world_path.join(terrain_ref);
        if terrain_manifest_path.exists() {
            let terrain_json = fs::read_to_string(&terrain_manifest_path)
                .map_err(|e| format!("Failed to read terrain manifest: {}", e))?;
            let terrain_manifest: TerrainManifest = serde_json::from_str(&terrain_json)
                .map_err(|e| format!("Failed to parse terrain manifest: {}", e))?;

            terrain_state.origin = terrain_manifest.origin;
            terrain_state.world_size = terrain_manifest.world_size;
            terrain_state.resolution = terrain_manifest.resolution;
            terrain_state.chunk_grid = terrain_manifest.chunk_grid;
            terrain_state.chunk_size = terrain_manifest.chunk_size;

            // Load material layers from manifest - use canonical material IDs
            // CANONICAL: layer IDs reference Material_Registry_State profiles
            terrain_state.material_layer_ids = terrain_manifest
                .material_layers
                .iter()
                .map(|layer| layer.layer_id)
                .collect();

            // Load layer materials from manifest
            terrain_state.layer_materials = terrain_manifest
                .material_layers
                .iter()
                .map(|layer| TerrainLayerMaterialState {
                    layer_id: layer.layer_id,
                    material_family: layer.material_family.clone(),
                    albedo_texture_ref: layer.albedo_texture_ref.clone(),
                    normal_texture_ref: layer.normal_texture_ref.clone(),
                    orm_texture_ref: layer.orm_texture_ref.clone(),
                    uv_scale: layer.uv_scale,
                    base_tint: layer.base_tint,
                })
                .collect();

            // If no layers in manifest, use default profile
            if terrain_state.material_layer_ids.is_empty() {
                terrain_state.material_layer_ids = vec![1]; // DEFAULT_TERRAIN_MATERIAL_PROFILE
            }

            let total_samples =
                (terrain_manifest.resolution[0] * terrain_manifest.resolution[1]) as usize;
            terrain_state.height_samples = vec![0.0; total_samples];
            terrain_state.layer_weights = vec![[1.0, 0.0, 0.0, 0.0]; total_samples];

            for chunk_desc in &terrain_manifest.chunks {
                let chunk_path = world_path.join("terrain").join(&chunk_desc.data_file);
                let loaded = chunk_path.exists();

                if loaded {
                    if let Ok(chunk_bytes) = fs::read(&chunk_path) {
                        if let Ok(chunk_data) = editor_dto_law::ChunkData::from_bytes(&chunk_bytes)
                        {
                            let chunk_res = terrain_state.chunk_size as usize;
                            let terrain_res_x = terrain_state.resolution[0] as usize;
                            let chunk_offset_x = chunk_desc.chunk_x as usize * chunk_res;
                            let chunk_offset_y = chunk_desc.chunk_y as usize * chunk_res;

                            for y in 0..chunk_res {
                                for x in 0..chunk_res {
                                    let chunk_idx = y * chunk_res + x;
                                    let terrain_idx =
                                        (chunk_offset_y + y) * terrain_res_x + (chunk_offset_x + x);
                                    if chunk_idx < chunk_data.heights.len()
                                        && terrain_idx < terrain_state.height_samples.len()
                                    {
                                        terrain_state.height_samples[terrain_idx] =
                                            chunk_data.heights[chunk_idx];
                                        terrain_state.layer_weights[terrain_idx] =
                                            chunk_data.material_weights[chunk_idx];
                                    }
                                }
                            }
                        }
                    }
                }

                terrain_state.chunks.push(TerrainChunk {
                    chunk_x: chunk_desc.chunk_x,
                    chunk_y: chunk_desc.chunk_y,
                    data_file: Some(chunk_desc.data_file.clone()),
                    loaded,
                    dirty: false,
                    mesh_built: loaded,
                });
            }
        }
    }

    Ok(terrain_state)
}

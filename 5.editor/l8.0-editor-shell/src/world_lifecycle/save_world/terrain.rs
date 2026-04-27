// Terrain package saving - canonical chunk and material layer persistence

use editor_dto_law::{ChunkData, ChunkDescriptor, ChunkHeader, MaterialLayer, TerrainManifest};
use engine_world::VerticalSliceScene;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn save_terrain_package(path: &Path, scene: &VerticalSliceScene) -> Result<(), String> {
    let terrain_dir = path.join("terrain");
    let chunks_dir = terrain_dir.join("chunks");
    fs::create_dir_all(&chunks_dir).map_err(|e| format!("Failed to create chunks dir: {}", e))?;

    // CANONICAL: Material layers reference Material_Registry_State profiles
    // Map layer_materials to MaterialLayer structs
    let material_layers: Vec<MaterialLayer> = scene
        .terrain
        .layer_materials
        .iter()
        .map(|layer| MaterialLayer {
            layer_id: layer.layer_id,
            material_family: layer.material_family.clone(),
            density_kg_m3: 2000.0, // Default density
            albedo_texture_ref: layer.albedo_texture_ref.clone(),
            normal_texture_ref: layer.normal_texture_ref.clone(),
            orm_texture_ref: layer.orm_texture_ref.clone(),
            uv_scale: layer.uv_scale,
            base_tint: layer.base_tint,
        })
        .collect();

    let terrain_manifest = TerrainManifest {
        terrain_id: Uuid::new_v4(),
        origin: scene.terrain.origin,
        world_size: scene.terrain.world_size,
        resolution: scene.terrain.resolution,
        chunk_grid: scene.terrain.chunk_grid,
        chunk_size: scene.terrain.chunk_size,
        height_range: [-100.0, 500.0],
        material_layers,
        chunks: scene
            .terrain
            .chunks
            .iter()
            .map(|c| ChunkDescriptor {
                chunk_x: c.chunk_x,
                chunk_y: c.chunk_y,
                data_file: format!("chunks/chunk_{}_{}.bin", c.chunk_x, c.chunk_y),
                revision: 1,
            })
            .collect(),
    };

    // Write chunk binary data
    save_chunk_data(&chunks_dir, scene)?;

    // Write terrain manifest
    let terrain_json = serde_json::to_string_pretty(&terrain_manifest)
        .map_err(|e| format!("Failed to serialize terrain manifest: {}", e))?;
    fs::write(terrain_dir.join("terrain_manifest.json"), terrain_json)
        .map_err(|e| format!("Failed to write terrain_manifest.json: {}", e))?;

    Ok(())
}

fn save_chunk_data(chunks_dir: &Path, scene: &VerticalSliceScene) -> Result<(), String> {
    let chunk_res = scene.terrain.chunk_size as usize;
    let terrain_res_x = scene.terrain.resolution[0] as usize;

    for chunk in &scene.terrain.chunks {
        let chunk_offset_x = chunk.chunk_x as usize * chunk_res;
        let chunk_offset_y = chunk.chunk_y as usize * chunk_res;

        let mut chunk_heights = Vec::with_capacity(chunk_res * chunk_res);
        let mut chunk_weights = Vec::with_capacity(chunk_res * chunk_res);

        for y in 0..chunk_res {
            for x in 0..chunk_res {
                let terrain_idx = (chunk_offset_y + y) * terrain_res_x + (chunk_offset_x + x);
                if terrain_idx < scene.terrain.height_samples.len() {
                    chunk_heights.push(scene.terrain.height_samples[terrain_idx]);
                    chunk_weights.push(scene.terrain.layer_weights[terrain_idx]);
                } else {
                    chunk_heights.push(0.0);
                    chunk_weights.push([1.0, 0.0, 0.0, 0.0]);
                }
            }
        }

        let chunk_data = ChunkData {
            header: ChunkHeader::new(chunk_res as u32, chunk_res as u32),
            heights: chunk_heights,
            material_weights: chunk_weights,
        };

        let chunk_path = chunks_dir.join(format!("chunk_{}_{}.bin", chunk.chunk_x, chunk.chunk_y));
        fs::write(&chunk_path, chunk_data.to_bytes()).map_err(|e| {
            format!(
                "Failed to write chunk {},{}: {}",
                chunk.chunk_x, chunk.chunk_y, e
            )
        })?;
    }

    Ok(())
}

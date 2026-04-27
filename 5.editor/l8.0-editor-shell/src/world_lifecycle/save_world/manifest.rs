// World manifest saving - canonical package format

use editor_dto_law::{SourceLineage, StableWorldId, WorldPackageManifest, WorldRole};
use engine_world::VerticalSliceScene;
use std::fs;
use std::path::Path;

pub fn save_world_manifest(
    path: &Path,
    world_ref: StableWorldId,
    scene: &VerticalSliceScene,
) -> Result<(), String> {
    let timestamp = chrono::Utc::now().to_rfc3339();

    let manifest = WorldPackageManifest {
        world_id: world_ref.0,
        world_label: scene.scene_name.clone(),
        world_role: WorldRole::Content,
        version: "1.0.0".to_string(),
        terrain_root_ref: Some("terrain/terrain_manifest.json".to_string()),
        environment_root_ref: Some("environment/sky_binding.json".to_string()),
        streaming_profile_ref: None,
        source_lineage: SourceLineage {
            created_at: timestamp.clone(),
            created_by: "stratumx_editor".to_string(),
            import_source: None,
            last_modified: timestamp,
        },
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
    fs::write(path.join("world.json"), manifest_json)
        .map_err(|e| format!("Failed to write world.json: {}", e))?;

    Ok(())
}

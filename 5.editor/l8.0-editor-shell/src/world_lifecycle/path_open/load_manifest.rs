// World manifest loading

use editor_dto_law::{FailureClass, WorldOpenResult, WorldPackageManifest};
use std::fs;
use std::path::Path;

pub(super) fn load_world_manifest(path: &Path) -> Result<WorldPackageManifest, WorldOpenResult> {
    let world_manifest_path = path.join("world.json");

    let manifest_json = fs::read_to_string(&world_manifest_path).map_err(|e| WorldOpenResult {
        accepted: false,
        world_ref: None,
        world_label: None,
        failure_class: Some(FailureClass::CorruptedData),
        recovery_hints: vec![format!("Failed to read world.json: {}", e)],
    })?;

    let manifest: WorldPackageManifest =
        serde_json::from_str(&manifest_json).map_err(|e| WorldOpenResult {
            accepted: false,
            world_ref: None,
            world_label: None,
            failure_class: Some(FailureClass::CorruptedData),
            recovery_hints: vec![format!("Failed to parse world.json: {}", e)],
        })?;

    Ok(manifest)
}

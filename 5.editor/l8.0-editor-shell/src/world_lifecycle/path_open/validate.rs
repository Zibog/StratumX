// World path validation

use editor_dto_law::{FailureClass, WorldOpenResult};
use std::path::Path;

pub(super) fn validate_world_path(path: &Path) -> Result<(), WorldOpenResult> {
    if !path.exists() {
        return Err(WorldOpenResult {
            accepted: false,
            world_ref: None,
            world_label: None,
            failure_class: Some(FailureClass::WorldNotFound),
            recovery_hints: vec![
                format!("World path not found: {}", path.display()),
                "Check that the path exists".to_string(),
            ],
        });
    }

    if !path.is_dir() {
        return Err(WorldOpenResult {
            accepted: false,
            world_ref: None,
            world_label: None,
            failure_class: Some(FailureClass::CorruptedData),
            recovery_hints: vec![
                format!("Path is not a directory: {}", path.display()),
                "World must be a directory package".to_string(),
            ],
        });
    }

    let world_manifest_path = path.join("world.json");
    if !world_manifest_path.exists() {
        return Err(WorldOpenResult {
            accepted: false,
            world_ref: None,
            world_label: None,
            failure_class: Some(FailureClass::CorruptedData),
            recovery_hints: vec![
                format!("No world.json found in: {}", path.display()),
                "World package must contain world.json manifest".to_string(),
            ],
        });
    }

    Ok(())
}

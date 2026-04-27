// Fallback handling - graceful degradation

use super::WorldLifecycleManager;
use editor_dto_law::{FailureClass, WorldOpenResult};

impl WorldLifecycleManager {
    /// Fallback if failed - graceful degradation
    ///
    /// CANONICAL RULE: Falls back to startup world, NOT demo
    pub fn fallback_if_failed(&mut self, failure: FailureClass) -> WorldOpenResult {
        match failure {
            FailureClass::WorldNotFound => {
                // Try startup world (NOT demo)
                self.open_startup_world_from_package()
            }
            FailureClass::CorruptedData => WorldOpenResult {
                accepted: false,
                world_ref: None,
                world_label: None,
                failure_class: Some(FailureClass::CorruptedData),
                recovery_hints: vec![
                    "World data is corrupted".to_string(),
                    "Try opening a different world".to_string(),
                ],
            },
            FailureClass::IncompatibleVersion => WorldOpenResult {
                accepted: false,
                world_ref: None,
                world_label: None,
                failure_class: Some(FailureClass::IncompatibleVersion),
                recovery_hints: vec![
                    "World version is incompatible".to_string(),
                    "Update editor or migrate world".to_string(),
                ],
            },
            _ => WorldOpenResult {
                accepted: false,
                world_ref: None,
                world_label: None,
                failure_class: Some(failure),
                recovery_hints: vec!["Unknown failure".to_string()],
            },
        }
    }
}

// Explicit world opening - requires real path, NO demo fallback

use super::WorldLifecycleManager;
use editor_dto_law::{FailureClass, StableWorldId, WorldOpenResult};

impl WorldLifecycleManager {
    /// Open world explicitly by ID
    ///
    /// CANONICAL RULE: Does NOT fall back to demo
    /// Returns controlled failure if world not found
    pub fn open_selected_world(&mut self, world_ref: StableWorldId) -> WorldOpenResult {
        // Explicit open requires recent registry lookup
        if let Ok(recent_worlds) = self.get_recent_worlds() {
            if let Some((_, path, _label)) =
                recent_worlds.iter().find(|(id, _, _)| id.0 == world_ref.0)
            {
                let result = self.open_world_from_path(path);
                if result.accepted {
                    return result;
                }
            }
        }

        WorldOpenResult {
            accepted: false,
            world_ref: Some(world_ref),
            world_label: None,
            failure_class: Some(FailureClass::WorldNotFound),
            recovery_hints: vec![
                "World not found in recent registry".to_string(),
                "Use open_world_from_path() with valid path".to_string(),
            ],
        }
    }
}

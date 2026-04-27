// Session restore - crash recovery

use super::WorldLifecycleManager;
use editor_dto_law::WorldOpenResult;

impl WorldLifecycleManager {
    /// Restore previous session - crash recovery
    ///
    /// CANONICAL RULE: Restores existing session or falls back to startup (NOT demo)
    pub fn restore_previous_session(&mut self) -> WorldOpenResult {
        if self.world_state.is_some() {
            let world_ref = self
                .current_world
                .unwrap_or_else(|| editor_dto_law::StableWorldId(uuid::Uuid::new_v4()));

            WorldOpenResult {
                accepted: true,
                world_ref: Some(world_ref),
                world_label: Some("Restored Session".to_string()),
                failure_class: None,
                recovery_hints: vec![],
            }
        } else {
            // No existing world, use startup world (NOT demo)
            self.open_startup_world_from_package()
        }
    }
}

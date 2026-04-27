// World Lifecycle - Canonical Package-Based World Loading
// NO DEMO FALLBACK - Editor loads world packages only

pub mod explicit_open;
pub mod fallback;
pub mod path_open;
pub mod recent_open;
pub mod restore_open;
pub mod save_world;
pub mod startup_open;
pub mod state_access;
pub mod types;

pub use path_open::*;
pub use types::*;

use editor_dto_law::{StableWorldId, WorldBindState};
use engine_world::WorldState;
use std::path::PathBuf;

/// World Lifecycle Manager - Canonical Implementation
///
/// RULES:
/// - NO demo fallback on Startup/Explicit/Recent modes
/// - Loads ONLY from world packages (world.json + terrain + environment)
/// - Returns controlled failures, not silent demo substitution
pub struct WorldLifecycleManager {
    current_world: Option<StableWorldId>,
    bind_state: Option<WorldBindState>,
    world_state: Option<WorldState>,

    // Configured paths
    startup_world_path: Option<PathBuf>,
    reference_world_path: Option<PathBuf>,
}

impl WorldLifecycleManager {
    pub fn new() -> Self {
        Self {
            current_world: None,
            bind_state: None,
            world_state: None,
            startup_world_path: None,
            reference_world_path: None,
        }
    }

    /// Configure startup world path
    pub fn set_startup_world_path(&mut self, path: PathBuf) {
        self.startup_world_path = Some(path);
    }

    /// Configure reference world path
    pub fn set_reference_world_path(&mut self, path: PathBuf) {
        self.reference_world_path = Some(path);
    }
}

impl Default for WorldLifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

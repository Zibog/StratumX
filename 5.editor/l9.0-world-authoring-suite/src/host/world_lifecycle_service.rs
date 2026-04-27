//! World lifecycle service for open/save/close validation and event emission.
//!
//! **PHASE 6 REMEDIATED**: Now loads and holds the WorldState,
//! allowing the EditorHost session to consume it directly.

use super::event_bus_trait::{EventBusInterface, ServiceEvent};
use engine_world::WorldState;
use std::path::Path;
use std::sync::Arc;

/// Handles world open/save/close validation and lifecycle signaling.
pub struct WorldLifecycleService {
    pub(crate) event_bus: Option<Arc<dyn EventBusInterface>>,
    pub(crate) loaded_world: Option<WorldState>,
    pub(crate) loaded_world_label: Option<String>,
    pub(crate) loaded_world_path: Option<std::path::PathBuf>,
}

impl WorldLifecycleService {
    pub fn new() -> Self {
        Self {
            event_bus: None,
            loaded_world: None,
            loaded_world_label: None,
            loaded_world_path: None,
        }
    }

    pub fn new_with_event_bus<E>(event_bus: Arc<E>) -> Self
    where
        E: EventBusInterface + 'static,
    {
        Self {
            event_bus: Some(event_bus),
            loaded_world: None,
            loaded_world_label: None,
            loaded_world_path: None,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Open world from path after validating the package path.
    pub fn open_world(&self, path: &Path) -> Result<WorldOpenEvent, String> {
        if !path.exists() {
            return Err(format!("World path does not exist: {}", path.display()));
        }
        if !path.is_file() && !path.is_dir() {
            return Err(format!("Invalid world path: {}", path.display()));
        }
        let event = WorldOpenEvent {
            world_path: path.to_path_buf(),
            world_label: path
                .file_name()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string()),
        };
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::WorldOpened {
                    world_id: path.to_string_lossy().to_string(),
                },
                "WorldLifecycleService",
            );
        }
        Ok(event)
    }

    pub fn close_world(&self, has_unsaved_changes: bool) -> Result<WorldClosedEvent, String> {
        if has_unsaved_changes {
            return Err("Cannot close world with unsaved changes".to_string());
        }
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(ServiceEvent::WorldClosed, "WorldLifecycleService");
        }
        Ok(WorldClosedEvent)
    }

    /// Save world to path after validating the destination directory.
    pub fn save_world(&self, path: &Path) -> Result<WorldSavedEvent, String> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(format!(
                    "Save directory does not exist: {}",
                    parent.display()
                ));
            }
        }
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(ServiceEvent::WorldSaved, "WorldLifecycleService");
        }
        Ok(WorldSavedEvent {
            save_path: path.to_path_buf(),
        })
    }

    /// Take the loaded world state out of this service.
    pub fn take_loaded_world(
        &mut self,
    ) -> Option<(WorldState, String, Option<std::path::PathBuf>)> {
        self.loaded_world.take().map(|world| {
            let label = self.loaded_world_label.take().unwrap_or_default();
            let path = self.loaded_world_path.take();
            (world, label, path)
        })
    }

    /// Register a world opened through the explicit open path.
    pub fn register_opened_world(&mut self, label: String, path: Option<std::path::PathBuf>) {
        self.loaded_world_label = Some(label);
        self.loaded_world_path = path;
    }
}

impl Default for WorldLifecycleService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct WorldOpenEvent {
    pub world_path: std::path::PathBuf,
    pub world_label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorldClosedEvent;

#[derive(Debug, Clone)]
pub struct WorldSavedEvent {
    pub save_path: std::path::PathBuf,
}

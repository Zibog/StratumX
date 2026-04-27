// Recent world opening - uses saved paths, NO demo fallback

use super::WorldLifecycleManager;
use editor_dto_law::{FailureClass, StableWorldId, WorldOpenResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecentWorldEntry {
    world_id: Uuid,
    world_path: PathBuf,
    world_label: String,
    last_opened: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecentWorldRegistry {
    entries: Vec<RecentWorldEntry>,
}

impl RecentWorldRegistry {
    fn load() -> Result<Self, String> {
        let registry_path = Self::registry_path();

        if !registry_path.exists() {
            return Ok(Self {
                entries: Vec::new(),
            });
        }

        let json = fs::read_to_string(&registry_path)
            .map_err(|e| format!("Failed to read recent worlds registry: {}", e))?;

        serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse recent worlds registry: {}", e))
    }

    fn save(&self) -> Result<(), String> {
        let registry_path = Self::registry_path();

        if let Some(parent) = registry_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create registry directory: {}", e))?;
        }

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize registry: {}", e))?;

        fs::write(&registry_path, json).map_err(|e| format!("Failed to write registry: {}", e))
    }

    fn registry_path() -> PathBuf {
        // Store in user's config directory
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("stratumx").join("recent_worlds.json")
        } else {
            PathBuf::from("recent_worlds.json")
        }
    }

    fn find_by_id(&self, world_id: Uuid) -> Option<&RecentWorldEntry> {
        self.entries.iter().find(|e| e.world_id == world_id)
    }

    fn add_or_update(&mut self, world_id: Uuid, world_path: PathBuf, world_label: String) {
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(entry) = self.entries.iter_mut().find(|e| e.world_id == world_id) {
            entry.world_path = world_path;
            entry.world_label = world_label;
            entry.last_opened = now;
        } else {
            self.entries.push(RecentWorldEntry {
                world_id,
                world_path,
                world_label,
                last_opened: now,
            });
        }

        // Keep only last 10 entries
        if self.entries.len() > 10 {
            self.entries
                .sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
            self.entries.truncate(10);
        }
    }
}

impl WorldLifecycleManager {
    /// Open recent world from saved path
    ///
    /// CANONICAL RULE: Uses saved world paths, NOT demo
    /// Falls back to startup world if recent not available
    pub fn open_recent_world(&mut self, world_ref: StableWorldId) -> WorldOpenResult {
        // Load recent worlds registry
        let registry = match RecentWorldRegistry::load() {
            Ok(r) => r,
            Err(e) => {
                return WorldOpenResult {
                    accepted: false,
                    world_ref: None,
                    world_label: None,
                    failure_class: Some(FailureClass::CorruptedData),
                    recovery_hints: vec![
                        format!("Failed to load recent worlds registry: {}", e),
                        "Falling back to startup world".to_string(),
                    ],
                };
            }
        };

        // Find world in registry
        if let Some(entry) = registry.find_by_id(world_ref.0) {
            // Validate path still exists
            if !entry.world_path.exists() {
                return WorldOpenResult {
                    accepted: false,
                    world_ref: Some(world_ref),
                    world_label: Some(entry.world_label.clone()),
                    failure_class: Some(FailureClass::WorldNotFound),
                    recovery_hints: vec![
                        format!("Recent world path no longer exists: {:?}", entry.world_path),
                        "World may have been moved or deleted".to_string(),
                        "Falling back to startup world".to_string(),
                    ],
                };
            }

            // Try to open from path
            let result = self.open_world_from_path(&entry.world_path);

            if result.accepted {
                return result;
            } else {
                // Path exists but world failed to load - controlled failure
                return WorldOpenResult {
                    accepted: false,
                    world_ref: Some(world_ref),
                    world_label: Some(entry.world_label.clone()),
                    failure_class: result.failure_class,
                    recovery_hints: vec![
                        format!("Failed to load recent world from {:?}", entry.world_path),
                        "World package may be corrupted".to_string(),
                        "Falling back to startup world".to_string(),
                    ],
                };
            }
        }

        // World not found in registry - fall back to startup
        WorldOpenResult {
            accepted: false,
            world_ref: Some(world_ref),
            world_label: None,
            failure_class: Some(FailureClass::WorldNotFound),
            recovery_hints: vec![
                "World not found in recent worlds registry".to_string(),
                "Falling back to startup world".to_string(),
            ],
        }
    }

    /// Register world as recently opened
    pub fn register_recent_world(
        &self,
        world_ref: StableWorldId,
        world_path: PathBuf,
        world_label: String,
    ) -> Result<(), String> {
        let mut registry = RecentWorldRegistry::load()?;
        registry.add_or_update(world_ref.0, world_path, world_label);
        registry.save()
    }

    /// Get list of recent worlds
    pub fn get_recent_worlds(&self) -> Result<Vec<(StableWorldId, PathBuf, String)>, String> {
        let registry = RecentWorldRegistry::load()?;
        Ok(registry
            .entries
            .iter()
            .map(|e| {
                (
                    StableWorldId(e.world_id),
                    e.world_path.clone(),
                    e.world_label.clone(),
                )
            })
            .collect())
    }
}

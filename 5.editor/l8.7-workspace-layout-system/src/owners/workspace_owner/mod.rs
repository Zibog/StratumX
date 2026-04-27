//! Workspace owner aggregate.
//!
//! This module makes workspace refs, layout truth, and active surfaces explicit
//! while preserving the existing mutation and validation API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

mod active_surfaces;
mod layout_truth;
mod workspace_refs;

pub use layout_truth::{DockPosition, DockingConfig, PanelGeometry};
pub use workspace_refs::PanelId;

/// Workspace state container.
///
/// Owns workspace-level state including panel layout, positions, docking,
/// and provides serialization/deserialization for persistence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceOwner {
    /// Schema version for migration support.
    pub schema_version: u32,

    /// List of open panel IDs.
    pub open_panel_ids: Vec<PanelId>,

    /// Panel positions and sizes.
    pub panel_positions: HashMap<PanelId, PanelGeometry>,

    /// Currently focused panel.
    pub focused_panel: Option<PanelId>,

    /// Docking configuration.
    pub docking_configuration: DockingConfig,
}

impl WorkspaceOwner {
    /// Current schema version.
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;

    /// Creates a new workspace state with default layout.
    pub fn new() -> Self {
        Self {
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            open_panel_ids: Vec::new(),
            panel_positions: HashMap::new(),
            focused_panel: None,
            docking_configuration: DockingConfig::default(),
        }
    }

    /// Serializes workspace state to a file.
    pub fn serialize_to_file(&self, path: &Path) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, json)
    }

    /// Deserializes workspace state from a file.
    pub fn deserialize_from_file(path: &Path) -> io::Result<Self> {
        let json = fs::read_to_string(path)?;
        let mut state: Self = serde_json::from_str(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if state.schema_version > Self::CURRENT_SCHEMA_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Workspace state schema version {} is newer than supported version {}",
                    state.schema_version,
                    Self::CURRENT_SCHEMA_VERSION
                ),
            ));
        }

        if state.schema_version < Self::CURRENT_SCHEMA_VERSION {
            state = Self::migrate(state)?;
        }

        Ok(state)
    }

    /// Migrates workspace state from older schema versions to current version.
    fn migrate(mut state: Self) -> io::Result<Self> {
        let original_version = state.schema_version;

        while state.schema_version < Self::CURRENT_SCHEMA_VERSION {
            state.schema_version += 1;
        }

        if original_version != state.schema_version {
            eprintln!(
                "Migrated workspace state from schema version {} to {}",
                original_version, state.schema_version
            );
        }

        Ok(state)
    }
}

impl Default for WorkspaceOwner {
    fn default() -> Self {
        Self::new()
    }
}

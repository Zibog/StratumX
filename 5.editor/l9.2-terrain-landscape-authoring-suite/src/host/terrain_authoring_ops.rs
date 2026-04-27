//! Terrain Authoring Operations
//!
//! Temporary bridge for terrain authoring operations until fully migrated to command spine.
//! This module provides the TerrainAuthoringOps struct that wraps terrain tool state
//! and brush settings for the UI.

use serde::{Deserialize, Serialize};

/// Terrain tool types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TerrainTool {
    #[default]
    Select,
    SculptRaise,
    SculptLower,
    SculptSmooth,
    SculptFlatten,
    PaintMaterial,
}

/// Brush settings for terrain editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushSettings {
    pub radius: f32,
    pub strength: f32,
    pub target_layer: u32,
    pub target_height: f32,
    pub continuous: bool,
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self {
            radius: 10.0,
            strength: 0.5,
            target_layer: 0,
            target_height: 0.0,
            continuous: false,
        }
    }
}

/// Terrain authoring state (transient UI state)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerrainAuthoringState {
    #[serde(default)]
    pub current_tool: TerrainTool,
    #[serde(default)]
    pub brush: BrushSettings,
    #[serde(default)]
    pub show_dirty_regions: bool,
    #[serde(default)]
    pub show_chunk_boundaries: bool,
    #[serde(default)]
    pub show_material_weights: bool,
    #[serde(default)]
    pub show_lod: bool,
}

/// Terrain authoring operations wrapper
#[derive(Debug, Clone)]
pub struct TerrainAuthoringOps {
    state: TerrainAuthoringState,
}

impl TerrainAuthoringOps {
    pub fn new() -> Self {
        Self {
            state: TerrainAuthoringState::default(),
        }
    }

    pub fn state(&self) -> &TerrainAuthoringState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut TerrainAuthoringState {
        &mut self.state
    }

    pub fn current_tool(&self) -> TerrainTool {
        self.state.current_tool
    }

    pub fn set_tool(&mut self, tool: TerrainTool) {
        self.state.current_tool = tool;
    }

    pub fn brush_settings(&self) -> &BrushSettings {
        &self.state.brush
    }

    pub fn brush_settings_mut(&mut self) -> &mut BrushSettings {
        &mut self.state.brush
    }
}

impl Default for TerrainAuthoringOps {
    fn default() -> Self {
        Self::new()
    }
}

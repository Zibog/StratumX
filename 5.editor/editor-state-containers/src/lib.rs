//! FUTURE_STUB: this crate is a reserved StratumX editor surface.
//! It is intentionally not product-integrated yet.
//! It must not report fake success or be treated as production-ready.

use serde::{Deserialize, Serialize};

// Re-export diagnostic types from the diagnostics surface crate
pub use stratumx_editor_l8_10_diagnostics_surface::diagnostics_types::{
    DiagnosticMessage, DiagnosticSource, FailureCode, Severity, TraceId, TraceLineage,
};

// Re-export ArtifactRef from diagnostics surface
pub use stratumx_editor_l8_10_diagnostics_surface::ArtifactRef;

pub mod cache;
pub mod owners;
pub mod persistence;
pub mod queries;
pub mod runtime;
pub mod validation;

pub use cache::*;
pub use owners::*;
pub use persistence::*;
pub use queries::*;
pub use runtime::*;
pub use validation::*;

// Top-level types
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldIdentity {
    pub world_id: Uuid,
    pub world_name: String,
    pub world_path: PathBuf,
}

impl WorldIdentity {
    pub fn new(world_id: Uuid, world_name: String, world_path: PathBuf) -> Self {
        Self {
            world_id,
            world_name,
            world_path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeatherCondition {
    Clear,
    Rain,
    Snow,
    Fog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    pub time_of_day: f32,
    pub weather: WeatherCondition,
}

impl Default for EnvironmentState {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentState {
    pub fn new() -> Self {
        Self {
            time_of_day: 12.0,
            weather: WeatherCondition::Clear,
        }
    }
    pub fn set_time_of_day(&mut self, hours: f32) {
        self.time_of_day = hours;
    }
    pub fn set_weather(&mut self, weather: WeatherCondition) {
        self.weather = weather;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterialProfileId(pub uuid::Uuid);

impl MaterialProfileId {
    pub fn new(id: uuid::Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialProfile {
    pub profile_id: MaterialProfileId,
    pub name: String,
    pub profile_name: String, // Alias for name
    pub base_color: [f32; 4],
    pub roughness: f32,
    pub metallic: f32,
}

impl MaterialProfile {
    pub fn new(profile_id: MaterialProfileId, name: String) -> Self {
        Self {
            profile_id,
            profile_name: name.clone(),
            name,
            base_color: [1.0, 1.0, 1.0, 1.0],
            roughness: 0.5,
            metallic: 0.0,
        }
    }

    // Alias for compatibility
    pub fn id(&self) -> &MaterialProfileId {
        &self.profile_id
    }
}

// Event types
#[derive(Debug, Clone)]
pub enum ProjectStateEvent {
    SaveGenerationIncremented { new_generation: u64 },
}

#[derive(Debug, Clone)]
pub enum WorldStateEvent {
    TerrainStateUpdated,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

// Session state types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub project_state: Option<runtime::state_types::ProjectState>,
    pub workspace_state: Option<runtime::state_types::WorkspaceState>,
    pub world_state: Option<runtime::state_types::WorldState>,
    pub active_world: Option<WorldIdentity>,
    pub selection_state: SelectionState,
    pub tool_mode: Option<ToolMode>,
    pub open_panels: Vec<PanelId>,
    pub focused_panel: Option<PanelId>,
    pub recently_opened_worlds: Vec<WorldIdentity>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            project_state: None,
            workspace_state: None,
            world_state: None,
            active_world: None,
            selection_state: SelectionState {
                selected_entities: Vec::new(),
                selection_mode: None,
            },
            tool_mode: None,
            open_panels: Vec::new(),
            focused_panel: None,
            recently_opened_worlds: Vec::new(),
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(self).map_err(|e| e.to_string())
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(data).map_err(|e| e.to_string())
    }

    pub fn apply_modification(&mut self, modification: StateModification) -> Result<(), String> {
        match modification {
            StateModification::AddToSelection(entity_id) => {
                self.selection_state.selected_entities.push(entity_id);
            }
            StateModification::SetSelection(entities) => {
                self.selection_state.selected_entities = entities;
            }
            StateModification::AddOpenPanel(panel_id) => {
                self.add_open_panel(panel_id);
            }
            StateModification::SetToolMode(tool_mode) => {
                self.tool_mode = parse_tool_mode(&tool_mode);
            }
            StateModification::SetSelectionMode(selection_mode) => {
                self.selection_state.selection_mode = parse_selection_mode(&selection_mode);
            }
            StateModification::SetActiveWorld(world_identity) => {
                self.active_world = Some(world_identity);
            }
            StateModification::SetFocusedPanel(panel_id) => {
                self.focused_panel = panel_id;
            }
        }

        Ok(())
    }

    pub fn validate_modification(&self, _modification: &StateModification) -> Result<(), ValidationError> {
        Ok(())
    }

    pub fn get_selection_state(&self) -> Option<&SelectionState> {
        Some(&self.selection_state)
    }

    pub fn get_open_panels(&self) -> Vec<PanelId> {
        self.open_panels.clone()
    }

    pub fn add_recently_opened_world(&mut self, world: WorldIdentity) {
        self.recently_opened_worlds.push(world);
    }

    pub fn get_recently_opened_worlds(&self) -> Vec<WorldIdentity> {
        self.recently_opened_worlds.clone()
    }

    pub fn get_tool_mode(&self) -> Option<String> {
        self.tool_mode.as_ref().map(|tm| format!("{:?}", tm))
    }

    pub fn set_active_world(&mut self, world: Option<WorldIdentity>) {
        self.active_world = world;
    }

    pub fn set_tool_mode(&mut self, tool_mode: ToolMode) {
        self.tool_mode = Some(tool_mode);
    }

    pub fn set_selection_mode(&mut self, selection_mode: Option<SelectionMode>) {
        self.selection_state.selection_mode = selection_mode;
    }

    pub fn get_active_world(&self) -> Option<&WorldIdentity> {
        self.active_world.as_ref()
            .or_else(|| self.world_state.as_ref().map(|ws| &ws.identity))
    }

    pub fn get_focused_panel(&self) -> Option<&PanelId> {
        self.focused_panel.as_ref()
    }

    pub fn add_open_panel(&mut self, panel_id: PanelId) {
        if !self.open_panels.contains(&panel_id) {
            self.open_panels.push(panel_id);
        }
    }

    pub fn has_selection(&self) -> bool {
        !self.selection_state.selected_entities.is_empty()
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum StateModification {
    AddToSelection(Uuid),
    SetSelection(Vec<Uuid>),
    AddOpenPanel(PanelId),
    SetToolMode(String),
    SetSelectionMode(String),
    SetActiveWorld(WorldIdentity),
    SetFocusedPanel(Option<PanelId>),
}

// Audio types
#[derive(Debug, Clone)]
pub struct AudioRegistryState {
    pub sources: Vec<AudioSource>,
}

impl AudioRegistryState {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: AudioSource) {
        if let Some(existing) = self.sources.iter_mut().find(|existing| existing.id == source.id) {
            *existing = source;
        } else {
            self.sources.push(source);
        }
    }
}

impl Default for AudioRegistryState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioSource {
    pub id: AudioSourceId,
    pub source_type: AudioSourceType,
}

impl AudioSource {
    pub fn new(id: AudioSourceId, source_type: AudioSourceType) -> Self {
        Self { id, source_type }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AudioSourceId(pub Uuid);

impl AudioSourceId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioSourceType {
    Ambient,
    Point,
}

// Selection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionState {
    pub selected_entities: Vec<Uuid>,
    pub selection_mode: Option<SelectionMode>,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selected_entities: Vec::new(),
            selection_mode: None,
        }
    }

    pub fn has_selection(&self) -> bool {
        !self.selected_entities.is_empty()
    }
}

impl Default for SelectionState {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_tool_mode(value: &str) -> Option<ToolMode> {
    match value {
        "Select" => Some(ToolMode::Select),
        "Move" => Some(ToolMode::Move),
        "Rotate" => Some(ToolMode::Rotate),
        "Scale" => Some(ToolMode::Scale),
        _ => None,
    }
}

fn parse_selection_mode(value: &str) -> Option<SelectionMode> {
    match value {
        "Single" => Some(SelectionMode::Single),
        "Multiple" => Some(SelectionMode::Multiple),
        "Box" => Some(SelectionMode::Box),
        _ => None,
    }
}

// State queries trait
pub trait StateQueries {
    fn get_selection(&self) -> &SelectionState;
    fn get_active_world(&self) -> Option<&WorldIdentity>;
    fn get_focused_panel(&self) -> Option<&PanelId>;
    fn get_open_panels(&self) -> Vec<PanelId>;
    fn get_material_profiles(&self) -> Vec<MaterialProfile>;
    fn get_audio_sources(&self) -> Vec<AudioSource>;
    fn get_diagnostics(&self) -> Vec<DiagnosticMessage>;
    fn has_project(&self) -> bool;
    fn has_active_world(&self) -> bool;
    fn has_selection(&self) -> bool;
}

// Additional types for tests
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolMode {
    Select,
    Move,
    Rotate,
    Scale,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectionMode {
    Single,
    Multiple,
    Box,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorCode {
    InvalidState,
    MissingRequirement,
    ConflictingModification,
    InvalidWorldIdentity,
    InvalidEntityId,
    InvalidPanelId,
    InvalidToolMode,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub code: ValidationErrorCode,
    pub message: String,
}

impl ValidationError {
    pub fn new(code: ValidationErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

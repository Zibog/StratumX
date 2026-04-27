use serde::{Deserialize, Serialize};

// ============================================================================
// RUNTIME INSPECTOR STATE
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeInspectorState {
    pub selected_entity: Option<u32>,
    pub material_stack_view: Option<MaterialStackView>,
    pub damage_view: Option<DamageView>,
    pub shot_history: Vec<ShotHistoryEntry>,
    pub diagnostics: RuntimeDiagnostics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialStackView {
    pub stack_id: u16,
    pub label: String,
    pub layers: Vec<MaterialLayerView>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayerView {
    pub layer_index: u8,
    pub archetype_label: String,
    pub thickness_mm: f32,
    pub coverage: f32,
    pub segmentation_mode: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageView {
    pub entity_id: u32,
    pub layers: Vec<LayerDamageView>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerDamageView {
    pub layer_index: u8,
    pub integrity: f32,
    pub accumulated_energy_j: f32,
    pub cracked_segments_count: usize,
    pub released_segments_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShotHistoryEntry {
    pub tick: u64,
    pub weapon_entity: u32,
    pub impact_position: Option<[f32; 3]>,
    pub entry_energy_j: Option<f32>,
    pub incidence_angle_deg: Option<f32>,
    pub verdict: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeDiagnostics {
    pub total_shots: usize,
    pub total_impacts: usize,
    pub total_segments_cracked: usize,
    pub total_segments_released: usize,
    pub last_simulation_time_ms: f32,
}

impl Default for RuntimeInspectorState {
    fn default() -> Self {
        Self {
            selected_entity: None,
            material_stack_view: None,
            damage_view: None,
            shot_history: Vec::new(),
            diagnostics: RuntimeDiagnostics {
                total_shots: 0,
                total_impacts: 0,
                total_segments_cracked: 0,
                total_segments_released: 0,
                last_simulation_time_ms: 0.0,
            },
        }
    }
}

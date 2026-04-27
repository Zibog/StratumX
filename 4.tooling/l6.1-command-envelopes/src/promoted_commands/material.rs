use serde::{Deserialize, Serialize};

/// Material authority lifecycle commands
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MaterialAuthorityCommand {
    Initialize,
    Dispose,
}

/// Material authoring commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialCommand {
    Create {
        material_name: String,
    },
    Delete {
        material_id: String,
    },
    DuplicateProfile {
        source_material_id: String,
        target_name: String,
    },
    BindVisualResponse {
        material_id: String,
        visual_family: String,
    },
    BindAcousticProfile {
        material_id: String,
        acoustic_profile: String,
    },
    BindLightResponse {
        material_id: String,
        light_response: String,
    },
    BindMicrodetailProfile {
        material_id: String,
        microdetail_profile: String,
    },
    BindWeatherModulation {
        material_id: String,
        weather_modulation: String,
    },
    PreviewBurn {
        material_id: String,
        preview_target: String,
    },
    SetCheapRuntimeRung {
        material_id: String,
        rung_level: u32,
    },
    InspectBranchCoverage {
        material_id: String,
    },
}

use serde::{Deserialize, Serialize};

/// Audio authority lifecycle commands
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AudioAuthorityCommand {
    Initialize,
    Dispose,
}

/// Audio authoring commands
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioCommand {
    CreateSource {
        source_name: String,
    },
    BindWorldSource {
        source_id: String,
        position: [f32; 3],
    },
    SetAcousticProfile {
        source_id: String,
        profile: String,
    },
    AssignEmitterClassWorldSource {
        source_id: String,
        emitter_class: String,
    },
    BindZoneProfileWorldSurface {
        zone_id: String,
        reverb_profile: String,
    },
    BindPriorityDuckingPolicy {
        policy_id: String,
    },
    PreviewAudibilityFreeCamera {
        listener_profile: String,
    },
    PreviewObstructionVsOcclusion {
        path_id: String,
    },
    PreviewIndoorOutdoorTransition {
        transition_path: String,
    },
    PreviewVoiceSubtitleLegality {
        dialogue_id: String,
    },
}

// Audio command builders

use crate::promoted_commands::types::PromotedCommand;

/// Builder for audio commands
pub struct AudioCommandBuilder;

impl AudioCommandBuilder {
    pub fn create_source(source_name: impl Into<String>) -> PromotedCommand {
        PromotedCommand::AudioCreateSource {
            source_name: source_name.into(),
        }
    }

    pub fn bind_world_source(source_id: impl Into<String>, position: [f32; 3]) -> PromotedCommand {
        PromotedCommand::AudioBindWorldSource {
            source_id: source_id.into(),
            position,
        }
    }

    pub fn set_acoustic_profile(
        source_id: impl Into<String>,
        profile: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::AudioSetAcousticProfile {
            source_id: source_id.into(),
            profile: profile.into(),
        }
    }
}

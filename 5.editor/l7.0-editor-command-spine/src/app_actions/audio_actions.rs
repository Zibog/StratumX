// Audio authoring actions

use super::ActionPayload;
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

pub fn create_source(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    payload
        .and_then(|p| p.as_string().map(|s| s.to_string()))
        .map(|source_name| PromotedCommand::AudioCreateSource { source_name })
}

pub fn bind_world_source(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::Multiple(parts)) = payload {
        if parts.len() >= 2 {
            if let (Some(source_id), Some(position)) = (
                parts[0].as_string().map(|s| s.to_string()),
                parts[1].as_vec3(),
            ) {
                return Some(PromotedCommand::AudioBindWorldSource {
                    source_id,
                    position,
                });
            }
        }
    }
    None
}

pub fn set_acoustic_profile(payload: Option<ActionPayload>) -> Option<PromotedCommand> {
    if let Some(ActionPayload::Multiple(parts)) = payload {
        if parts.len() >= 2 {
            if let (Some(source_id), Some(profile)) = (
                parts[0].as_string().map(|s| s.to_string()),
                parts[1].as_string().map(|s| s.to_string()),
            ) {
                return Some(PromotedCommand::AudioSetAcousticProfile { source_id, profile });
            }
        }
    }
    None
}

// Audio command validators

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;
use crate::validation::rules::*;

pub fn validate_audio_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        PromotedCommand::AudioAuthorityInitialize | PromotedCommand::AudioAuthorityDispose => {
            ValidationResult::Valid
        }
        PromotedCommand::AudioCreateSource { source_name } => {
            validate_non_empty(source_name, "source_name")
        }
        PromotedCommand::AudioBindWorldSource {
            source_id,
            position,
        } => validate_audio_source(source_id, position),
        PromotedCommand::AudioSetAcousticProfile { source_id, profile } => {
            validate_audio_binding(source_id, profile)
        }
        PromotedCommand::AudioAssignEmitterClassWorldSource {
            source_id,
            emitter_class,
        } => validate_audio_binding(source_id, emitter_class),
        PromotedCommand::AudioBindZoneProfileWorldSurface {
            zone_id,
            reverb_profile,
        } => validate_audio_binding(zone_id, reverb_profile),
        PromotedCommand::AudioBindPriorityDuckingPolicy { policy_id } => {
            validate_non_empty(policy_id, "policy_id")
        }
        PromotedCommand::AudioPreviewAudibilityFreeCamera { listener_profile } => {
            validate_non_empty(listener_profile, "listener_profile")
        }
        PromotedCommand::AudioPreviewObstructionVsOcclusion { path_id } => {
            validate_non_empty(path_id, "path_id")
        }
        PromotedCommand::AudioPreviewIndoorOutdoorTransition { transition_path } => {
            validate_non_empty(transition_path, "transition_path")
        }
        PromotedCommand::AudioPreviewVoiceSubtitleLegality { dialogue_id } => {
            validate_non_empty(dialogue_id, "dialogue_id")
        }
        _ => ValidationResult::Valid,
    }
}

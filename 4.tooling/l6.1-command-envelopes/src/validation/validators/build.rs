// Build, validation, automation, and scene command validators

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;

pub fn validate_build_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        PromotedCommand::BuildRun
        | PromotedCommand::BuildRelease
        | PromotedCommand::ValidationRunFull
        | PromotedCommand::ValidationRunSmoke
        | PromotedCommand::AutomationRebuildAll
        | PromotedCommand::AutomationValidateAll
        | PromotedCommand::SceneBootstrap
        | PromotedCommand::SceneReset
        | PromotedCommand::SceneFireTestShot { .. } => ValidationResult::Valid,
        _ => ValidationResult::Valid,
    }
}

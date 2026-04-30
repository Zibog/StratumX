// Graphics and shell command validators

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;

pub fn validate_graphics_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        PromotedCommand::ShellActivateViewport
        | PromotedCommand::ShellActivateOutliner
        | PromotedCommand::ShellActivateInspector
        | PromotedCommand::ShellActivateContentBrowser
        | PromotedCommand::ShellActivateMaterialLab
        | PromotedCommand::ShellActivateTerrainLab
        | PromotedCommand::ShellActivateSkyLab => ValidationResult::Valid,
        _ => ValidationResult::Valid,
    }
}

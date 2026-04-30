// Runtime and world command validators

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;
use crate::validation::rules::*;

pub fn validate_runtime_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        PromotedCommand::WorldOpen { world_path } => validate_path(world_path, "world_path"),
        PromotedCommand::WorldSave { world_path } => validate_path(world_path, "world_path"),
        PromotedCommand::WorldClose
        | PromotedCommand::RuntimePlay
        | PromotedCommand::RuntimePause
        | PromotedCommand::RuntimeStop
        | PromotedCommand::RuntimeSimulate => ValidationResult::Valid,
        _ => ValidationResult::Valid,
    }
}

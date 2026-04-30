// Project command validators

use crate::promoted_commands::PromotedCommand;
use crate::validation::errors::ValidationResult;
use crate::validation::rules::*;

pub fn validate_project_command(cmd: &PromotedCommand) -> ValidationResult {
    match cmd {
        PromotedCommand::ProjectBootstrap { project_name } => validate_project_name(project_name),
        PromotedCommand::ProjectCreate {
            project_name,
            project_root,
            world_name,
        } => validate_project_create(project_name, project_root, world_name),
        PromotedCommand::ProjectSave { save_path } => validate_path(save_path, "save_path"),
        PromotedCommand::ProjectBuild { target_platform } => {
            validate_non_empty(target_platform, "target_platform")
        }
        PromotedCommand::ProjectExport { export_path } => validate_path(export_path, "export_path"),
        PromotedCommand::ProjectLaunch { launch_mode } => {
            validate_non_empty(launch_mode, "launch_mode")
        }
        PromotedCommand::ProjectVerifyFirstResult => ValidationResult::Valid,
        _ => ValidationResult::Valid,
    }
}

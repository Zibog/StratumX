// Build and release command builders

use crate::promoted_commands::types::PromotedCommand;

/// Builder for project commands
pub struct ProjectCommandBuilder;

impl ProjectCommandBuilder {
    pub fn bootstrap(project_name: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectBootstrap {
            project_name: project_name.into(),
        }
    }

    pub fn create(
        project_name: impl Into<String>,
        project_root: impl Into<String>,
        world_name: impl Into<String>,
    ) -> PromotedCommand {
        PromotedCommand::ProjectCreate {
            project_name: project_name.into(),
            project_root: project_root.into(),
            world_name: world_name.into(),
        }
    }

    pub fn save(save_path: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectSave {
            save_path: save_path.into(),
        }
    }

    pub fn build(target_platform: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectBuild {
            target_platform: target_platform.into(),
        }
    }

    pub fn export(export_path: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectExport {
            export_path: export_path.into(),
        }
    }

    pub fn launch(launch_mode: impl Into<String>) -> PromotedCommand {
        PromotedCommand::ProjectLaunch {
            launch_mode: launch_mode.into(),
        }
    }

    pub fn verify_first_result() -> PromotedCommand {
        PromotedCommand::ProjectVerifyFirstResult
    }
}

//! Project and world dialog state owned by the shell crate.

use std::path::Path;

use serde::{Deserialize, Serialize};
use stratumx_tooling_l6_1_command_envelopes::PromotedCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectDialogKind {
    NewProject,
    OpenWorld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationTone {
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogValidation {
    pub tone: ValidationTone,
    pub message: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewProjectDialogState {
    pub is_open: bool,
    pub project_name: String,
    pub project_path: String,
    pub world_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenWorldDialogState {
    pub is_open: bool,
    pub world_path: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectDialogsState {
    pub new_project: NewProjectDialogState,
    pub open_world: OpenWorldDialogState,
}

impl ProjectDialogsState {
    pub fn show(&mut self, kind: ProjectDialogKind) {
        match kind {
            ProjectDialogKind::NewProject => self.new_project.is_open = true,
            ProjectDialogKind::OpenWorld => self.open_world.is_open = true,
        }
    }

    pub fn close(&mut self, kind: ProjectDialogKind) {
        match kind {
            ProjectDialogKind::NewProject => self.new_project.is_open = false,
            ProjectDialogKind::OpenWorld => self.open_world.is_open = false,
        }
    }

    pub fn validate_new_project_root(&self) -> Option<DialogValidation> {
        let path = self.new_project.project_path.trim();
        if path.is_empty() {
            return None;
        }

        let path = Path::new(path);
        if !path.exists() {
            Some(DialogValidation {
                tone: ValidationTone::Warning,
                message: "Path does not exist (will be created)".to_string(),
            })
        } else if !path.is_dir() {
            Some(DialogValidation {
                tone: ValidationTone::Error,
                message: "Path exists but is not a directory".to_string(),
            })
        } else {
            Some(DialogValidation {
                tone: ValidationTone::Success,
                message: "Path exists".to_string(),
            })
        }
    }

    pub fn validate_open_world_path(&self) -> Option<DialogValidation> {
        let path = self.open_world.world_path.trim();
        if path.is_empty() {
            return None;
        }

        let path = Path::new(path);
        if !path.exists() {
            Some(DialogValidation {
                tone: ValidationTone::Error,
                message: "Path does not exist".to_string(),
            })
        } else if !path.is_dir() {
            Some(DialogValidation {
                tone: ValidationTone::Error,
                message: "Path must be a directory".to_string(),
            })
        } else if !path.join("world.json").exists() {
            Some(DialogValidation {
                tone: ValidationTone::Warning,
                message: "Directory exists but missing world.json".to_string(),
            })
        } else {
            Some(DialogValidation {
                tone: ValidationTone::Success,
                message: "Valid world package".to_string(),
            })
        }
    }

    pub fn build_new_project_command(&self) -> Result<PromotedCommand, String> {
        let project_name = self.new_project.project_name.trim();
        let project_root = self.new_project.project_path.trim();
        let world_name = self.new_project.world_name.trim();

        if project_name.is_empty() || project_root.is_empty() || world_name.is_empty() {
            return Err("Project name, root, and world name are required".to_string());
        }

        if Path::new(project_root).exists() && !Path::new(project_root).is_dir() {
            return Err("Project root must be a directory".to_string());
        }

        Ok(PromotedCommand::ProjectCreate {
            project_name: project_name.to_string(),
            project_root: project_root.to_string(),
            world_name: world_name.to_string(),
        })
    }

    pub fn build_open_world_command(&self) -> Result<PromotedCommand, String> {
        let world_path = self.open_world.world_path.trim();
        if world_path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }

        let path = Path::new(world_path);
        if !path.exists() {
            return Err("World path does not exist".to_string());
        }
        if !path.is_dir() {
            return Err("World path must be a directory".to_string());
        }
        if !path.join("world.json").exists() {
            return Err("World package is missing world.json".to_string());
        }

        Ok(PromotedCommand::WorldOpen {
            world_path: world_path.to_string(),
        })
    }
}

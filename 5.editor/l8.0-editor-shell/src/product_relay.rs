use crate::command_spine::CommandSpine;
use crate::editor_product_model::EditorProduct;
use stratumx_tooling_l6_0_tool_session::ToolingError;
use stratumx_tooling_l6_1_command_envelopes::{CommandLifecycleState, PromotedCommand};

pub struct ProductRelay {
    product: EditorProduct,
    spine: CommandSpine,
}

impl Default for ProductRelay {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductRelay {
    pub fn new() -> Self {
        Self {
            product: EditorProduct::default(),
            spine: CommandSpine::new(),
        }
    }

    pub fn create_project(&mut self, project_name: impl Into<String>) -> Result<u64, ToolingError> {
        let command = PromotedCommand::ProjectBootstrap {
            project_name: project_name.into(),
        };
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn save_project(&mut self, save_path: impl Into<String>) -> Result<u64, ToolingError> {
        let command = PromotedCommand::ProjectSave {
            save_path: save_path.into(),
        };
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn build_project(
        &mut self,
        target_platform: impl Into<String>,
    ) -> Result<u64, ToolingError> {
        let command = PromotedCommand::ProjectBuild {
            target_platform: target_platform.into(),
        };
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn export_project(&mut self, export_path: impl Into<String>) -> Result<u64, ToolingError> {
        let command = PromotedCommand::ProjectExport {
            export_path: export_path.into(),
        };
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn launch_project(&mut self, launch_mode: impl Into<String>) -> Result<u64, ToolingError> {
        let command = PromotedCommand::ProjectLaunch {
            launch_mode: launch_mode.into(),
        };
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn verify_first_result(&mut self) -> Result<u64, ToolingError> {
        let command = PromotedCommand::ProjectVerifyFirstResult;
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn bootstrap_scene(&mut self) -> Result<u64, ToolingError> {
        let command = PromotedCommand::SceneBootstrap;
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn fire_test_shot(&mut self, weapon_entity_id: u32) -> Result<u64, ToolingError> {
        let command = PromotedCommand::SceneFireTestShot { weapon_entity_id };
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn reset_scene(&mut self) -> Result<u64, ToolingError> {
        let command = PromotedCommand::SceneReset;
        self.spine
            .execute_promoted_command(command, &mut self.product)
    }

    pub fn get_command_state(&self, command_id: u64) -> Option<CommandLifecycleState> {
        self.spine.get_command_state(command_id)
    }

    pub fn get_diagnostics(&self) -> Vec<String> {
        self.spine.all_diagnostics()
    }

    pub fn get_diagnostics_for_command(&self, command_id: u64) -> Vec<String> {
        self.spine.get_diagnostics_for_command(command_id)
    }

    pub fn product(&self) -> &EditorProduct {
        &self.product
    }

    pub fn product_mut(&mut self) -> &mut EditorProduct {
        &mut self.product
    }

    pub fn spine(&self) -> &CommandSpine {
        &self.spine
    }

    pub fn spine_mut(&mut self) -> &mut CommandSpine {
        &mut self.spine
    }

    /// Initialize vertical slice session for scene commands
    /// Scene commands (bootstrap, fire, reset) require this to be called first
    pub fn initialize_vertical_slice_session(&mut self) -> Result<(), ToolingError> {
        Err(ToolingError::Message(
            "Vertical slice session initialization must be wired through CommandExecutor".into(),
        ))
    }
}

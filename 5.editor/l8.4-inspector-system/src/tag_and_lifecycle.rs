use crate::*;

impl EditorProduct {
    pub fn add_selected_tag(&mut self, tag: impl Into<String>) -> Result<(), ToolingError> {
        let handle = self.inspector.selected.ok_or(ToolingError::UnknownObject)?;
        self.tooling.apply_command(
            ToolCommand::AddTag {
                handle,
                tag: tag.into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        self.refresh_from_tooling()
    }
    pub fn remove_selected_tag(&mut self, tag: impl Into<String>) -> Result<(), ToolingError> {
        let handle = self.inspector.selected.ok_or(ToolingError::UnknownObject)?;
        self.tooling.apply_command(
            ToolCommand::RemoveTag {
                handle,
                tag: tag.into(),
            },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        self.refresh_from_tooling()
    }
    pub fn retire_selected(&mut self) -> Result<(), ToolingError> {
        let handle = self.inspector.selected.ok_or(ToolingError::UnknownObject)?;
        self.tooling.apply_command(
            ToolCommand::RetireObject { handle },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        self.refresh_from_tooling()
    }
    pub fn restore_selected(&mut self) -> Result<(), ToolingError> {
        let handle = self.inspector.selected.ok_or(ToolingError::UnknownObject)?;
        self.tooling.apply_command(
            ToolCommand::RestoreObject { handle },
            CommandOrigin::User,
            stratumx_tooling::ApprovalClass::None,
            stratumx_tooling::BudgetClass::Interactive,
        )?;
        self.refresh_from_tooling()
    }
}
